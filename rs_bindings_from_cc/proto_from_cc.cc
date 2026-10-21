// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <optional>
#include <utility>

#include "absl/status/statusor.h"
#include "common/ffi_types.h"
#include "rs_bindings_from_cc/cmdline_flags.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/ir_ffi_support.h"
#include "rs_bindings_from_cc/ir_from_cc_dependency.h"
#include "rs_bindings_from_cc/ir_generated.h"
#include "llvm/Support/ErrorHandling.h"
#include "llvm/Support/FormatVariadic.h"

namespace crubit {

// This is intended to be called from Rust tests.
extern "C" FfiU8SliceBox proto_from_cc_dependency(
    FfiU8Slice target_triple, FfiU8Slice header_source,
    FfiU8Slice dependency_header_source, FfiU8Slice extra_feature,
    bool kythe_annotations, bool carcinize, bool lazy_import_alien_decls,
    bool check_importer_invariants) {
  absl::StatusOr<IR> ir = IrFromCcDependency(
      target_triple, header_source, dependency_header_source, extra_feature,
      kythe_annotations,
      carcinize ? CarcinizeMode::kStrict : CarcinizeMode::kOff,
      lazy_import_alien_decls, check_importer_invariants);

  if (!ir.ok()) {
    llvm::report_fatal_error(llvm::formatv("IrFromCc reported an error: {0}",
                                           ir.status().message()));
  }
  return AllocFfiU8SliceBox(MakeFfiU8Slice(ir->SerializeAsString()));
}

extern "C" ffi::FfiTestNode crubit_test_make_ffi_node() {
  ffi::FfiTestNode child{
      .id = 100,
      .is_inline = false,
      .label = std::nullopt,
      .kind = ffi::UnqualifiedIdentifier::Constructor(),
      .headers = {},
      .children = {},
  };
  return ffi::FfiTestNode{
      .id = 42,
      .is_inline = true,
      .label = ffi::FfiStr("root_node"),
      .kind = ffi::UnqualifiedIdentifier::Operator(
          ffi::Operator{.name = ffi::FfiStr("+=")}),
      .headers = {HeaderName("foo.h").AsFfi(), HeaderName("bar.h").AsFfi()},
      .children = {ffi::FfiRc<ffi::FfiTestNode>(std::move(child))},
  };
}

}  // namespace crubit
