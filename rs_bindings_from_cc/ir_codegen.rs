// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Code generator for `#[repr(C)]` IR types shared in-memory between C++ (`ir_generated.h`)
//! and Rust (`ir_generated.rs`).
//!
//! Run `common/golden_update.sh` after modifying the schema in this file.

use std::env;
use std::fmt::Write as _;
use std::fs;

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum FieldType {
    Bool,
    U64,
    I64,
    Str,
    Named(&'static str),
    Vec(Box<FieldType>),
    Option(Box<FieldType>),
    Rc(Box<FieldType>),
}

impl FieldType {
    fn vec(inner: FieldType) -> Self {
        Self::Vec(Box::new(inner))
    }
    fn option(inner: FieldType) -> Self {
        Self::Option(Box::new(inner))
    }
    fn rc(inner: FieldType) -> Self {
        Self::Rc(Box::new(inner))
    }

    fn cc_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::U64 => "uint64_t".to_string(),
            Self::I64 => "int64_t".to_string(),
            Self::Str => "FfiStr".to_string(),
            Self::Named(name) => format!("::crubit::ffi::{name}"),
            Self::Vec(inner) => format!("FfiVec<{}>", inner.cc_type()),
            Self::Option(inner) => format!("FfiOption<{}>", inner.cc_type()),
            Self::Rc(inner) => format!("FfiRc<{}>", inner.cc_type()),
        }
    }

    fn rs_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::U64 => "u64".to_string(),
            Self::I64 => "i64".to_string(),
            Self::Str => "FfiStr".to_string(),
            Self::Named(name) => (*name).to_string(),
            Self::Vec(inner) => format!("FfiVec<{}>", inner.rs_type()),
            Self::Option(inner) => format!("FfiOption<{}>", inner.rs_type()),
            Self::Rc(inner) => format!("FfiRc<{}>", inner.rs_type()),
        }
    }
}

struct FieldDef {
    name: &'static str,
    ty: FieldType,
}

struct VariantDef {
    name: &'static str,
    payload: Option<FieldType>,
}

enum TypeDef {
    Struct { name: &'static str, doc: &'static str, fields: Vec<FieldDef> },
    CStyleEnum { name: &'static str, doc: &'static str, variants: Vec<&'static str> },
    TaggedEnum { name: &'static str, doc: &'static str, variants: Vec<VariantDef> },
}

macro_rules! field_ty {
    (Bool) => {
        FieldType::Bool
    };
    (U64) => {
        FieldType::U64
    };
    (I64) => {
        FieldType::I64
    };
    (Str) => {
        FieldType::Str
    };
    (Named($name:literal)) => {
        FieldType::Named($name)
    };
    (Vec($inner:tt $( ( $($args:tt)* ) )?)) => {
        FieldType::vec(field_ty!($inner $( ( $($args)* ) )?))
    };
    (Option($inner:tt $( ( $($args:tt)* ) )?)) => {
        FieldType::option(field_ty!($inner $( ( $($args)* ) )?))
    };
    (Rc($inner:tt $( ( $($args:tt)* ) )?)) => {
        FieldType::rc(field_ty!($inner $( ( $($args)* ) )?))
    };
}

macro_rules! variant_def {
    ($vname:ident ( $vty:tt $( ( $($vargs:tt)* ) )? )) => {
        VariantDef {
            name: stringify!($vname),
            payload: Some(field_ty!($vty $( ( $($vargs)* ) )?)),
        }
    };
    ($vname:ident) => {
        VariantDef {
            name: stringify!($vname),
            payload: None,
        }
    };
}

macro_rules! ir_schema_item {
    (
        $types:ident;
        #[doc = $doc:literal]
        struct $name:ident {
            $( $fname:ident : $fty:tt $( ( $($fargs:tt)* ) )? ),* $(,)?
        }
    ) => {
        $types.push(TypeDef::Struct {
            name: stringify!($name),
            doc: $doc.trim(),
            fields: vec![
                $(
                    FieldDef {
                        name: stringify!($fname),
                        ty: field_ty!($fty $( ( $($fargs)* ) )?),
                    }
                ),*
            ],
        });
    };
    (
        $types:ident;
        #[doc = $doc:literal]
        enum $name:ident {
            $( $vname:ident ),* $(,)?
        }
    ) => {
        $types.push(TypeDef::CStyleEnum {
            name: stringify!($name),
            doc: $doc.trim(),
            variants: vec![ $( stringify!($vname) ),* ],
        });
    };
    (
        $types:ident;
        #[doc = $doc:literal]
        union $name:ident {
            $( $vname:ident $( ( $vty:tt $( ( $($vargs:tt)* ) )? ) )? ),* $(,)?
        }
    ) => {
        $types.push(TypeDef::TaggedEnum {
            name: stringify!($name),
            doc: $doc.trim(),
            variants: vec![
                $( variant_def!($vname $( ( $vty $( ( $($vargs)* ) )? ) )?) ),*
            ],
        });
    };
}

macro_rules! ir_schema {
    (
        $(
            #[doc = $doc:literal]
            $kind:ident $name:ident {
                $($body:tt)*
            }
        )*
    ) => {{
        let mut types = Vec::new();
        $(
            ir_schema_item!(types; #[doc = $doc] $kind $name { $($body)* });
        )*
        types
    }};
}

fn schema() -> Vec<TypeDef> {
    ir_schema! {
        /// A name of a public header of the C++ library.
        struct HeaderName {
            name: Str,
        }
        /// Special non-identifier C++ function names.
        enum SpecialName {
            Destructor,
            Constructor,
        }
        /// An identifier in C++ or Rust.
        struct Identifier {
            identifier: Str,
        }
        /// A C++ overloaded operator name.
        struct Operator {
            name: Str,
        }
        /// An unqualified C++ function name (identifier, operator, constructor, or destructor).
        union UnqualifiedIdentifier {
            Identifier(Named("Identifier")),
            Operator(Named("Operator")),
            Constructor,
            Destructor,
        }
        /// Representative recursive IR node exercising all FFI vocabulary primitives.
        struct FfiTestNode {
            id: U64,
            is_inline: Bool,
            label: Option(Str),
            kind: Named("UnqualifiedIdentifier"),
            headers: Vec(Named("HeaderName")),
            children: Vec(Rc(Named("FfiTestNode"))),
        }
    }
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn generate_cc(types: &[TypeDef]) -> String {
    let mut out = String::new();
    writeln!(out, "// Automatically @generated by ir_codegen.rs. Do not edit directly.").unwrap();
    writeln!(out, "// To regenerate, run: common/golden_update.sh").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "// clang-format off").unwrap();
    writeln!(out, "#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_").unwrap();
    writeln!(out, "#define CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#include <cstdint>").unwrap();
    writeln!(out, "#include <memory>").unwrap();
    writeln!(out, "#include <new>").unwrap();
    writeln!(out, "#include <utility>").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#include \"third_party/absl/log/check.h\"").unwrap();
    writeln!(out, "#include \"rs_bindings_from_cc/ir_ffi_support.h\"").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "namespace crubit::ffi {{").unwrap();
    writeln!(out).unwrap();

    for ty in types {
        match ty {
            TypeDef::Struct { name, doc, fields } => {
                writeln!(out, "// {doc}").unwrap();
                writeln!(out, "struct {name} {{").unwrap();
                for f in fields {
                    writeln!(out, "  {} {};", f.ty.cc_type(), f.name).unwrap();
                }
                writeln!(out).unwrap();
                writeln!(out, "  friend bool operator==(const {name}&, const {name}&) = default;")
                    .unwrap();
                writeln!(out, "}};").unwrap();
                writeln!(out).unwrap();
            }
            TypeDef::CStyleEnum { name, doc, variants } => {
                writeln!(out, "// {doc}").unwrap();
                writeln!(out, "enum class {name} : uint32_t {{").unwrap();
                for (i, v) in variants.iter().enumerate() {
                    writeln!(out, "  k{v} = {i},").unwrap();
                }
                writeln!(out, "}};").unwrap();
                writeln!(out).unwrap();
            }
            TypeDef::TaggedEnum { name, doc, variants } => {
                writeln!(out, "// {doc}").unwrap();
                writeln!(out, "// Layout matches Rust's `#[repr(C, u32)] pub enum {name}`.")
                    .unwrap();
                writeln!(out, "class {name} {{").unwrap();
                writeln!(out, " public:").unwrap();
                writeln!(out, "  enum class Tag : uint32_t {{").unwrap();
                for (i, v) in variants.iter().enumerate() {
                    writeln!(out, "    k{} = {i},", v.name).unwrap();
                }
                writeln!(out, "  }};").unwrap();
                writeln!(out).unwrap();

                // Constructors for each variant.
                for v in variants {
                    let vname = v.name;
                    let snake = to_snake_case(vname);
                    if let Some(payload) = &v.payload {
                        let pty = payload.cc_type();
                        writeln!(out, "  static {name} {vname}({pty} value) {{").unwrap();
                        writeln!(out, "    {name} result(Tag::k{vname});").unwrap();
                        writeln!(out, "    new (&result.{snake}_) {pty}(std::move(value));")
                            .unwrap();
                        writeln!(out, "    return result;").unwrap();
                        writeln!(out, "  }}").unwrap();
                    } else {
                        writeln!(out, "  static {name} {vname}() {{").unwrap();
                        writeln!(out, "    return {name}(Tag::k{vname});").unwrap();
                        writeln!(out, "  }}").unwrap();
                    }
                }
                writeln!(out).unwrap();

                // Copy constructor.
                writeln!(out, "  {name}(const {name}& other) : tag_(other.tag_), empty_(0) {{")
                    .unwrap();
                writeln!(out, "    switch (tag_) {{").unwrap();
                for v in variants {
                    if let Some(payload) = &v.payload {
                        let vname = v.name;
                        let snake = to_snake_case(vname);
                        let pty = payload.cc_type();
                        writeln!(out, "      case Tag::k{vname}:").unwrap();
                        writeln!(out, "        new (&{snake}_) {pty}(other.{snake}_);").unwrap();
                        writeln!(out, "        break;").unwrap();
                    } else {
                        writeln!(out, "      case Tag::k{}:", v.name).unwrap();
                        writeln!(out, "        break;").unwrap();
                    }
                }
                writeln!(out, "    }}").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out).unwrap();

                // Move constructor.
                writeln!(out, "  {name}({name}&& other) noexcept : tag_(other.tag_), empty_(0) {{")
                    .unwrap();
                writeln!(out, "    switch (tag_) {{").unwrap();
                for v in variants {
                    if let Some(payload) = &v.payload {
                        let vname = v.name;
                        let snake = to_snake_case(vname);
                        let pty = payload.cc_type();
                        writeln!(out, "      case Tag::k{vname}:").unwrap();
                        writeln!(out, "        new (&{snake}_) {pty}(std::move(other.{snake}_));")
                            .unwrap();
                        writeln!(out, "        break;").unwrap();
                    } else {
                        writeln!(out, "      case Tag::k{}:", v.name).unwrap();
                        writeln!(out, "        break;").unwrap();
                    }
                }
                writeln!(out, "    }}").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out).unwrap();

                writeln!(out, "  {name}& operator=(const {name}& other) {{").unwrap();
                writeln!(out, "    if (this != &other) {{").unwrap();
                writeln!(out, "      Destroy();").unwrap();
                writeln!(out, "      new (this) {name}(other);").unwrap();
                writeln!(out, "    }}").unwrap();
                writeln!(out, "    return *this;").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out).unwrap();

                writeln!(out, "  {name}& operator=({name}&& other) noexcept {{").unwrap();
                writeln!(out, "    if (this != &other) {{").unwrap();
                writeln!(out, "      Destroy();").unwrap();
                writeln!(out, "      new (this) {name}(std::move(other));").unwrap();
                writeln!(out, "    }}").unwrap();
                writeln!(out, "    return *this;").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out).unwrap();

                writeln!(out, "  ~{name}() {{ Destroy(); }}").unwrap();
                writeln!(out).unwrap();
                writeln!(out, "  Tag tag() const {{ return tag_; }}").unwrap();
                for v in variants {
                    if let Some(payload) = &v.payload {
                        let vname = v.name;
                        let snake = to_snake_case(vname);
                        let pty = payload.cc_type();
                        writeln!(out, "  const {pty}& as_{snake}() const {{").unwrap();
                        writeln!(out, "    CHECK(tag_ == Tag::k{vname});").unwrap();
                        writeln!(out, "    return {snake}_;").unwrap();
                        writeln!(out, "  }}").unwrap();
                    }
                }
                writeln!(out).unwrap();

                writeln!(out, "  friend bool operator==(const {name}& lhs, const {name}& rhs) {{")
                    .unwrap();
                writeln!(out, "    if (lhs.tag_ != rhs.tag_) return false;").unwrap();
                writeln!(out, "    switch (lhs.tag_) {{").unwrap();
                for v in variants {
                    let vname = v.name;
                    let snake = to_snake_case(vname);
                    writeln!(out, "      case Tag::k{vname}:").unwrap();
                    if v.payload.is_some() {
                        writeln!(out, "        return lhs.{snake}_ == rhs.{snake}_;").unwrap();
                    } else {
                        writeln!(out, "        return true;").unwrap();
                    }
                }
                writeln!(out, "    }}").unwrap();
                writeln!(out, "    return false;").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out).unwrap();

                writeln!(out, " private:").unwrap();
                writeln!(out, "  explicit {name}(Tag tag) : tag_(tag), empty_(0) {{}}").unwrap();
                writeln!(out, "  void Destroy() {{").unwrap();
                writeln!(out, "    switch (tag_) {{").unwrap();
                for v in variants {
                    if v.payload.is_some() {
                        let vname = v.name;
                        let snake = to_snake_case(vname);
                        writeln!(out, "      case Tag::k{vname}:").unwrap();
                        writeln!(out, "        std::destroy_at(&{snake}_);").unwrap();
                        writeln!(out, "        break;").unwrap();
                    } else {
                        writeln!(out, "      case Tag::k{}:", v.name).unwrap();
                        writeln!(out, "        break;").unwrap();
                    }
                }
                writeln!(out, "    }}").unwrap();
                writeln!(out, "  }}").unwrap();
                writeln!(out).unwrap();
                writeln!(out, "  Tag tag_;").unwrap();
                writeln!(out, "  union {{").unwrap();
                writeln!(out, "    char empty_;").unwrap();
                for v in variants {
                    if let Some(payload) = &v.payload {
                        let snake = to_snake_case(v.name);
                        let pty = payload.cc_type();
                        writeln!(out, "    {pty} {snake}_;").unwrap();
                    }
                }
                writeln!(out, "  }};").unwrap();
                writeln!(out, "}};").unwrap();
                writeln!(out).unwrap();
            }
        }
    }

    writeln!(out, "}}  // namespace crubit::ffi").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_GENERATED_H_").unwrap();
    out
}

fn generate_rs(types: &[TypeDef]) -> String {
    let mut out = String::new();
    writeln!(out, "// Automatically @generated by ir_codegen.rs. Do not edit directly.").unwrap();
    writeln!(out, "// To regenerate, run: common/golden_update.sh").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "#[rustfmt::skip]").unwrap();
    writeln!(out, "use crate::ir_ffi_support::{{FfiOption, FfiRc, FfiStr, FfiVec}};").unwrap();

    for ty in types {
        writeln!(out).unwrap();
        match ty {
            TypeDef::Struct { name, doc, fields } => {
                writeln!(out, "/// {doc}").unwrap();
                writeln!(out, "#[rustfmt::skip]").unwrap();
                writeln!(out, "#[repr(C)]").unwrap();
                writeln!(out, "#[derive(Debug, Clone, PartialEq, Eq, Hash)]").unwrap();
                writeln!(out, "pub struct {name} {{").unwrap();
                for f in fields {
                    writeln!(out, "    pub {}: {},", f.name, f.ty.rs_type()).unwrap();
                }
                writeln!(out, "}}").unwrap();
                writeln!(out).unwrap();

                writeln!(out, "#[rustfmt::skip]").unwrap();
                writeln!(out, "impl {name} {{").unwrap();
                if fields.len() == 1 && matches!(fields[0].ty, FieldType::Str) {
                    let fname = fields[0].name;
                    writeln!(out, "    #[allow(clippy::should_implement_trait)]").unwrap();
                    writeln!(out, "    pub fn from_str({fname}: &str) -> Self {{").unwrap();
                    writeln!(out, "        Self {{ {fname}: FfiStr::from_str({fname}) }}").unwrap();
                    writeln!(out, "    }}").unwrap();
                    writeln!(out).unwrap();
                }
                for f in fields {
                    let fname = f.name;
                    match &f.ty {
                        FieldType::Str => {
                            writeln!(out, "    pub fn {fname}(&self) -> &str {{").unwrap();
                            writeln!(out, "        self.{fname}.as_str()").unwrap();
                            writeln!(out, "    }}").unwrap();
                        }
                        FieldType::Bool | FieldType::U64 | FieldType::I64 => {
                            let rty = f.ty.rs_type();
                            writeln!(out, "    pub fn {fname}(&self) -> {rty} {{").unwrap();
                            writeln!(out, "        self.{fname}").unwrap();
                            writeln!(out, "    }}").unwrap();
                        }
                        FieldType::Named(inner) => {
                            writeln!(out, "    pub fn {fname}(&self) -> &{inner} {{").unwrap();
                            writeln!(out, "        &self.{fname}").unwrap();
                            writeln!(out, "    }}").unwrap();
                        }
                        FieldType::Vec(inner) => {
                            let rty = inner.rs_type();
                            writeln!(out, "    pub fn {fname}(&self) -> &[{rty}] {{").unwrap();
                            writeln!(out, "        self.{fname}.as_slice()").unwrap();
                            writeln!(out, "    }}").unwrap();
                        }
                        FieldType::Option(inner) => {
                            if matches!(**inner, FieldType::Str) {
                                writeln!(out, "    pub fn {fname}(&self) -> Option<&str> {{")
                                    .unwrap();
                                writeln!(out, "        self.{fname}.as_deref()").unwrap();
                                writeln!(out, "    }}").unwrap();
                            } else {
                                let rty = inner.rs_type();
                                writeln!(out, "    pub fn {fname}(&self) -> Option<&{rty}> {{")
                                    .unwrap();
                                writeln!(out, "        self.{fname}.as_ref()").unwrap();
                                writeln!(out, "    }}").unwrap();
                            }
                        }
                        FieldType::Rc(inner) => {
                            let rty = inner.rs_type();
                            writeln!(out, "    pub fn {fname}(&self) -> &{rty} {{").unwrap();
                            writeln!(out, "        &self.{fname}").unwrap();
                            writeln!(out, "    }}").unwrap();
                        }
                    }
                }
                writeln!(out, "}}").unwrap();
            }
            TypeDef::CStyleEnum { name, doc, variants } => {
                writeln!(out, "/// {doc}").unwrap();
                writeln!(out, "#[rustfmt::skip]").unwrap();
                writeln!(out, "#[repr(u32)]").unwrap();
                writeln!(out, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]").unwrap();
                writeln!(out, "pub enum {name} {{").unwrap();
                for (i, v) in variants.iter().enumerate() {
                    writeln!(out, "    {v} = {i},").unwrap();
                }
                writeln!(out, "}}").unwrap();
            }
            TypeDef::TaggedEnum { name, doc, variants } => {
                writeln!(out, "/// {doc}").unwrap();
                writeln!(out, "#[rustfmt::skip]").unwrap();
                writeln!(out, "#[repr(C, u32)]").unwrap();
                writeln!(out, "#[derive(Debug, Clone, PartialEq, Eq, Hash)]").unwrap();
                writeln!(out, "pub enum {name} {{").unwrap();
                for (i, v) in variants.iter().enumerate() {
                    if let Some(payload) = &v.payload {
                        writeln!(out, "    {}({}) = {i},", v.name, payload.rs_type()).unwrap();
                    } else {
                        writeln!(out, "    {} = {i},", v.name).unwrap();
                    }
                }
                writeln!(out, "}}").unwrap();
            }
        }
    }

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 3, "Usage: ir_codegen <out_cc_header> <out_rs_file>");
    let types = schema();
    fs::write(&args[1], generate_cc(&types)).expect("Failed to write generated C++ header");
    fs::write(&args[2], generate_rs(&types)).expect("Failed to write generated Rust file");
}
