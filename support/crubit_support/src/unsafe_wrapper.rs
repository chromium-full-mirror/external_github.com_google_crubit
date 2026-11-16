// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use core::fmt;
use core::marker::PhantomData;
use core::mem::MaybeUninit;

/// A wrapper around a value whose lifetime or validity invariants could not be verified across
/// the C++/Rust boundary.
///
/// When Crubit binds a C++ API that mentions a lifetime-parameterized Rust type without knowing
/// its lifetime, it instantiates the inner type with `'static` and wraps it in `Unsafe<T>`.
/// Because the underlying value may in reality borrow data with a shorter lifetime than `'static`,
/// accessing or dropping the inner `T` requires `unsafe`: the caller must assert knowledge of the
/// actual lifetime and validity invariants.
///
/// # Representation and Drop Semantics
///
/// `Unsafe<T>` is `#[repr(transparent)]` over [`MaybeUninit<T>`], so it has the exact same size,
/// alignment, and C ABI as `T`. Storing the value inside `MaybeUninit<T>` means:
///
/// - Dropping an `Unsafe<T>` does **not** run `T`'s destructor (since running `Drop` on a value
///   with a dangling lifetime could be undefined behavior). To release resources owned by `T`, call
///   [`Unsafe::into_inner`] or [`Unsafe::drop_in_place`].
/// - Merely returning, storing, or moving an `Unsafe<T>` across FFI does not assert Rust's validity
///   invariants for `T` (for example, if `T` contains a reference whose referent is no longer
///   live).
///
/// `Unsafe<T>` is `!Send` and `!Sync` regardless of `T`, implements [`Copy`] iff `T: Copy`, and
/// implements [`Unpin`] iff `T: Unpin`. Its [`fmt::Debug`] implementation does not inspect `T`.
#[repr(transparent)]
#[must_use = "dropping an `Unsafe<T>` does not run `T`'s destructor; unwrap with `into_inner` or destroy with `drop_in_place`"]
pub struct Unsafe<T> {
    value: MaybeUninit<T>,
    // 1-aligned ZST: opts out of `Send` and `Sync` without unstable negative impls while
    // preserving `Unpin` iff `T: Unpin`, `Copy` iff `T: Copy`, and `#[repr(transparent)]` ABI
    // compatibility with `T`.
    _not_send_or_sync: PhantomData<*const ()>,
}

impl<T> Unsafe<T> {
    /// Wraps `value` in an `Unsafe<T>`.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self { value: MaybeUninit::new(value), _not_send_or_sync: PhantomData }
    }

    /// Consumes `self` and returns the wrapped value.
    ///
    /// # Safety
    ///
    /// - `self` must contain an initialized value of type `T` (in particular,
    ///   [`Unsafe::drop_in_place`] must not have been called on `self`).
    /// - The wrapped value must satisfy all of `T`'s validity and safety invariants, including that
    ///   every lifetime parameter of `T` (such as `'static` substituted by Crubit when a C++
    ///   lifetime is unknown) remains valid for all subsequent uses of the returned `T` and through
    ///   its destruction.
    #[inline]
    pub const unsafe fn into_inner(self) -> T {
        // SAFETY: The caller guarantees that `self.value` is initialized and satisfies all
        // validity and lifetime invariants of `T`.
        unsafe { self.value.assume_init() }
    }

    /// Returns a shared reference to the wrapped value.
    ///
    /// # Safety
    ///
    /// - `self` must contain an initialized value of type `T` (in particular,
    ///   [`Unsafe::drop_in_place`] must not have been called on `self`).
    /// - The wrapped value must satisfy all of `T`'s validity and safety invariants, including that
    ///   every lifetime parameter of `T` remains valid for at least the duration of the returned
    ///   reference.
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        // SAFETY: The caller guarantees that `self.value` is initialized and valid as `T` for the
        // duration of the returned shared reference.
        unsafe { self.value.assume_init_ref() }
    }

    /// Returns a mutable reference to the wrapped value.
    ///
    /// # Safety
    ///
    /// - `self` must contain an initialized value of type `T` (in particular,
    ///   [`Unsafe::drop_in_place`] must not have been called on `self`).
    /// - The wrapped value must satisfy all of `T`'s validity and safety invariants, including that
    ///   every lifetime parameter of `T` remains valid for at least the duration of the returned
    ///   mutable reference.
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        // SAFETY: The caller guarantees that `self.value` is initialized and valid as `T` for the
        // duration of the returned mutable reference.
        unsafe { self.value.assume_init_mut() }
    }

    /// Runs `T`'s destructor in place without moving `self`.
    ///
    /// # Safety
    ///
    /// - `self` must contain an initialized value of type `T` (in particular,
    ///   [`Unsafe::drop_in_place`] must not have been called on `self` previously).
    /// - The wrapped value must satisfy all preconditions required to drop `T`, including validity
    ///   of any lifetimes accessed during destruction.
    /// - After calling `drop_in_place`, `self` is left uninitialized and must not be passed to
    ///   [`Unsafe::into_inner`], [`Unsafe::as_ref`], [`Unsafe::as_mut`], or
    ///   [`Unsafe::drop_in_place`] again unless overwritten with a new `Unsafe<T>`.
    #[inline]
    pub unsafe fn drop_in_place(&mut self) {
        // SAFETY: The caller guarantees that `self.value` is initialized, satisfies `T`'s drop
        // preconditions, and will not be accessed or dropped again while uninitialized.
        unsafe { self.value.assume_init_drop() };
    }
}

impl<T: Copy> Copy for Unsafe<T> {}

impl<T: Copy> Clone for Unsafe<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> fmt::Debug for Unsafe<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Unsafe").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use core::cell::Cell;
    use core::marker::PhantomPinned;
    use core::mem::{align_of, size_of};
    use googletest::{expect_eq, gtest};

    struct DropCounter<'a>(&'a Cell<usize>);

    impl Drop for DropCounter<'_> {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    struct ReprCPair {
        a: u32,
        b: u64,
    }

    #[gtest]
    fn test_layout_matches_inner() {
        expect_eq!(size_of::<Unsafe<()>>(), size_of::<()>());
        expect_eq!(align_of::<Unsafe<()>>(), align_of::<()>());
        expect_eq!(size_of::<Unsafe<u8>>(), size_of::<u8>());
        expect_eq!(align_of::<Unsafe<u8>>(), align_of::<u8>());
        expect_eq!(size_of::<Unsafe<ReprCPair>>(), size_of::<ReprCPair>());
        expect_eq!(align_of::<Unsafe<ReprCPair>>(), align_of::<ReprCPair>());
        expect_eq!(size_of::<Unsafe<&'static i32>>(), size_of::<&'static i32>());
        expect_eq!(align_of::<Unsafe<&'static i32>>(), align_of::<&'static i32>());
    }

    #[gtest]
    fn test_accessors_and_into_inner() {
        let mut wrapped = Unsafe::new(10i32);
        // SAFETY: `wrapped` was initialized via `Unsafe::new` and `i32` has no lifetime invariants.
        unsafe {
            expect_eq!(*wrapped.as_ref(), 10);
            *wrapped.as_mut() = 42;
            expect_eq!(*wrapped.as_ref(), 42);
            expect_eq!(wrapped.into_inner(), 42);
        }
    }

    #[gtest]
    fn test_drop_is_suppressed_until_unwrapped_or_dropped_in_place() {
        let drops = Cell::new(0usize);

        {
            let wrapped = Unsafe::new(DropCounter(&drops));
            expect_eq!(drops.get(), 0);
            drop(wrapped);
            // Dropping `Unsafe<T>` must not run `T`'s destructor.
            expect_eq!(drops.get(), 0);
        }

        {
            let wrapped = Unsafe::new(DropCounter(&drops));
            // SAFETY: `wrapped` holds an initialized `DropCounter` whose `&drops` reference is
            // still live.
            let inner = unsafe { wrapped.into_inner() };
            drop(inner);
            expect_eq!(drops.get(), 1);
        }

        {
            let mut wrapped = Unsafe::new(DropCounter(&drops));
            // SAFETY: `wrapped` holds an initialized `DropCounter` whose `&drops` reference is
            // still live, and `wrapped` is not accessed after `drop_in_place`.
            unsafe { wrapped.drop_in_place() };
            expect_eq!(drops.get(), 2);
        }
    }

    #[gtest]
    fn test_copy_and_clone() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<Unsafe<ReprCPair>>();

        let a = Unsafe::new(ReprCPair { a: 1, b: 2 });
        let b = a;
        let c = a.clone();
        // SAFETY: `a`, `b`, and `c` were initialized via `Unsafe::new` with a `Copy` value.
        unsafe {
            expect_eq!(a.into_inner(), ReprCPair { a: 1, b: 2 });
            expect_eq!(b.into_inner(), ReprCPair { a: 1, b: 2 });
            expect_eq!(c.into_inner(), ReprCPair { a: 1, b: 2 });
        }
    }

    #[gtest]
    fn test_debug_does_not_inspect_inner() {
        struct PanickingDebug;
        impl fmt::Debug for PanickingDebug {
            fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
                panic!("Unsafe::fmt must not delegate to T::fmt");
            }
        }

        let wrapped = Unsafe::new(PanickingDebug);
        expect_eq!(format!("{wrapped:?}"), "Unsafe(..)");
        // SAFETY: `wrapped` holds a valid ZST `PanickingDebug`.
        let _ = unsafe { wrapped.into_inner() };
    }

    #[gtest]
    fn test_unpin_and_ffi_safety() {
        fn assert_unpin<T: Unpin>() {}
        assert_unpin::<Unsafe<i32>>();
        assert_unpin::<Unsafe<ReprCPair>>();

        // Compile-time check that `Unsafe<T>` passes `improper_ctypes` for FFI-safe `T`.
        #[deny(improper_ctypes)]
        unsafe extern "C" {
            #[allow(dead_code)]
            fn _crubit_unsafe_ffi_check(x: Unsafe<ReprCPair>) -> Unsafe<ReprCPair>;
        }

        // Verify `Unsafe<PhantomPinned>` is constructible and preserves layout.
        let pinned = Unsafe::new(PhantomPinned);
        expect_eq!(size_of::<Unsafe<PhantomPinned>>(), size_of::<PhantomPinned>());
        // SAFETY: `pinned` holds a valid `PhantomPinned`.
        let _ = unsafe { pinned.into_inner() };
    }
}
