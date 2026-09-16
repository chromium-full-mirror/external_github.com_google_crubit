// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/semantic_import:semantic_import

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/function/semantic_import/semantic_import.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class S) == 4);
static_assert(alignof(class S) == 4);

extern "C" void __rust_thunk___ZN1SC1Ei(class S* __this, int x) {
  crubit::construct_at(__this, x);
}

static_assert(CRUBIT_SIZEOF(class T) == 8);
static_assert(alignof(class T) == 4);

extern "C" void __rust_thunk___ZN1TC1Eif(class T* __this, int x, float y) {
  crubit::construct_at(__this, x, y);
}

static_assert(sizeof(class Chars) == 3);
static_assert(alignof(class Chars) == 1);

extern "C" void __rust_thunk___ZN5CharsC1Ev(class Chars* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(class Bools) == 1);
static_assert(alignof(class Bools) == 1);

extern "C" void __rust_thunk___ZN5BoolsC1Ev(class Bools* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class Pointers) == 16);
static_assert(alignof(class Pointers) == 8);

extern "C" void __rust_thunk___ZN8PointersC1Ev(class Pointers* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class NonTrivial) == 16);
static_assert(alignof(class NonTrivial) == 8);

extern "C" void __rust_thunk___ZN10NonTrivialC1ERKS_(
    class NonTrivial* __this, class NonTrivial const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" class NonTrivial* __rust_thunk___ZN10NonTrivialaSERKS_(
    class NonTrivial* __this, class NonTrivial const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((class NonTrivial & (::NonTrivial::*)(class NonTrivial const&)) &
              ::NonTrivial::operator=);

extern "C" void __rust_thunk___ZN10NonTrivialC1Ev(class NonTrivial* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN10NonTrivialD1Ev(class NonTrivial* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(class Padding) == 8);
static_assert(alignof(class Padding) == 8);
static_assert(CRUBIT_OFFSET_OF(pad_, class Padding) == 0);

extern "C" void __rust_thunk___ZN7PaddingC1Ev(class Padding* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class PointerBase) == 8);
static_assert(alignof(class PointerBase) == 8);

extern "C" void __rust_thunk___ZN11PointerBaseC1Ev(class PointerBase* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class DerivedPointers) == 16);
static_assert(alignof(class DerivedPointers) == 8);

extern "C" void __rust_thunk___ZN15DerivedPointersC1Ev(
    class DerivedPointers* __this) {
  crubit::construct_at(__this);
}

extern "C" int const* __rust_thunk___ZNK15DerivedPointers1pEv(
    class DerivedPointers const* __this) {
  return __this->p();
}

static_assert((int const* (::DerivedPointers::*)() const) &
              ::DerivedPointers::p);

extern "C" void __rust_thunk___ZN15DerivedPointers5set_pEPKi(
    class DerivedPointers* __this, int const* p) {
  __this->set_p(p);
}

static_assert((void (::DerivedPointers::*)(int const*)) &
              ::DerivedPointers::set_p);

static_assert(CRUBIT_SIZEOF(class MorePointers) == 16);
static_assert(alignof(class MorePointers) == 8);

extern "C" void __rust_thunk___ZN12MorePointersC1Ev(
    class MorePointers* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class Slices) == 16);
static_assert(alignof(class Slices) == 8);

extern "C" void __rust_thunk___ZN6SlicesC1Ev(class Slices* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6Slices5set_sEN2rs8SliceRefIKiEE(
    class Slices* __this, ::rs::SliceRef<const int> s) {
  __this->set_s(s);
}

static_assert((void (::Slices::*)(::rs::SliceRef<const int>)) &
              ::Slices::set_s);

static_assert(CRUBIT_SIZEOF(class StructWithRecordAndFreeGetter) == 4);
static_assert(alignof(class StructWithRecordAndFreeGetter) == 4);

extern "C" void __rust_thunk___ZN29StructWithRecordAndFreeGetterC1Ei(
    class StructWithRecordAndFreeGetter* __this, int x) {
  crubit::construct_at(__this, x);
}

static_assert(
    CRUBIT_SIZEOF(
        class std::reverse_iterator<class std::__wrap_iter<const int*>>) == 8);
static_assert(
    alignof(class std::reverse_iterator<class std::__wrap_iter<const int*>>) ==
    8);

extern "C" void
__rust_thunk__116c07cb__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1Ev(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__9046235f__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPKiEEEC1ES4_(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __this,
    class std::__wrap_iter<const int*>* __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

static_assert(
    CRUBIT_SIZEOF(class std::reverse_iterator<class std::__wrap_iter<int*>>) ==
    8);
static_assert(
    alignof(class std::reverse_iterator<class std::__wrap_iter<int*>>) == 8);

extern "C" void
__rust_thunk__116c07cb__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1Ev(
    class std::reverse_iterator<class std::__wrap_iter<int*>>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__9046235f__ZNSt3__u16reverse_iteratorINS_11__wrap_iterIPiEEEC1ES3_(
    class std::reverse_iterator<class std::__wrap_iter<int*>>* __this,
    class std::__wrap_iter<int*>* __x) {
  crubit::construct_at(__this, crubit::UnsafeTakeValueOnConversion(__x));
}

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<const int*>) == 8);
static_assert(alignof(class std::__wrap_iter<const int*>) == 8);

extern "C" void __rust_thunk__1eeb8fed__ZNSt3__u11__wrap_iterIPKiEC1Ev(
    class std::__wrap_iter<const int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__ee336382__ZNKSt3__u11__wrap_iterIPKiEplEl(
    class std::__wrap_iter<const int*>* __return,
    class std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::__wrap_iter<const int*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const int*>::operator+);

extern "C" class std::__wrap_iter<const int*>*
__rust_thunk__aec51f55__ZNSt3__u11__wrap_iterIPKiEpLEl(
    class std::__wrap_iter<const int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<const int*> &
               (::std::__wrap_iter<const int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const int*>::operator+=);

extern "C" void __rust_thunk__a5af0030__ZNKSt3__u11__wrap_iterIPKiEmiEl(
    class std::__wrap_iter<const int*>* __return,
    class std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::__wrap_iter<const int*>::*)(ptrdiff_t) const) &
              ::std::__wrap_iter<const int*>::operator-);

extern "C" class std::__wrap_iter<const int*>*
__rust_thunk__f35fefa5__ZNSt3__u11__wrap_iterIPKiEmIEl(
    class std::__wrap_iter<const int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<const int*> &
               (::std::__wrap_iter<const int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<const int*>::operator-=);

extern "C" int const* __rust_thunk__0248de23__ZNKSt3__u11__wrap_iterIPKiEixEl(
    class std::__wrap_iter<const int*> const* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator[](__n));
}

static_assert((int const& (::std::__wrap_iter<const int*>::*)(ptrdiff_t)
                   const) &
              ::std::__wrap_iter<const int*>::operator[]);

static_assert(CRUBIT_SIZEOF(class std::__wrap_iter<int*>) == 8);
static_assert(alignof(class std::__wrap_iter<int*>) == 8);

extern "C" void __rust_thunk__1eeb8fed__ZNSt3__u11__wrap_iterIPiEC1Ev(
    class std::__wrap_iter<int*>* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk__ee336382__ZNKSt3__u11__wrap_iterIPiEplEl(
    class std::__wrap_iter<int*>* __return,
    class std::__wrap_iter<int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator+(__n));
}

static_assert((class std::__wrap_iter<int*> (::std::__wrap_iter<int*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<int*>::operator+);

extern "C" class std::__wrap_iter<int*>*
__rust_thunk__aec51f55__ZNSt3__u11__wrap_iterIPiEpLEl(
    class std::__wrap_iter<int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator+=(__n));
}

static_assert((class std::__wrap_iter<int*> &
               (::std::__wrap_iter<int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<int*>::operator+=);

extern "C" void __rust_thunk__a5af0030__ZNKSt3__u11__wrap_iterIPiEmiEl(
    class std::__wrap_iter<int*>* __return,
    class std::__wrap_iter<int*> const* __this, ptrdiff_t __n) {
  new (__return) auto(__this->operator-(__n));
}

static_assert((class std::__wrap_iter<int*> (::std::__wrap_iter<int*>::*)(
                  ptrdiff_t) const) &
              ::std::__wrap_iter<int*>::operator-);

extern "C" class std::__wrap_iter<int*>*
__rust_thunk__f35fefa5__ZNSt3__u11__wrap_iterIPiEmIEl(
    class std::__wrap_iter<int*>* __this, ptrdiff_t __n) {
  return std::addressof(__this->operator-=(__n));
}

static_assert((class std::__wrap_iter<int*> &
               (::std::__wrap_iter<int*>::*)(ptrdiff_t)) &
              ::std::__wrap_iter<int*>::operator-=);

static_assert(
    CRUBIT_SIZEOF(class std::span<const int, 18446744073709551615UL>) == 16);
static_assert(alignof(class std::span<const int, 18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__041ce97c__ZNSt3__u4spanIKiLm18446744073709551615EEC1Ev(
    class std::span<const int, 18446744073709551615UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__e6710516__ZNKSt3__u4spanIKiLm18446744073709551615EE5firstEm(
    class std::span<const int, 18446744073709551615UL>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert((class std::span<const int, 18446744073709551615UL> (
                  ::std::span<const int, 18446744073709551615UL>::*)(size_t)
                   const) &
              ::std::span<const int, 18446744073709551615UL>::first);

extern "C" void
__rust_thunk__01427f91__ZNKSt3__u4spanIKiLm18446744073709551615EE4lastEm(
    class std::span<const int, 18446744073709551615UL>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert((class std::span<const int, 18446744073709551615UL> (
                  ::std::span<const int, 18446744073709551615UL>::*)(size_t)
                   const) &
              ::std::span<const int, 18446744073709551615UL>::last);

extern "C" void
__rust_thunk__03346ce2__ZNKSt3__u4spanIKiLm18446744073709551615EE7subspanEmm(
    class std::span<const int, 18446744073709551615UL>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this,
    size_t __offset, size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert((class std::span<const int, 18446744073709551615UL> (
                  ::std::span<const int, 18446744073709551615UL>::*)(size_t,
                                                                     size_t)
                   const) &
              ::std::span<const int, 18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__5545573d__ZNKSt3__u4spanIKiLm18446744073709551615EE10size_bytesEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert((size_t (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__2f1f91b1__ZNKSt3__u4spanIKiLm18446744073709551615EE5emptyEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::empty);

extern "C" int const*
__rust_thunk__efeb2c22__ZNKSt3__u4spanIKiLm18446744073709551615EEixEm(
    class std::span<const int, 18446744073709551615UL> const* __this,
    size_t __idx) {
  return std::addressof(__this->operator[](__idx));
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)(
                  size_t) const) &
              ::std::span<const int, 18446744073709551615UL>::operator[]);

extern "C" int const*
__rust_thunk__07a5bf78__ZNKSt3__u4spanIKiLm18446744073709551615EE5frontEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::front);

extern "C" int const*
__rust_thunk__a3ec4015__ZNKSt3__u4spanIKiLm18446744073709551615EE4backEv(
    class std::span<const int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert((int const& (::std::span<const int, 18446744073709551615UL>::*)()
                   const) &
              ::std::span<const int, 18446744073709551615UL>::back);

extern "C" void
__rust_thunk__9aeb93a7__ZNKSt3__u4spanIKiLm18446744073709551615EE5beginEv(
    class std::__wrap_iter<const int*>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__3fb9ec58__ZNKSt3__u4spanIKiLm18446744073709551615EE3endEv(
    class std::__wrap_iter<const int*>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert((class std::__wrap_iter<const int*> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::end);

extern "C" void
__rust_thunk__84fdcbe6__ZNKSt3__u4spanIKiLm18446744073709551615EE6rbeginEv(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<const int*>> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__4dfa2303__ZNKSt3__u4spanIKiLm18446744073709551615EE4rendEv(
    class std::reverse_iterator<class std::__wrap_iter<const int*>>* __return,
    class std::span<const int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<const int*>> (
                  ::std::span<const int, 18446744073709551615UL>::*)() const) &
              ::std::span<const int, 18446744073709551615UL>::rend);

static_assert(CRUBIT_SIZEOF(class std::span<int, 18446744073709551615UL>) ==
              16);
static_assert(alignof(class std::span<int, 18446744073709551615UL>) == 8);

extern "C" void
__rust_thunk__041ce97c__ZNSt3__u4spanIiLm18446744073709551615EEC1Ev(
    class std::span<int, 18446744073709551615UL>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk__e6710516__ZNKSt3__u4spanIiLm18446744073709551615EE5firstEm(
    class std::span<int, 18446744073709551615UL>* __return,
    class std::span<int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->first(__count));
}

static_assert((class std::span<int, 18446744073709551615UL> (
                  ::std::span<int, 18446744073709551615UL>::*)(size_t) const) &
              ::std::span<int, 18446744073709551615UL>::first);

extern "C" void
__rust_thunk__01427f91__ZNKSt3__u4spanIiLm18446744073709551615EE4lastEm(
    class std::span<int, 18446744073709551615UL>* __return,
    class std::span<int, 18446744073709551615UL> const* __this,
    size_t __count) {
  new (__return) auto(__this->last(__count));
}

static_assert((class std::span<int, 18446744073709551615UL> (
                  ::std::span<int, 18446744073709551615UL>::*)(size_t) const) &
              ::std::span<int, 18446744073709551615UL>::last);

extern "C" void
__rust_thunk__03346ce2__ZNKSt3__u4spanIiLm18446744073709551615EE7subspanEmm(
    class std::span<int, 18446744073709551615UL>* __return,
    class std::span<int, 18446744073709551615UL> const* __this, size_t __offset,
    size_t __count) {
  new (__return) auto(__this->subspan(__offset, __count));
}

static_assert((class std::span<int, 18446744073709551615UL> (
                  ::std::span<int, 18446744073709551615UL>::*)(size_t, size_t)
                   const) &
              ::std::span<int, 18446744073709551615UL>::subspan);

extern "C" size_t
__rust_thunk__5545573d__ZNKSt3__u4spanIiLm18446744073709551615EE10size_bytesEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return __this->size_bytes();
}

static_assert((size_t (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::size_bytes);

extern "C" bool
__rust_thunk__2f1f91b1__ZNKSt3__u4spanIiLm18446744073709551615EE5emptyEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return __this->empty();
}

static_assert((bool (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::empty);

extern "C" int*
__rust_thunk__07a5bf78__ZNKSt3__u4spanIiLm18446744073709551615EE5frontEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->front());
}

static_assert((int& (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::front);

extern "C" int*
__rust_thunk__a3ec4015__ZNKSt3__u4spanIiLm18446744073709551615EE4backEv(
    class std::span<int, 18446744073709551615UL> const* __this) {
  return std::addressof(__this->back());
}

static_assert((int& (::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::back);

extern "C" void
__rust_thunk__9aeb93a7__ZNKSt3__u4spanIiLm18446744073709551615EE5beginEv(
    class std::__wrap_iter<int*>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->begin());
}

static_assert((class std::__wrap_iter<int*> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::begin);

extern "C" void
__rust_thunk__3fb9ec58__ZNKSt3__u4spanIiLm18446744073709551615EE3endEv(
    class std::__wrap_iter<int*>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->end());
}

static_assert((class std::__wrap_iter<int*> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::end);

extern "C" void
__rust_thunk__84fdcbe6__ZNKSt3__u4spanIiLm18446744073709551615EE6rbeginEv(
    class std::reverse_iterator<class std::__wrap_iter<int*>>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rbegin());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<int*>> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::rbegin);

extern "C" void
__rust_thunk__4dfa2303__ZNKSt3__u4spanIiLm18446744073709551615EE4rendEv(
    class std::reverse_iterator<class std::__wrap_iter<int*>>* __return,
    class std::span<int, 18446744073709551615UL> const* __this) {
  new (__return) auto(__this->rend());
}

static_assert((class std::reverse_iterator<class std::__wrap_iter<int*>> (
                  ::std::span<int, 18446744073709551615UL>::*)() const) &
              ::std::span<int, 18446744073709551615UL>::rend);

#pragma clang diagnostic pop
