// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use googletest::matchers::eq;
use googletest::{expect_that, gtest};
#[gtest]
fn test_globals() {
    use globals::{const_global_var, global_var, tentative_def};
    let _ = unsafe { core::ptr::addr_of!(global_var) };
    let _ = unsafe { core::ptr::addr_of!(const_global_var) };
    let _ = unsafe { core::ptr::addr_of!(tentative_def) };
}
