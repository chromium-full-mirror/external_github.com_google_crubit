// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/semantic_import:semantic_import

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "1S"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=S
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct S {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) x_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for S {}
impl !Sync for S {}
unsafe impl ::cxx::ExternType for S {
    type Id = ::cxx::type_id!("S");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("S"), crate::S);
impl S {
    #[inline(always)]
    pub fn x<'__this>(&'__this self) -> ::ffi_11::c_int {
        unsafe { self::s::x(self) }
    }
    #[inline(always)]
    pub fn get_x<'__this>(&'__this mut self) -> ::ffi_11::c_int {
        unsafe { self::s::get_x(self) }
    }
    #[inline(always)]
    pub fn set_x<'__this>(&'__this mut self, x: ::ffi_11::c_int) {
        unsafe { self::s::set_x(self, x) }
    }
}

impl From<::ffi_11::c_int> for S {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1SC1Ei(&raw mut tmp as *mut _, x);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for S {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

pub mod s {
    #[inline(always)]
    pub(crate) fn x<'__this>(__this: &'__this crate::S) -> ::ffi_11::c_int {
        unsafe {
            ::core::ptr::read(((&raw const *__this) as *const u8).add(0) as *const ::ffi_11::c_int)
        }
    }
    #[inline(always)]
    pub(crate) fn get_x<'__this>(__this: &'__this mut crate::S) -> ::ffi_11::c_int {
        unsafe {
            ::core::ptr::read(((&raw const *__this) as *const u8).add(0) as *const ::ffi_11::c_int)
        }
    }
    #[inline(always)]
    pub(crate) fn set_x<'__this>(__this: &'__this mut crate::S, x: ::ffi_11::c_int) {
        unsafe {
            ::core::ptr::write((__this as *mut _ as *mut u8).add(0) as *mut ::ffi_11::c_int, x)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "1T"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=T
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct T {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) y_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for T {}
impl !Sync for T {}
unsafe impl ::cxx::ExternType for T {
    type Id = ::cxx::type_id!("T");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("T"), crate::T);
impl T {
    #[inline(always)]
    pub fn y<'__this>(&'__this self) -> f32 {
        unsafe { self::t::y(self) }
    }
}

impl From<(::ffi_11::c_int, f32)> for T {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, f32)) -> Self {
        let (mut x, mut y) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1TC1Eif(&raw mut tmp as *mut _, x, y);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, f32)> for T {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, f32)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, f32)>>::from(args)
    }
}

pub mod t {
    #[inline(always)]
    pub(crate) fn y<'__this>(__this: &'__this crate::T) -> f32 {
        unsafe { ::core::ptr::read(((&raw const *__this) as *const u8).add(4) as *const f32) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Chars"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Chars
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Chars {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) c_: [::core::mem::MaybeUninit<u8>; 1],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) sc_: [::core::mem::MaybeUninit<u8>; 1],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) uc_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Chars {}
impl !Sync for Chars {}
unsafe impl ::cxx::ExternType for Chars {
    type Id = ::cxx::type_id!("Chars");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Chars"), crate::Chars);
impl Chars {
    #[inline(always)]
    pub fn c<'__this>(&'__this mut self) -> ::ffi_11::c_char {
        unsafe { self::chars::c(self) }
    }
    #[inline(always)]
    pub fn sc<'__this>(&'__this mut self) -> ::ffi_11::c_schar {
        unsafe { self::chars::sc(self) }
    }
    #[inline(always)]
    pub fn uc<'__this>(&'__this mut self) -> ::ffi_11::c_uchar {
        unsafe { self::chars::uc(self) }
    }
    #[inline(always)]
    pub fn set_c<'__this>(&'__this mut self, c: ::ffi_11::c_char) {
        unsafe { self::chars::set_c(self, c) }
    }
    #[inline(always)]
    pub fn set_sc<'__this>(&'__this mut self, sc: ::ffi_11::c_schar) {
        unsafe { self::chars::set_sc(self, sc) }
    }
    #[inline(always)]
    pub fn set_uc<'__this>(&'__this mut self, uc: ::ffi_11::c_uchar) {
        unsafe { self::chars::set_uc(self, uc) }
    }
}

impl Default for Chars {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5CharsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod chars {
    #[inline(always)]
    pub(crate) fn c<'__this>(__this: &'__this mut crate::Chars) -> ::ffi_11::c_char {
        unsafe {
            ::core::ptr::read(((&raw const *__this) as *const u8).add(0) as *const ::ffi_11::c_char)
        }
    }
    #[inline(always)]
    pub(crate) fn sc<'__this>(__this: &'__this mut crate::Chars) -> ::ffi_11::c_schar {
        unsafe {
            ::core::ptr::read(((&raw const *__this) as *const u8).add(1) as *const ::ffi_11::c_schar)
        }
    }
    #[inline(always)]
    pub(crate) fn uc<'__this>(__this: &'__this mut crate::Chars) -> ::ffi_11::c_uchar {
        unsafe {
            ::core::ptr::read(((&raw const *__this) as *const u8).add(2) as *const ::ffi_11::c_uchar)
        }
    }
    #[inline(always)]
    pub(crate) fn set_c<'__this>(__this: &'__this mut crate::Chars, c: ::ffi_11::c_char) {
        unsafe {
            ::core::ptr::write((__this as *mut _ as *mut u8).add(0) as *mut ::ffi_11::c_char, c)
        }
    }
    #[inline(always)]
    pub(crate) fn set_sc<'__this>(__this: &'__this mut crate::Chars, sc: ::ffi_11::c_schar) {
        unsafe {
            ::core::ptr::write((__this as *mut _ as *mut u8).add(1) as *mut ::ffi_11::c_schar, sc)
        }
    }
    #[inline(always)]
    pub(crate) fn set_uc<'__this>(__this: &'__this mut crate::Chars, uc: ::ffi_11::c_uchar) {
        unsafe {
            ::core::ptr::write((__this as *mut _ as *mut u8).add(2) as *mut ::ffi_11::c_uchar, uc)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "5Bools"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Bools
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Bools {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b_: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Bools {}
impl !Sync for Bools {}
unsafe impl ::cxx::ExternType for Bools {
    type Id = ::cxx::type_id!("Bools");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Bools"), crate::Bools);
impl Bools {
    #[inline(always)]
    pub fn b<'__this>(&'__this mut self) -> bool {
        unsafe { self::bools::b(self) }
    }
    #[inline(always)]
    pub fn set_b<'__this>(&'__this mut self, b: bool) {
        unsafe { self::bools::set_b(self, b) }
    }
}

impl Default for Bools {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5BoolsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod bools {
    #[inline(always)]
    pub(crate) fn b<'__this>(__this: &'__this mut crate::Bools) -> bool {
        unsafe { ::core::ptr::read(((&raw const *__this) as *const u8).add(0) as *const bool) }
    }
    #[inline(always)]
    pub(crate) fn set_b<'__this>(__this: &'__this mut crate::Bools, b: bool) {
        unsafe { ::core::ptr::write((__this as *mut _ as *mut u8).add(0) as *mut bool, b) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "8Pointers"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Pointers
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Pointers {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) p_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) mut_p_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for Pointers {}
impl !Sync for Pointers {}
unsafe impl ::cxx::ExternType for Pointers {
    type Id = ::cxx::type_id!("Pointers");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Pointers"), crate::Pointers);
impl Pointers {
    #[inline(always)]
    pub fn p<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::pointers::p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_p<'__this>(&'__this mut self, p: *const ::ffi_11::c_int) {
        unsafe { self::pointers::set_p(self, p) }
    }
    #[inline(always)]
    pub fn mut_p<'__this>(&'__this self) -> *mut ::ffi_11::c_int {
        unsafe { self::pointers::mut_p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `mut_p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_mut_p<'__this>(&'__this mut self, mut_p: *mut ::ffi_11::c_int) {
        unsafe { self::pointers::set_mut_p(self, mut_p) }
    }
}

impl Default for Pointers {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8PointersC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod pointers {
    #[inline(always)]
    pub(crate) fn p<'__this>(__this: &'__this crate::Pointers) -> *const ::ffi_11::c_int {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(0) as *const *const ::ffi_11::c_int
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_p<'__this>(
        __this: &'__this mut crate::Pointers,
        p: *const ::ffi_11::c_int,
    ) {
        unsafe {
            ::core::ptr::write(
                (__this as *mut _ as *mut u8).add(0) as *mut *const ::ffi_11::c_int,
                p,
            )
        }
    }
    #[inline(always)]
    pub(crate) fn mut_p<'__this>(__this: &'__this crate::Pointers) -> *mut ::ffi_11::c_int {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(8) as *const *mut ::ffi_11::c_int
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `mut_p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_mut_p<'__this>(
        __this: &'__this mut crate::Pointers,
        mut_p: *mut ::ffi_11::c_int,
    ) {
        unsafe {
            ::core::ptr::write(
                (__this as *mut _ as *mut u8).add(8) as *mut *mut ::ffi_11::c_int,
                mut_p,
            )
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "10NonTrivial"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=NonTrivial
pub struct NonTrivial {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) p_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) mut_p_: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for NonTrivial {}
impl !Sync for NonTrivial {}
unsafe impl ::cxx::ExternType for NonTrivial {
    type Id = ::cxx::type_id!("NonTrivial");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("NonTrivial"), crate::NonTrivial);
impl NonTrivial {
    #[inline(always)]
    pub fn p<'__this>(&'__this self) -> *const crate::NonTrivial {
        unsafe { self::non_trivial::p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_p<'__this>(self: ::core::pin::Pin<&'__this mut Self>, p: *const Self) {
        unsafe { self::non_trivial::set_p(self, p) }
    }
    #[inline(always)]
    pub fn mut_p<'__this>(self: ::core::pin::Pin<&'__this mut Self>) -> *mut crate::NonTrivial {
        unsafe { self::non_trivial::mut_p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `mut_p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_mut_p<'__this>(self: ::core::pin::Pin<&'__this mut Self>, mut_p: *mut Self) {
        unsafe { self::non_trivial::set_mut_p(self, mut_p) }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for NonTrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let mut __param_0 = args;
                crate::detail::__rust_thunk___ZN10NonTrivialC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for NonTrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for NonTrivial {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NonTrivialaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::CtorNew<()> for NonTrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                let () = args;
                crate::detail::__rust_thunk___ZN10NonTrivialC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

/// A user-provided destructor is what makes this type non-trivial (and so
/// `!Unpin` in Rust).
/// `= default` would defeat the purpose.
impl ::ctor::PinnedDrop for NonTrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10NonTrivialD1Ev(self) }
    }
}

pub mod non_trivial {
    #[inline(always)]
    pub(crate) fn p<'__this>(__this: &'__this crate::NonTrivial) -> *const crate::NonTrivial {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(0) as *const *const crate::NonTrivial
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_p<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
        p: *const crate::NonTrivial,
    ) {
        unsafe {
            ::core::ptr::write(
                (::core::pin::Pin::into_inner_unchecked(__this) as *mut _ as *mut u8).add(0)
                    as *mut *const crate::NonTrivial,
                p,
            )
        }
    }
    #[inline(always)]
    pub(crate) fn mut_p<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
    ) -> *mut crate::NonTrivial {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(8) as *const *mut crate::NonTrivial
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `mut_p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_mut_p<'__this>(
        __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
        mut_p: *mut crate::NonTrivial,
    ) {
        unsafe {
            ::core::ptr::write(
                (::core::pin::Pin::into_inner_unchecked(__this) as *mut _ as *mut u8).add(8)
                    as *mut *mut crate::NonTrivial,
                mut_p,
            )
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "7Padding"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Padding
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Padding {
    pub pad_: ::ffi_11::c_long,
}
impl !Send for Padding {}
impl !Sync for Padding {}
unsafe impl ::cxx::ExternType for Padding {
    type Id = ::cxx::type_id!("Padding");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Padding"), crate::Padding);

impl Default for Padding {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7PaddingC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "11PointerBase"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PointerBase
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct PointerBase {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) base_p_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for PointerBase {}
impl !Sync for PointerBase {}
unsafe impl ::cxx::ExternType for PointerBase {
    type Id = ::cxx::type_id!("PointerBase");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("PointerBase"), crate::PointerBase);
impl PointerBase {
    #[inline(always)]
    pub fn base_p<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::pointer_base::base_p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `base_p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_base_p<'__this>(&'__this mut self, base_p: *const ::ffi_11::c_int) {
        unsafe { self::pointer_base::set_base_p(self, base_p) }
    }
}

impl Default for PointerBase {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11PointerBaseC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod pointer_base {
    #[inline(always)]
    pub(crate) fn base_p<'__this>(__this: &'__this crate::PointerBase) -> *const ::ffi_11::c_int {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(0) as *const *const ::ffi_11::c_int
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `base_p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_base_p<'__this>(
        __this: &'__this mut crate::PointerBase,
        base_p: *const ::ffi_11::c_int,
    ) {
        unsafe {
            ::core::ptr::write(
                (__this as *mut _ as *mut u8).add(0) as *mut *const ::ffi_11::c_int,
                base_p,
            )
        }
    }
}

/// `PointerBase` is at a non-zero offset within `DerivedPointers`, so the
/// accessors declared here must not reuse `PointerBase`'s offset for `base_p_`.
/// They therefore keep using a thunk, unlike `PointerBase`'s own accessors.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "15DerivedPointers"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=DerivedPointers
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct DerivedPointers {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 16],
}
impl !Send for DerivedPointers {}
impl !Sync for DerivedPointers {}
unsafe impl ::cxx::ExternType for DerivedPointers {
    type Id = ::cxx::type_id!("DerivedPointers");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("DerivedPointers"),
    crate::DerivedPointers
);
impl DerivedPointers {
    #[inline(always)]
    pub fn p<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::derived_pointers::p(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub unsafe fn set_p<'__this>(&'__this mut self, p: *const ::ffi_11::c_int) {
        unsafe { self::derived_pointers::set_p(self, p) }
    }
}

impl Default for DerivedPointers {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15DerivedPointersC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod derived_pointers {
    #[inline(always)]
    pub(crate) fn p<'__this>(__this: &'__this crate::DerivedPointers) -> *const ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK15DerivedPointers1pEv(__this) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `p`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_p<'__this>(
        __this: &'__this mut crate::DerivedPointers,
        p: *const ::ffi_11::c_int,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN15DerivedPointers5set_pEPKi(__this, p) }
    }
}

forward_declare::forward_declare!(pub Incomplete = forward_declare::symbol!("Incomplete"));

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "12MorePointers"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=MorePointers
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct MorePointers {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) v_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for MorePointers {}
impl !Sync for MorePointers {}
unsafe impl ::cxx::ExternType for MorePointers {
    type Id = ::cxx::type_id!("MorePointers");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("MorePointers"), crate::MorePointers);
impl MorePointers {
    #[inline(always)]
    pub fn v<'__this>(&'__this self) -> *mut ::ffi_11::c_void {
        unsafe { self::more_pointers::v(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `v`: raw pointer
    #[inline(always)]
    pub unsafe fn set_v<'__this>(&'__this mut self, v: *mut ::ffi_11::c_void) {
        unsafe { self::more_pointers::set_v(self, v) }
    }
    #[inline(always)]
    pub fn i<'__this>(&'__this self) -> *mut crate::Incomplete {
        unsafe { self::more_pointers::i(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `i`: raw pointer
    #[inline(always)]
    pub unsafe fn set_i<'__this>(&'__this mut self, i: *mut crate::Incomplete) {
        unsafe { self::more_pointers::set_i(self, i) }
    }
}

impl Default for MorePointers {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12MorePointersC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod more_pointers {
    #[inline(always)]
    pub(crate) fn v<'__this>(__this: &'__this crate::MorePointers) -> *mut ::ffi_11::c_void {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(0) as *const *mut ::ffi_11::c_void
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `v`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_v<'__this>(
        __this: &'__this mut crate::MorePointers,
        v: *mut ::ffi_11::c_void,
    ) {
        unsafe {
            ::core::ptr::write(
                (__this as *mut _ as *mut u8).add(0) as *mut *mut ::ffi_11::c_void,
                v,
            )
        }
    }
    #[inline(always)]
    pub(crate) fn i<'__this>(__this: &'__this crate::MorePointers) -> *mut crate::Incomplete {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(8) as *const *mut crate::Incomplete
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `i`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_i<'__this>(
        __this: &'__this mut crate::MorePointers,
        i: *mut crate::Incomplete,
    ) {
        unsafe {
            ::core::ptr::write(
                (__this as *mut _ as *mut u8).add(8) as *mut *mut crate::Incomplete,
                i,
            )
        }
    }
}

/// `rs_std::SliceRef` maps to a Rust slice pointer. Crubit relies on the two
/// having the same layout, so the getter reads the field directly, but the
/// setter keeps using a thunk: only primitives and plain pointers get a
/// thunkless setter.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "6Slices"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=Slices
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct Slices {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) s_: [::core::mem::MaybeUninit<u8>; 16],
}
impl !Send for Slices {}
impl !Sync for Slices {}
unsafe impl ::cxx::ExternType for Slices {
    type Id = ::cxx::type_id!("Slices");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Slices"), crate::Slices);
impl Slices {
    #[inline(always)]
    pub fn s<'__this>(&'__this self) -> *const [::ffi_11::c_int] {
        unsafe { self::slices::s(self) }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `s`: raw pointer
    #[inline(always)]
    pub unsafe fn set_s<'__this>(&'__this mut self, s: *const [::ffi_11::c_int]) {
        unsafe { self::slices::set_s(self, s) }
    }
}

impl Default for Slices {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN6SlicesC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

pub mod slices {
    #[inline(always)]
    pub(crate) fn s<'__this>(__this: &'__this crate::Slices) -> *const [::ffi_11::c_int] {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(0) as *const *const [::ffi_11::c_int]
            )
        }
    }
    /// # Safety
    ///
    /// The caller must ensure that the following unsafe arguments are not misused by the function:
    /// * `s`: raw pointer
    #[inline(always)]
    pub(crate) unsafe fn set_s<'__this>(
        __this: &'__this mut crate::Slices,
        s: *const [::ffi_11::c_int],
    ) {
        unsafe { crate::detail::__rust_thunk___ZN6Slices5set_sEN2rs8SliceRefIKiEE(__this, s) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "29StructWithRecordAndFreeGetter"]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=StructWithRecordAndFreeGetter
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct StructWithRecordAndFreeGetter {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) s_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for StructWithRecordAndFreeGetter {}
impl !Sync for StructWithRecordAndFreeGetter {}
unsafe impl ::cxx::ExternType for StructWithRecordAndFreeGetter {
    type Id = ::cxx::type_id!("StructWithRecordAndFreeGetter");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructWithRecordAndFreeGetter"),
    crate::StructWithRecordAndFreeGetter
);
impl StructWithRecordAndFreeGetter {
    #[inline(always)]
    pub fn s<'__this>(&'__this self) -> crate::S {
        unsafe { self::struct_with_record_and_free_getter::s(self) }
    }
}

impl From<::ffi_11::c_int> for StructWithRecordAndFreeGetter {
    #[inline(always)]
    fn from(args: ::ffi_11::c_int) -> Self {
        let mut x = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN29StructWithRecordAndFreeGetterC1Ei(
                &raw mut tmp as *mut _,
                x,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ffi_11::c_int> for StructWithRecordAndFreeGetter {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        <Self as From<::ffi_11::c_int>>::from(args)
    }
}

#[inline(always)]
pub fn free_get_s<'__param_0>(
    __param_0: &'__param_0 crate::StructWithRecordAndFreeGetter,
) -> crate::S {
    unsafe { ::core::ptr::read(((&raw const *__param_0) as *const u8).add(0) as *const crate::S) }
}

pub mod struct_with_record_and_free_getter {
    #[inline(always)]
    pub(crate) fn s<'__this>(__this: &'__this crate::StructWithRecordAndFreeGetter) -> crate::S {
        unsafe { ::core::ptr::read(((&raw const *__this) as *const u8).add(0) as *const crate::S) }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < class std :: __wrap_iter < const int *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: reverse_iterator < class std :: __wrap_iter < const int *>>"),
    crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
);
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_ki_eeee::base(self)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__116c07cb__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE) -> Self {
        let mut __x = args;
        let mut __x = ::core::mem::MaybeUninit::new(__x);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__9046235f__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(&raw mut tmp as*mut _,__x.as_mut_ptr());
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_ip_ki_eeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            ::core::ptr::read(((&raw const *__this) as *const u8).add(0)
                as *const crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: reverse_iterator < class std :: __wrap_iter < int *>>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) current: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {}
impl !Sync for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: reverse_iterator < class std :: __wrap_iter < int *>>"),
    crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE
);
impl __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    #[must_use]
    #[inline(always)]
    pub fn base<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            self::cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_i_pi_eeee::base(self)
        }
    }
}

impl Default for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__116c07cb__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(&raw mut tmp as*mut _);
            tmp.assume_init()
        }
    }
}

impl From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE
{
    #[inline(always)]
    fn from(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE) -> Self {
        let mut __x = args;
        let mut __x = ::core::mem::MaybeUninit::new(__x);
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__9046235f__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(&raw mut tmp as*mut _,__x.as_mut_ptr());
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>
    for __CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE
{
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE) -> Self::CtorType {
        <Self as From<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>>::from(args)
    }
}

pub mod cc_template_inst_n_st3_u16reverse_iterator_ins_11_wrap_iter_i_pi_eeee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn base<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            ::core::ptr::read(((&raw const *__this) as *const u8).add(0)
                as *const crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE)
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPKiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < const int *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: __wrap_iter < const int *>"),
    crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
);

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__1eeb8fed__ZNSt3__u11__wrap_iterIPKiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Add<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__ee336382__ZNKSt3__u11__wrap_iterIPKiEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__aec51f55__ZNSt3__u11__wrap_iterIPKiEpLEl(self, __n);
        }
    }
}

impl<'__this> ::core::ops::Sub<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__a5af0030__ZNKSt3__u11__wrap_iterIPKiEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    #[inline(always)]
    fn sub_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__f35fefa5__ZNSt3__u11__wrap_iterIPKiEmIEl(self, __n);
        }
    }
}

impl ::operator::CcIndex<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    type Output<'ctnr> = &'ctnr ::ffi_11::c_int;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __n: isize) -> Self::Output<'ctnr> {
        unsafe { crate::detail::__rust_thunk__0248de23__ZNKSt3__u11__wrap_iterIPKiEixEl(self, __n) }
    }
}
impl ::core::ops::Index<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
    type Output = ::ffi_11::c_int;
    #[inline(always)]
    fn index(&self, index: isize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u11__wrap_iterIPiEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: __wrap_iter < int *>
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __i_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {}
impl !Sync for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: __wrap_iter < int *>"),
    crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE
);

impl Default for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__1eeb8fed__ZNSt3__u11__wrap_iterIPiEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::ops::Add<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
    #[inline(always)]
    fn add(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__ee336382__ZNKSt3__u11__wrap_iterIPiEplEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::AddAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    #[inline(always)]
    fn add_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__aec51f55__ZNSt3__u11__wrap_iterIPiEpLEl(self, __n);
        }
    }
}

impl<'__this> ::core::ops::Sub<isize>
    for &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE
{
    type Output = crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
    #[inline(always)]
    fn sub(self, __n: isize) -> Self::Output {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__a5af0030__ZNKSt3__u11__wrap_iterIPiEmiEl(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                self,
                __n,
            );
            __crubit_return.assume_init()
        }
    }
}

impl ::core::ops::SubAssign<isize> for __CcTemplateInstNSt3__u11__wrap_iterIPiEE {
    #[inline(always)]
    fn sub_assign<'__this>(&'__this mut self, __n: isize) {
        unsafe {
            crate::detail::__rust_thunk__f35fefa5__ZNSt3__u11__wrap_iterIPiEmIEl(self, __n);
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: span < const int , 18446744073709551615UL >
pub struct __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {}
impl !Sync for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: span < const int , 18446744073709551615UL >"),
    crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE
);
impl __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    #[must_use]
    #[inline(always)]
    pub fn first<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::first(
                self, __count,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn last<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::last(self, __count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn subspan<'__this>(
        &'__this self,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::subspan(
                self, __offset, __count,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::size(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn size_bytes<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::size_bytes(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::empty(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::front(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::back(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *const ::ffi_11::c_int {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::data(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::begin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::end(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::rbegin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee::rend(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__041ce97c__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

impl ::operator::CcIndex<usize> for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    type Output<'ctnr> = &'ctnr ::ffi_11::c_int;
    #[inline(always)]
    fn cc_index<'ctnr>(&'ctnr self, __idx: usize) -> Self::Output<'ctnr> {
        unsafe {
            crate::detail::__rust_thunk__efeb2c22__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm(
                self, __idx,
            )
        }
    }
}
impl ::core::ops::Index<usize> for __CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
    type Output = ::ffi_11::c_int;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        ::operator::CcIndex::cc_index(self, index)
    }
}

pub mod cc_template_inst_n_st3_u4span_i_ki_lm18446744073709551615_eee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn first<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__e6710516__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn last<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__01427f91__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
                __count,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn subspan<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__03346ce2__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__offset,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> usize {
        unsafe { ::core::ptr::read(((&raw const *__this) as *const u8).add(8) as *const usize) }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size_bytes<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__5545573d__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__2f1f91b1__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__07a5bf78__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__a3ec4015__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> *const ::ffi_11::c_int {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(0) as *const *const ::ffi_11::c_int
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__9aeb93a7__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            >::uninit();
            crate::detail::__rust_thunk__3fb9ec58__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__84fdcbe6__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__4dfa2303__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=std :: span < int , 18446744073709551615UL >
pub struct __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __data_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) __size_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {}
impl !Sync for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("std :: span < int , 18446744073709551615UL >"),
    crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE
);
impl __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    #[must_use]
    #[inline(always)]
    pub fn first<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::first(self, __count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn last<'__this>(
        &'__this self,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::last(self, __count)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn subspan<'__this>(
        &'__this self,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::subspan(
                self, __offset, __count,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn size<'__this>(&'__this self) -> usize {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::size(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn size_bytes<'__this>(&'__this self) -> usize {
        unsafe {
            self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::size_bytes(self)
        }
    }
    #[must_use]
    #[inline(always)]
    pub fn empty<'__this>(&'__this self) -> bool {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::empty(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn front<'__this>(&'__this self) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::front(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn back<'__this>(&'__this self) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::back(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn data<'__this>(&'__this self) -> *mut ::ffi_11::c_int {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::data(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn begin<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::begin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn end<'__this>(&'__this self) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::end(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rbegin<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::rbegin(self) }
    }
    #[must_use]
    #[inline(always)]
    pub fn rend<'__this>(
        &'__this self,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe { self::cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee::rend(self) }
    }
}

impl Default for __CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__041ce97c__ZNSt3__u4spanIiLm18446744073709551615EEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

pub mod cc_template_inst_n_st3_u4span_ii_lm18446744073709551615_eee {
    #[must_use]
    #[inline(always)]
    pub(crate) fn first<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__e6710516__ZNKSt3__u4spanIiLm18446744073709551615EE5firstEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
                __count,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn last<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__01427f91__ZNKSt3__u4spanIiLm18446744073709551615EE4lastEm(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
                __count,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn subspan<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        __offset: usize,
        __count: usize,
    ) -> crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            >::uninit();
            crate::detail::__rust_thunk__03346ce2__ZNKSt3__u4spanIiLm18446744073709551615EE7subspanEmm(&raw mut __crubit_return as*mut::core::ffi::c_void,__this,__offset,__count);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> usize {
        unsafe { ::core::ptr::read(((&raw const *__this) as *const u8).add(8) as *const usize) }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn size_bytes<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> usize {
        unsafe {
            crate::detail::__rust_thunk__5545573d__ZNKSt3__u4spanIiLm18446744073709551615EE10size_bytesEv(__this)
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn empty<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> bool {
        unsafe {
            crate::detail::__rust_thunk__2f1f91b1__ZNKSt3__u4spanIiLm18446744073709551615EE5emptyEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn front<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__07a5bf78__ZNKSt3__u4spanIiLm18446744073709551615EE5frontEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn back<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe {
            crate::detail::__rust_thunk__a3ec4015__ZNKSt3__u4spanIiLm18446744073709551615EE4backEv(
                __this,
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn data<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> *mut ::ffi_11::c_int {
        unsafe {
            ::core::ptr::read(
                ((&raw const *__this) as *const u8).add(0) as *const *mut ::ffi_11::c_int
            )
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn begin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__9aeb93a7__ZNKSt3__u4spanIiLm18446744073709551615EE5beginEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn end<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            >::uninit();
            crate::detail::__rust_thunk__3fb9ec58__ZNKSt3__u4spanIiLm18446744073709551615EE3endEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rbegin<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__84fdcbe6__ZNKSt3__u4spanIiLm18446744073709551615EE6rbeginEv(&raw mut __crubit_return as*mut::core::ffi::c_void,__this);
            __crubit_return.assume_init()
        }
    }
    #[must_use]
    #[inline(always)]
    pub(crate) fn rend<'__this>(
        __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
    ) -> crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<
                crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
            >::uninit();
            crate::detail::__rust_thunk__4dfa2303__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                __this,
            );
            __crubit_return.assume_init()
        }
    }
}

// Type bindings for rs::SliceRef<const int> suppressed due to being mapped to an existing Rust type (*const[::ffi_11::c_int])

// Type bindings for rs::SliceRef<int> suppressed due to being mapped to an existing Rust type (*mut[::ffi_11::c_int])

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN1SC1Ei(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN1TC1Eif(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
            y: f32,
        );
        pub(crate) unsafe fn __rust_thunk___ZN5CharsC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5BoolsC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN8PointersC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::NonTrivial,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
            __param_0: &'__param_0 crate::NonTrivial,
        ) -> ::core::pin::Pin<&'__this mut crate::NonTrivial>;
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN10NonTrivialD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::NonTrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN7PaddingC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN11PointerBaseC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN15DerivedPointersC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK15DerivedPointers1pEv<'__this>(
            __this: &'__this crate::DerivedPointers,
        ) -> *const ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN15DerivedPointers5set_pEPKi<'__this>(
            __this: &'__this mut crate::DerivedPointers,
            p: *const ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN12MorePointersC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN6SlicesC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN6Slices5set_sEN2rs8SliceRefIKiEE<'__this>(
            __this: &'__this mut crate::Slices,
            s: *const [::ffi_11::c_int],
        );
        pub(crate) unsafe fn __rust_thunk___ZN29StructWithRecordAndFreeGetterC1Ei(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk__116c07cb__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9046235f__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
        );
        pub(crate) unsafe fn __rust_thunk__116c07cb__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__9046235f__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(
            __this: *mut ::core::ffi::c_void,
            __x: *mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
        );
        pub(crate) unsafe fn __rust_thunk__1eeb8fed__ZNSt3__u11__wrap_iterIPKiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ee336382__ZNKSt3__u11__wrap_iterIPKiEplEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__aec51f55__ZNSt3__u11__wrap_iterIPKiEpLEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        pub(crate) unsafe fn __rust_thunk__a5af0030__ZNKSt3__u11__wrap_iterIPKiEmiEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__f35fefa5__ZNSt3__u11__wrap_iterIPKiEmIEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE;
        pub(crate) unsafe fn __rust_thunk__0248de23__ZNKSt3__u11__wrap_iterIPKiEixEl<'__this>(
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE,
            __n: isize,
        ) -> &'__this ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__1eeb8fed__ZNSt3__u11__wrap_iterIPiEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__ee336382__ZNKSt3__u11__wrap_iterIPiEplEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__aec51f55__ZNSt3__u11__wrap_iterIPiEpLEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
        pub(crate) unsafe fn __rust_thunk__a5af0030__ZNKSt3__u11__wrap_iterIPiEmiEl<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        );
        pub(crate) unsafe fn __rust_thunk__f35fefa5__ZNSt3__u11__wrap_iterIPiEmIEl<'__this>(
            __this: &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE,
            __n: isize,
        ) -> &'__this mut crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE;
        pub(crate) unsafe fn __rust_thunk__041ce97c__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__e6710516__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__01427f91__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__03346ce2__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__5545573d__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__2f1f91b1__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__efeb2c22__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __idx: usize,
        ) -> &'__this ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk__07a5bf78__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__a3ec4015__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__9aeb93a7__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__3fb9ec58__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__84fdcbe6__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__4dfa2303__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__041ce97c__ZNSt3__u4spanIiLm18446744073709551615EEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk__e6710516__ZNKSt3__u4spanIiLm18446744073709551615EE5firstEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__01427f91__ZNKSt3__u4spanIiLm18446744073709551615EE4lastEm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__03346ce2__ZNKSt3__u4spanIiLm18446744073709551615EE7subspanEmm<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __offset: usize,
            __count: usize,
        );
        pub(crate) unsafe fn __rust_thunk__5545573d__ZNKSt3__u4spanIiLm18446744073709551615EE10size_bytesEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> usize;
        pub(crate) unsafe fn __rust_thunk__2f1f91b1__ZNKSt3__u4spanIiLm18446744073709551615EE5emptyEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> bool;
        pub(crate) unsafe fn __rust_thunk__07a5bf78__ZNKSt3__u4spanIiLm18446744073709551615EE5frontEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__a3ec4015__ZNKSt3__u4spanIiLm18446744073709551615EE4backEv<
            '__this,
        >(
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        ) -> ::cref::CMut<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk__9aeb93a7__ZNKSt3__u4spanIiLm18446744073709551615EE5beginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__3fb9ec58__ZNKSt3__u4spanIiLm18446744073709551615EE3endEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__84fdcbe6__ZNKSt3__u4spanIiLm18446744073709551615EE6rbeginEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
        pub(crate) unsafe fn __rust_thunk__4dfa2303__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv<
            '__this,
        >(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::S>() == 4);
    assert!(::core::mem::align_of::<crate::S>() == 4);
    static_assertions::assert_impl_all!(crate::S: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::S: Drop);
    assert!(::core::mem::offset_of!(crate::S, x_) == 0);
    assert!(::core::mem::size_of::<crate::T>() == 8);
    assert!(::core::mem::align_of::<crate::T>() == 4);
    static_assertions::assert_impl_all!(crate::T: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::T: Drop);
    assert!(::core::mem::offset_of!(crate::T, y_) == 4);
    assert!(::core::mem::size_of::<crate::Chars>() == 3);
    assert!(::core::mem::align_of::<crate::Chars>() == 1);
    static_assertions::assert_impl_all!(crate::Chars: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Chars: Drop);
    assert!(::core::mem::offset_of!(crate::Chars, c_) == 0);
    assert!(::core::mem::offset_of!(crate::Chars, sc_) == 1);
    assert!(::core::mem::offset_of!(crate::Chars, uc_) == 2);
    assert!(::core::mem::size_of::<crate::Bools>() == 1);
    assert!(::core::mem::align_of::<crate::Bools>() == 1);
    static_assertions::assert_impl_all!(crate::Bools: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Bools: Drop);
    assert!(::core::mem::offset_of!(crate::Bools, b_) == 0);
    assert!(::core::mem::size_of::<crate::Pointers>() == 16);
    assert!(::core::mem::align_of::<crate::Pointers>() == 8);
    static_assertions::assert_impl_all!(crate::Pointers: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Pointers: Drop);
    assert!(::core::mem::offset_of!(crate::Pointers, p_) == 0);
    assert!(::core::mem::offset_of!(crate::Pointers, mut_p_) == 8);
    assert!(::core::mem::size_of::<crate::NonTrivial>() == 16);
    assert!(::core::mem::align_of::<crate::NonTrivial>() == 8);
    static_assertions::assert_impl_all!(crate::NonTrivial: Drop);
    static_assertions::assert_not_impl_any!(crate::NonTrivial: Copy);
    assert!(::core::mem::offset_of!(crate::NonTrivial, p_) == 0);
    assert!(::core::mem::offset_of!(crate::NonTrivial, mut_p_) == 8);
    assert!(::core::mem::size_of::<crate::Padding>() == 8);
    assert!(::core::mem::align_of::<crate::Padding>() == 8);
    static_assertions::assert_impl_all!(crate::Padding: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Padding: Drop);
    assert!(::core::mem::offset_of!(crate::Padding, pad_) == 0);
    assert!(::core::mem::size_of::<crate::PointerBase>() == 8);
    assert!(::core::mem::align_of::<crate::PointerBase>() == 8);
    static_assertions::assert_impl_all!(crate::PointerBase: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::PointerBase: Drop);
    assert!(::core::mem::offset_of!(crate::PointerBase, base_p_) == 0);
    assert!(::core::mem::size_of::<crate::DerivedPointers>() == 16);
    assert!(::core::mem::align_of::<crate::DerivedPointers>() == 8);
    static_assertions::assert_impl_all!(crate::DerivedPointers: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DerivedPointers: Drop);

    assert!(::core::mem::size_of::<crate::MorePointers>() == 16);
    assert!(::core::mem::align_of::<crate::MorePointers>() == 8);
    static_assertions::assert_impl_all!(crate::MorePointers: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::MorePointers: Drop);
    assert!(::core::mem::offset_of!(crate::MorePointers, v_) == 0);
    assert!(::core::mem::offset_of!(crate::MorePointers, i_) == 8);
    assert!(::core::mem::size_of::<crate::Slices>() == 16);
    assert!(::core::mem::align_of::<crate::Slices>() == 8);
    static_assertions::assert_impl_all!(crate::Slices: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Slices: Drop);
    assert!(::core::mem::offset_of!(crate::Slices, s_) == 0);
    assert!(::core::mem::size_of::<crate::StructWithRecordAndFreeGetter>() == 4);
    assert!(::core::mem::align_of::<crate::StructWithRecordAndFreeGetter>() == 4);
    static_assertions::assert_impl_all!(crate::StructWithRecordAndFreeGetter: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::StructWithRecordAndFreeGetter: Drop);
    assert!(::core::mem::offset_of!(crate::StructWithRecordAndFreeGetter, s_) == 0);
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEE,
            current
        ) == 0
    );
    assert!(
        ::core::mem::size_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
        >() == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEE,
            current
        ) == 0
    );
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPKiEE, __i_) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>() == 8);
    assert!(::core::mem::align_of::<crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE>() == 8);
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE: Drop);
    assert!(::core::mem::offset_of!(crate::__CcTemplateInstNSt3__u11__wrap_iterIPiEE, __i_) == 0);
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIKiLm18446744073709551615EEE,
            __size_
        ) == 8
    );
    assert!(
        ::core::mem::size_of::<crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE>()
            == 8
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE: Drop);
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __data_
        ) == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::__CcTemplateInstNSt3__u4spanIiLm18446744073709551615EEE,
            __size_
        ) == 8
    );
    assert!(::core::mem::size_of::<*const [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*const [::ffi_11::c_int]>() == 8);
    assert!(::core::mem::size_of::<*mut [::ffi_11::c_int]>() == 16);
    assert!(::core::mem::align_of::<*mut [::ffi_11::c_int]>() == 8);
};
