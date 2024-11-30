#![allow(dead_code)]
use crate::{ast::Operator, evaluator::formatting::display_value};

use super::value::Value;

pub fn format_mismatched_types(expected: &str, found: &Value) -> String {
  format!("expected `{}`, found `{}`", expected, display_value(found))
}
pub fn format_error<T: ToString>(err: T) -> String {
  format!("error: {}", err.to_string().to_lowercase())
}

pub fn format_missing_property(property: &Value) -> String {
  format!("missing property `{}`", display_value(property))
}

pub fn format_missing_module(module: &str) -> String {
  format!("not found module `{}`", module)
}

pub fn format_expected_function(value: &Value) -> String {
  format!("expected function, found `{}`", display_value(value))
}

pub fn format_expected_number(value: &Value) -> String {
  format!("expected number, found `{}`", display_value(value))
}

pub fn format_undeclared_variable(name: &str) -> String {
  format!("cannot find value `{}` in this scope", name)
}

// cannot access field of non-object
pub fn format_missing_field() -> String {
  format!("cannot access field of non-object")
}

pub fn format_missing_fn_in_module(name: &str, module: &str) -> String {
  format!("not found `{}`, in `{}`", name, module)
}

pub fn format_missing_method(name: &str) -> String {
  format!("not found property `{}`", name)
}

// not found `exit`, in module `process`
pub fn format_function_arity_mismatch(expected: usize, found: usize) -> String {
  match expected {
    1 => format!("expected 1 arg, found `{}`", found),
    _ => format!("expected {} args, found {}", expected, found),
  }
}

pub fn format_option_call_arity_mismatch(found: usize) -> String {
  format!("expected one type argument, found `{}`", found)
}

pub fn format_unsupported_operator(left: &Value, operator: &Operator, right: &Value) -> String {
  use Operator::*;
  match operator {
    ADD => format!("cannot add `{}` and `{}`", display_value(left), display_value(right)),
    SUB => format!("cannot subtract `{}` from `{}`", display_value(left), display_value(right)),
    MUL => format!("cannot multiply `{}` and `{}`", display_value(left), display_value(right)),
    DIV => format!("cannot divide `{}` by `{}`", display_value(left), display_value(right)),
    REM => format!("cannot take modulus of `{}` and `{}`", display_value(left), display_value(right)),
    REQ => format!("cannot take modulus of `{}` and `{}`", display_value(left), display_value(right)),
    EQ => format!("cannot compare `{}` and `{}`", display_value(left), display_value(right)),
    AND => format!("cannot perform `and` on `{}` and `{}`", display_value(left), display_value(right)),
    OR => format!("cannot perform `or` on `{}` and `{}`", display_value(left), display_value(right)),
    NOT => format!("cannot apply logical NOT to `{}`", display_value(right)),
    RANGE => format!("cannot concatenate `{}` and `{}`", display_value(left), display_value(right)),
    SHL => format!("cannot shift `{}` by `{}`", display_value(left), display_value(right)),
    SHR => format!("cannot shift `{}` by `{}`", display_value(left), display_value(right)),
    POW => format!("cannot raise `{}` to the power of `{}`", display_value(left), display_value(right)),
    PIPE => format!("cannot pipe `{}` to `{}`", display_value(left), display_value(right)),
    LE => format!("`{}` cannot be less than `{}`", display_value(left), display_value(right)),
    GE => format!("`{}` cannot be greater than `{}`", display_value(left), display_value(right)),
    LT => format!("`{}` cannot be less than `{}`", display_value(left), display_value(right)),
    GT => format!("`{}` cannot be greater than `{}`", display_value(left), display_value(right)),
    NEQ => format!("cannot compare `{}` and `{}`", display_value(left), display_value(right)),
    XOR => format!("cannot perform `xor` on `{}` and `{}`", display_value(left), display_value(right)),
    BOR => format!("cannot perform `bitwise or` on `{}` and `{}`", display_value(left), display_value(right)),
  }
}

pub fn format_redeclared_in_same_scope(name: &str) -> String {
  format!("`{}` redeclared in same scope", name)
}

pub fn format_module_not_found(name: &str) -> String {
  format!("module `{}` not found", name)
}

pub fn format_module_not_exported(name: &str) -> String {
  format!("module `{}` doesn’t export", name)
}

pub fn format_missing_variable_declaration(name: &str) -> String {
  format!("undeclared value `{}`", name)
}

pub fn format_unexpected_type(expected: &str, found: &str) -> String {
  format!("expected `{}`, found `{}`", expected, found)
}

pub fn format_warning_duplicate_case_in_switch(case: &str) -> String {
  format!("duplicate case `{}` in switch", case)
}
