#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_parens)]

use std::sync::Mutex;

// Core modules (always available)
mod utils;
mod ast;
mod lexer;
mod parser;
mod symbols;
mod codegen;
mod vm;
mod alloc;
mod array;
mod bytearray;
mod runtime;
mod deepcopy;
mod exec_tests;

// Platform-specific modules (only with SDL feature)
#[cfg(feature = "sdl")]
mod host;
#[cfg(feature = "sdl")]
mod window;
#[cfg(feature = "sdl")]
mod audio;

/// Command-line arguments accessible to the program
/// Used by host functions to access command-line args
pub static REST_ARGS: Mutex<Vec<String>> = Mutex::new(vec![]);

// Re-export public API
pub use vm::{VM, Value, Actor};
pub use ast::Program;
pub use parser::{parse_file, parse_str};
pub use alloc::Alloc;

// Wasm-specific exports
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_init() {
    // Set panic hook for better error messages in browser console
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct PlushVM {
    vm: VM,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PlushVM {
    /// Create a new Plush VM from source code
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Result<PlushVM, JsValue> {
        let prog = parse_str(source)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

        let mut prog = prog;
        prog.resolve_syms()
            .map_err(|e| JsValue::from_str(&format!("Symbol resolution error: {}", e)))?;

        let vm = VM::new(prog);
        Ok(PlushVM { vm })
    }

    /// Execute the main function and return the result as a string
    pub fn run(&mut self) -> Result<String, JsValue> {
        let main_fn = self.vm.prog.main_fn;
        let ret = VM::call(&mut self.vm, main_fn, vec![]);

        Ok(format!("{:?}", ret))
    }

    /// Evaluate a Plush expression and return the result
    pub fn eval(&mut self, source: &str) -> Result<String, JsValue> {
        let prog = parse_str(source)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

        let mut prog = prog;
        prog.resolve_syms()
            .map_err(|e| JsValue::from_str(&format!("Symbol resolution error: {}", e)))?;

        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        Ok(format!("{:?}", ret))
    }
}

// Standalone evaluation function for simple use cases
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn plush_eval(source: &str) -> Result<String, JsValue> {
    let prog = parse_str(source)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    let mut prog = prog;
    prog.resolve_syms()
        .map_err(|e| JsValue::from_str(&format!("Symbol resolution error: {}", e)))?;

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let ret = VM::call(&mut vm, main_fn, vec![]);

    Ok(format!("{:?}", ret))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let source = "1 + 2";
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 3),
            _ => panic!("Expected Int64, got {:?}", ret),
        }
    }

    #[test]
    fn test_multiplication_precedence() {
        let source = "1 + 2 * 3";
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 7),
            _ => panic!("Expected Int64(7), got {:?}", ret),
        }
    }

    #[test]
    fn test_function_definition() {
        let source = "let f = |x| x * 2; f(21)";
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 42),
            _ => panic!("Expected Int64(42), got {:?}", ret),
        }
    }

    #[test]
    fn test_factorial() {
        let source = r#"
            let factorial = |n| {
                if (n <= 1) { 1 } else { n * factorial(n - 1) }
            };
            factorial(5)
        "#;
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 120),
            _ => panic!("Expected Int64(120), got {:?}", ret),
        }
    }

    #[test]
    fn test_fibonacci() {
        let source = r#"
            let fib = |n| {
                if (n <= 1) { n } else { fib(n - 1) + fib(n - 2) }
            };
            fib(10)
        "#;
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 55),
            _ => panic!("Expected Int64(55), got {:?}", ret),
        }
    }

    #[test]
    fn test_string_operations() {
        let source = r#"let a = "Hello"; let b = "World"; a + " " + b"#;
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::String(_) => {
                let str_val = ret.unwrap_rust_str();
                assert_eq!(str_val, "Hello World");
            }
            _ => panic!("Expected String, got {:?}", ret),
        }
    }

    #[test]
    fn test_array_creation() {
        let source = "let arr = [1, 2, 3]; arr.length";
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 3),
            _ => panic!("Expected Int64(3), got {:?}", ret),
        }
    }

    #[test]
    fn test_array_indexing() {
        let source = "let arr = [10, 20, 30]; arr[1]";
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 20),
            _ => panic!("Expected Int64(20), got {:?}", ret),
        }
    }

    #[test]
    fn test_closure() {
        let source = r#"
            let make_adder = |x| {
                |y| x + y
            };
            let add5 = make_adder(5);
            add5(10)
        "#;
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 15),
            _ => panic!("Expected Int64(15), got {:?}", ret),
        }
    }

    #[test]
    fn test_object_creation() {
        let source = r#"
            let obj = { x: 10, y: 20 };
            obj.x + obj.y
        "#;
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 30),
            _ => panic!("Expected Int64(30), got {:?}", ret),
        }
    }

    #[test]
    fn test_loop_with_break() {
        let source = r#"
            let sum = 0;
            let i = 0;
            loop {
                if (i >= 5) { break; }
                sum = sum + i;
                i = i + 1;
            }
            sum
        "#;
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 10), // 0+1+2+3+4 = 10
            _ => panic!("Expected Int64(10), got {:?}", ret),
        }
    }

    #[test]
    fn test_boolean_operations() {
        let source = "let x = true; let y = false; x && y";
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        assert!(matches!(ret, Value::False));
    }

    #[test]
    fn test_comparison() {
        let source = "let x = 10; let y = 20; x < y";
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        assert!(matches!(ret, Value::True));
    }

    #[test]
    fn test_parse_error() {
        let source = "let x = ;"; // Invalid syntax
        let result = parse_str(source);
        assert!(result.is_err());
    }

    #[test]
    fn test_float_operations() {
        let source = "3.14 * 2.0";
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Float64(v) => assert!((v - 6.28).abs() < 0.01),
            _ => panic!("Expected Float64, got {:?}", ret),
        }
    }

    #[test]
    fn test_nested_functions() {
        let source = r#"
            let outer = |x| {
                let inner = |y| x + y;
                inner(10) + inner(20)
            };
            outer(5)
        "#;
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);

        match ret {
            Value::Int64(v) => assert_eq!(v, 40), // (5+10) + (5+20) = 40
            _ => panic!("Expected Int64(40), got {:?}", ret),
        }
    }
}
