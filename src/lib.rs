#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_parens)]

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
    fn test_basic_eval() {
        let source = "1 + 2";
        let prog = parse_str(source).unwrap();
        let mut prog = prog;
        prog.resolve_syms().unwrap();
        let mut vm = VM::new(prog);
        let main_fn = vm.prog.main_fn;
        let ret = VM::call(&mut vm, main_fn, vec![]);
        // Basic smoke test
        assert!(matches!(ret, Value::Int64(_)));
    }
}
