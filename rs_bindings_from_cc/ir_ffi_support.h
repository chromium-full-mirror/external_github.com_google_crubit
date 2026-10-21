// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_FFI_SUPPORT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IR_FFI_SUPPORT_H_

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <initializer_list>
#include <new>
#include <optional>
#include <ostream>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "common/ffi_types.h"

namespace crubit::ffi {

// An owned UTF-8 string with a C-compatible layout shared between C++ and Rust.
// Memory is allocated and freed via `AllocFfiBytes` / `FreeFfiBytes` so
// ownership can be transferred across FFI in either direction without copying.
class FfiStr {
 public:
  FfiStr() : ptr_(reinterpret_cast<const uint8_t*>(1)), len_(0) {}

  explicit FfiStr(absl::string_view sv) : FfiStr() {
    if (!sv.empty()) {
      uint8_t* buf = static_cast<uint8_t*>(AllocFfiBytes(sv.size(), 1));
      std::memcpy(buf, sv.data(), sv.size());
      ptr_ = buf;
      len_ = sv.size();
    }
  }

  explicit FfiStr(const std::string& s) : FfiStr(absl::string_view(s)) {}
  explicit FfiStr(const char* s) : FfiStr(absl::string_view(s)) {}

  FfiStr(const FfiStr& other) : FfiStr(other.AsView()) {}

  FfiStr(FfiStr&& other) noexcept : ptr_(other.ptr_), len_(other.len_) {
    other.ptr_ = reinterpret_cast<const uint8_t*>(1);
    other.len_ = 0;
  }

  FfiStr& operator=(const FfiStr& other) {
    if (this != &other) {
      *this = FfiStr(other);
    }
    return *this;
  }

  FfiStr& operator=(FfiStr&& other) noexcept {
    if (this != &other) {
      Reset();
      ptr_ = other.ptr_;
      len_ = other.len_;
      other.ptr_ = reinterpret_cast<const uint8_t*>(1);
      other.len_ = 0;
    }
    return *this;
  }

  ~FfiStr() { Reset(); }

  absl::string_view AsView() const {
    return absl::string_view(reinterpret_cast<const char*>(ptr_), len_);
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator absl::string_view() const { return AsView(); }

  const char* data() const { return reinterpret_cast<const char*>(ptr_); }
  size_t size() const { return len_; }
  bool empty() const { return len_ == 0; }

  friend bool operator==(const FfiStr& lhs, const FfiStr& rhs) {
    return lhs.AsView() == rhs.AsView();
  }
  friend bool operator!=(const FfiStr& lhs, const FfiStr& rhs) {
    return !(lhs == rhs);
  }
  friend bool operator==(const FfiStr& lhs, absl::string_view rhs) {
    return lhs.AsView() == rhs;
  }

  template <typename H>
  friend H AbslHashValue(H h, const FfiStr& s) {
    return H::combine(std::move(h), s.AsView());
  }

  friend std::ostream& operator<<(std::ostream& os, const FfiStr& s) {
    return os << s.AsView();
  }

 private:
  void Reset() {
    if (len_ > 0) {
      FreeFfiBytes(const_cast<uint8_t*>(ptr_), len_, 1);
      ptr_ = reinterpret_cast<const uint8_t*>(1);
      len_ = 0;
    }
  }

  const uint8_t* ptr_;
  size_t len_;
};

static_assert(sizeof(FfiStr) == 2 * sizeof(size_t));
static_assert(alignof(FfiStr) == alignof(size_t));

// An owned contiguous vector with a `#[repr(C)]` layout shared between C++ and
// Rust.
template <typename T>
class FfiVec {
 public:
  FfiVec() : ptr_(Dangling()), len_(0), cap_(0) {}

  FfiVec(std::initializer_list<T> init) : FfiVec() {
    reserve(init.size());
    for (const T& item : init) {
      push_back(item);
    }
  }

  template <typename Iter>
  FfiVec(Iter first, Iter last) : FfiVec() {
    for (; first != last; ++first) {
      push_back(*first);
    }
  }

  FfiVec(const FfiVec& other) : FfiVec() {
    reserve(other.len_);
    for (size_t i = 0; i < other.len_; ++i) {
      new (ptr_ + i) T(other.ptr_[i]);
    }
    len_ = other.len_;
  }

  FfiVec(FfiVec&& other) noexcept
      : ptr_(other.ptr_), len_(other.len_), cap_(other.cap_) {
    other.ptr_ = Dangling();
    other.len_ = 0;
    other.cap_ = 0;
  }

  FfiVec& operator=(const FfiVec& other) {
    if (this != &other) {
      *this = FfiVec(other);
    }
    return *this;
  }

  FfiVec& operator=(FfiVec&& other) noexcept {
    if (this != &other) {
      Reset();
      ptr_ = other.ptr_;
      len_ = other.len_;
      cap_ = other.cap_;
      other.ptr_ = Dangling();
      other.len_ = 0;
      other.cap_ = 0;
    }
    return *this;
  }

  ~FfiVec() { Reset(); }

  void reserve(size_t new_cap) {
    if (new_cap <= cap_) return;
    T* new_ptr =
        static_cast<T*>(AllocFfiBytes(new_cap * sizeof(T), alignof(T)));
    for (size_t i = 0; i < len_; ++i) {
      new (new_ptr + i) T(std::move(ptr_[i]));
      ptr_[i].~T();
    }
    if (cap_ > 0) {
      FreeFfiBytes(ptr_, cap_ * sizeof(T), alignof(T));
    }
    ptr_ = new_ptr;
    cap_ = new_cap;
  }

  void push_back(T value) {
    if (len_ == cap_) {
      reserve(cap_ == 0 ? 4 : cap_ * 2);
    }
    new (ptr_ + len_) T(std::move(value));
    ++len_;
  }

  size_t size() const { return len_; }
  bool empty() const { return len_ == 0; }
  const T* data() const { return ptr_; }
  T* data() { return ptr_; }
  const T* begin() const { return ptr_; }
  const T* end() const { return ptr_ + len_; }
  T* begin() { return ptr_; }
  T* end() { return ptr_ + len_; }

  const T& operator[](size_t idx) const {
    CHECK(idx < len_);
    return ptr_[idx];
  }
  T& operator[](size_t idx) {
    CHECK(idx < len_);
    return ptr_[idx];
  }

  absl::Span<const T> AsSpan() const { return absl::Span<const T>(ptr_, len_); }

  friend bool operator==(const FfiVec& lhs, const FfiVec& rhs) {
    return lhs.AsSpan() == rhs.AsSpan();
  }
  friend bool operator!=(const FfiVec& lhs, const FfiVec& rhs) {
    return !(lhs == rhs);
  }

 private:
  static T* Dangling() { return reinterpret_cast<T*>(alignof(T)); }

  void Reset() {
    for (size_t i = 0; i < len_; ++i) {
      ptr_[i].~T();
    }
    if (cap_ > 0) {
      FreeFfiBytes(ptr_, cap_ * sizeof(T), alignof(T));
    }
    ptr_ = Dangling();
    len_ = 0;
    cap_ = 0;
  }

  T* ptr_;
  size_t len_;
  size_t cap_;
};

// Matches Rust's `#[repr(C, u8)] pub enum FfiOption<T> { None = 0, Some(T) = 1
// }`.
template <typename T>
class FfiOption {
 public:
  FfiOption() : tag_(0), empty_(0) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  FfiOption(std::nullopt_t) : FfiOption() {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  FfiOption(T value) : tag_(1), value_(std::move(value)) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  FfiOption(std::optional<T> opt) : FfiOption() {
    if (opt.has_value()) {
      tag_ = 1;
      new (&value_) T(*std::move(opt));
    }
  }

  FfiOption(const FfiOption& other) : tag_(other.tag_), empty_(0) {
    if (tag_ == 1) {
      new (&value_) T(other.value_);
    }
  }

  FfiOption(FfiOption&& other) noexcept : tag_(other.tag_), empty_(0) {
    if (tag_ == 1) {
      new (&value_) T(std::move(other.value_));
      other.Reset();
    }
  }

  FfiOption& operator=(const FfiOption& other) {
    if (this != &other) {
      Reset();
      tag_ = other.tag_;
      if (tag_ == 1) {
        new (&value_) T(other.value_);
      }
    }
    return *this;
  }

  FfiOption& operator=(FfiOption&& other) noexcept {
    if (this != &other) {
      Reset();
      tag_ = other.tag_;
      if (tag_ == 1) {
        new (&value_) T(std::move(other.value_));
        other.Reset();
      }
    }
    return *this;
  }

  ~FfiOption() { Reset(); }

  bool has_value() const { return tag_ == 1; }
  explicit operator bool() const { return has_value(); }

  const T& value() const {
    CHECK(has_value());
    return value_;
  }
  T& value() {
    CHECK(has_value());
    return value_;
  }
  const T& operator*() const { return value(); }
  T& operator*() { return value(); }
  const T* operator->() const { return &value(); }
  T* operator->() { return &value(); }

  friend bool operator==(const FfiOption& lhs, const FfiOption& rhs) {
    if (lhs.tag_ != rhs.tag_) return false;
    return lhs.tag_ == 0 || lhs.value_ == rhs.value_;
  }
  friend bool operator!=(const FfiOption& lhs, const FfiOption& rhs) {
    return !(lhs == rhs);
  }

 private:
  void Reset() {
    if (tag_ == 1) {
      value_.~T();
      tag_ = 0;
    }
  }

  uint8_t tag_;
  union {
    char empty_;
    T value_;
  };
};

// Reference-counted shared pointer with a `#[repr(C)]` control block shared
// between C++ and Rust.
template <typename T>
struct FfiRcInner {
  std::atomic<size_t> strong_count;
  T value;
};

template <typename T>
class FfiRc {
 public:
  explicit FfiRc(T value) {
    void* mem = AllocFfiBytes(sizeof(FfiRcInner<T>), alignof(FfiRcInner<T>));
    ptr_ = new (mem) FfiRcInner<T>{1, std::move(value)};
  }

  FfiRc(const FfiRc& other) noexcept : ptr_(other.ptr_) {
    if (ptr_ != nullptr) {
      ptr_->strong_count.fetch_add(1, std::memory_order_relaxed);
    }
  }

  FfiRc(FfiRc&& other) noexcept : ptr_(other.ptr_) { other.ptr_ = nullptr; }

  FfiRc& operator=(const FfiRc& other) noexcept {
    if (this != &other) {
      Reset();
      ptr_ = other.ptr_;
      if (ptr_ != nullptr) {
        ptr_->strong_count.fetch_add(1, std::memory_order_relaxed);
      }
    }
    return *this;
  }

  FfiRc& operator=(FfiRc&& other) noexcept {
    if (this != &other) {
      Reset();
      ptr_ = other.ptr_;
      other.ptr_ = nullptr;
    }
    return *this;
  }

  ~FfiRc() { Reset(); }

  const T& get() const {
    CHECK(ptr_ != nullptr);
    return ptr_->value;
  }
  const T& operator*() const { return get(); }
  const T* operator->() const { return &get(); }

  friend bool operator==(const FfiRc& lhs, const FfiRc& rhs) {
    if (lhs.ptr_ == rhs.ptr_) return true;
    if (lhs.ptr_ == nullptr || rhs.ptr_ == nullptr) return false;
    return lhs.ptr_->value == rhs.ptr_->value;
  }
  friend bool operator!=(const FfiRc& lhs, const FfiRc& rhs) {
    return !(lhs == rhs);
  }

 private:
  void Reset() {
    if (ptr_ != nullptr) {
      if (ptr_->strong_count.fetch_sub(1, std::memory_order_acq_rel) == 1) {
        ptr_->~FfiRcInner<T>();
        FreeFfiBytes(ptr_, sizeof(FfiRcInner<T>), alignof(FfiRcInner<T>));
      }
      ptr_ = nullptr;
    }
  }

  FfiRcInner<T>* ptr_;
};

template <typename T, typename... Args>
FfiRc<T> MakeFfiRc(Args&&... args) {
  return FfiRc<T>(T{std::forward<Args>(args)...});
}

}  // namespace crubit::ffi

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_FFI_SUPPORT_H_
