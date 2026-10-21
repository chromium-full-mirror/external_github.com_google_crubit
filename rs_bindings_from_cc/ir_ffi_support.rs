// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! `#[repr(C)]` FFI vocabulary types (`FfiStr`, `FfiVec<T>`, `FfiOption<T>`, `FfiRc<T>`)
//! shared in-memory between C++ (`ir_ffi_support.h`) and Rust.

use ffi_types::{AllocFfiBytes, FreeFfiBytes};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr::{self, NonNull};
use std::slice;
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};

/// An owned UTF-8 string with a `#[repr(C)]` layout matching `crubit::ffi::FfiStr` in C++.
#[repr(C)]
pub struct FfiStr {
    ptr: *const u8,
    len: usize,
}

impl FfiStr {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        if s.is_empty() {
            return Self::default();
        }
        unsafe {
            let buf = AllocFfiBytes(s.len(), 1);
            ptr::copy_nonoverlapping(s.as_ptr(), buf, s.len());
            Self { ptr: buf, len: s.len() }
        }
    }

    pub fn as_str(&self) -> &str {
        if self.len == 0 {
            return "";
        }
        unsafe { std::str::from_utf8_unchecked(slice::from_raw_parts(self.ptr, self.len)) }
    }
}

impl Default for FfiStr {
    fn default() -> Self {
        Self { ptr: NonNull::<u8>::dangling().as_ptr(), len: 0 }
    }
}

impl Drop for FfiStr {
    fn drop(&mut self) {
        if self.len > 0 {
            unsafe {
                FreeFfiBytes(self.ptr as *mut u8, self.len, 1);
            }
        }
    }
}

impl Clone for FfiStr {
    fn clone(&self) -> Self {
        Self::from_str(self.as_str())
    }
}

impl Deref for FfiStr {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for FfiStr {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for FfiStr {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<String> for FfiStr {
    fn from(s: String) -> Self {
        Self::from_str(&s)
    }
}

impl fmt::Debug for FfiStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for FfiStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl PartialEq for FfiStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for FfiStr {}

impl PartialEq<str> for FfiStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for FfiStr {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialOrd for FfiStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FfiStr {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for FfiStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

/// An owned contiguous vector with a `#[repr(C)]` layout matching `crubit::ffi::FfiVec<T>` in C++.
#[repr(C)]
pub struct FfiVec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

impl<T> FfiVec<T> {
    pub fn new() -> Self {
        Self { ptr: NonNull::<T>::dangling().as_ptr(), len: 0, cap: 0 }
    }

    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        if slice.is_empty() {
            return Self::new();
        }
        let cap = slice.len();
        unsafe {
            let ptr = AllocFfiBytes(size_of_val(slice), align_of::<T>()) as *mut T;
            for (i, item) in slice.iter().enumerate() {
                ptr.add(i).write(item.clone());
            }
            Self { ptr, len: cap, cap }
        }
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        if v.is_empty() {
            return Self::new();
        }
        let cap = v.len();
        unsafe {
            let ptr = AllocFfiBytes(cap * size_of::<T>(), align_of::<T>()) as *mut T;
            for (i, item) in v.into_iter().enumerate() {
                ptr.add(i).write(item);
            }
            Self { ptr, len: cap, cap }
        }
    }

    pub fn as_slice(&self) -> &[T] {
        if self.len == 0 {
            return &[];
        }
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> Default for FfiVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for FfiVec<T> {
    fn drop(&mut self) {
        unsafe {
            if self.len > 0 {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr, self.len));
            }
            if self.cap > 0 {
                FreeFfiBytes(self.ptr as *mut u8, self.cap * size_of::<T>(), align_of::<T>());
            }
        }
    }
}

impl<T: Clone> Clone for FfiVec<T> {
    fn clone(&self) -> Self {
        Self::from_slice(self.as_slice())
    }
}

impl<T> Deref for FfiVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> From<Vec<T>> for FfiVec<T> {
    fn from(v: Vec<T>) -> Self {
        Self::from_vec(v)
    }
}

impl<T> FromIterator<T> for FfiVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::from_vec(iter.into_iter().collect())
    }
}

impl<'a, T> IntoIterator for &'a FfiVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

impl<T: fmt::Debug> fmt::Debug for FfiVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_slice(), f)
    }
}

impl<T: PartialEq> PartialEq for FfiVec<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq> Eq for FfiVec<T> {}

impl<T: Hash> Hash for FfiVec<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

/// An optional value with a `#[repr(C, u8)]` layout matching `crubit::ffi::FfiOption<T>` in C++.
#[repr(C, u8)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FfiOption<T> {
    #[default]
    None = 0,
    Some(T) = 1,
}

impl<T> FfiOption<T> {
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Self::None => None,
            Self::Some(v) => Some(v),
        }
    }

    pub fn as_deref(&self) -> Option<&T::Target>
    where
        T: Deref,
    {
        match self {
            Self::None => None,
            Self::Some(v) => Some(v.deref()),
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Some(v) => Some(v),
        }
    }
}

impl<T> From<Option<T>> for FfiOption<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => Self::None,
            Some(v) => Self::Some(v),
        }
    }
}

impl<T> From<FfiOption<T>> for Option<T> {
    fn from(opt: FfiOption<T>) -> Self {
        opt.into_option()
    }
}

impl<T: fmt::Debug> fmt::Debug for FfiOption<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_ref(), f)
    }
}

#[repr(C)]
struct FfiRcInner<T> {
    strong_count: AtomicUsize,
    value: T,
}

/// A reference-counted pointer with a `#[repr(C)]` control block matching `crubit::ffi::FfiRc<T>`
/// in C++.
#[repr(C)]
pub struct FfiRc<T> {
    ptr: NonNull<FfiRcInner<T>>,
}

impl<T> FfiRc<T> {
    pub fn new(value: T) -> Self {
        unsafe {
            let raw = AllocFfiBytes(size_of::<FfiRcInner<T>>(), align_of::<FfiRcInner<T>>())
                as *mut FfiRcInner<T>;
            raw.write(FfiRcInner { strong_count: AtomicUsize::new(1), value });
            Self { ptr: NonNull::new_unchecked(raw) }
        }
    }
}

impl<T> Clone for FfiRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            self.ptr.as_ref().strong_count.fetch_add(1, AtomicOrdering::Relaxed);
        }
        Self { ptr: self.ptr }
    }
}

impl<T> Drop for FfiRc<T> {
    fn drop(&mut self) {
        unsafe {
            let inner = self.ptr.as_ptr();
            if (*inner).strong_count.fetch_sub(1, AtomicOrdering::AcqRel) == 1 {
                ptr::drop_in_place(inner);
                FreeFfiBytes(
                    inner as *mut u8,
                    size_of::<FfiRcInner<T>>(),
                    align_of::<FfiRcInner<T>>(),
                );
            }
        }
    }
}

impl<T> Deref for FfiRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &self.ptr.as_ref().value }
    }
}

impl<T> AsRef<T> for FfiRc<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T: fmt::Debug> fmt::Debug for FfiRc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.deref(), f)
    }
}

impl<T: PartialEq> PartialEq for FfiRc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr || self.deref() == other.deref()
    }
}

impl<T: Eq> Eq for FfiRc<T> {}

impl<T: Hash> Hash for FfiRc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.deref().hash(state);
    }
}

const _: () = {
    assert!(size_of::<FfiStr>() == 2 * size_of::<usize>());
    assert!(align_of::<FfiStr>() == align_of::<usize>());
    assert!(size_of::<FfiVec<u8>>() == 3 * size_of::<usize>());
    assert!(align_of::<FfiVec<u8>>() == align_of::<usize>());
    assert!(size_of::<FfiRc<u8>>() == size_of::<usize>());
    assert!(align_of::<FfiRc<u8>>() == align_of::<usize>());
};
