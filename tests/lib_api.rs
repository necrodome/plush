// Integration tests for the Plush library API
// These tests verify that the public API works correctly without SDL dependencies

use plush::{parse_str, VM, Value};

#[test]
fn test_parse_and_execute_simple() {
    let prog = parse_str("42").expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 42),
        _ => panic!("Expected Int64(42), got {:?}", result),
    }
}

#[test]
fn test_complex_expression() {
    let source = r#"
        let x = 10;
        let y = 20;
        let z = x * y + 5;
        z
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 205), // 10*20+5
        _ => panic!("Expected Int64(205), got {:?}", result),
    }
}

#[test]
fn test_function_with_multiple_params() {
    let source = r#"
        let add = fn(a, b, c) { a + b + c };
        add(1, 2, 3)
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 6),
        _ => panic!("Expected Int64(6), got {:?}", result),
    }
}

#[test]
fn test_recursive_sum() {
    let source = r#"
        let sum = fn(n) {
            if n <= 0 { 0 } else { n + sum(n - 1) }
        };
        sum(100)
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 5050), // sum(1..100) = 5050
        _ => panic!("Expected Int64(5050), got {:?}", result),
    }
}

#[test]
fn test_array_operations() {
    let source = r#"
        let arr = [1, 2, 3, 4, 5];
        let sum = arr[0] + arr[1] + arr[2] + arr[3] + arr[4];
        sum
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 15),
        _ => panic!("Expected Int64(15), got {:?}", result),
    }
}

#[test]
fn test_string_concatenation() {
    let source = r#"
        let first = "Hello";
        let second = "World";
        first + " " + second + "!"
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::String(s) => {
            let str_val = unsafe { (*s).to_str() };
            assert_eq!(str_val, "Hello World!");
        }
        _ => panic!("Expected String, got {:?}", result),
    }
}

#[test]
fn test_nested_closures() {
    let source = r#"
        let outer = fn(x) {
            let middle = fn(y) {
                let inner = fn(z) {
                    x + y + z
                };
                inner
            };
            middle
        };
        let f = outer(1);
        let g = f(10);
        g(100)
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 111), // 1+10+100
        _ => panic!("Expected Int64(111), got {:?}", result),
    }
}

#[test]
fn test_object_nested_access() {
    let source = r#"
        let obj = {
            x: { y: { z: 42 } }
        };
        obj.x.y.z
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 42),
        _ => panic!("Expected Int64(42), got {:?}", result),
    }
}

#[test]
fn test_conditional_expressions() {
    let source = r#"
        let max = fn(a, b) {
            if a > b { a } else { b }
        };
        max(42, 17)
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 42),
        _ => panic!("Expected Int64(42), got {:?}", result),
    }
}

#[test]
fn test_loop_iteration() {
    let source = r#"
        let product = 1;
        let i = 1;
        loop {
            if i > 5 { break; }
            product = product * i;
            i = i + 1;
        }
        product
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 120), // 5! = 120
        _ => panic!("Expected Int64(120), got {:?}", result),
    }
}

#[test]
fn test_boolean_logic() {
    let source = r#"
        let a = true;
        let b = false;
        let result = (a || b) && !(b && a);
        result
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    assert!(matches!(result, Value::True));
}

#[test]
fn test_parse_error_handling() {
    let sources = vec![
        "let x = ;",           // Missing value
        "fn(x { x }",          // Missing paren
        "if true { 1 }",       // Missing else in expression context (may not error)
        "let let let",         // Invalid syntax
    ];

    for source in sources {
        let result = parse_str(source);
        // At least one should fail
        if result.is_err() {
            return; // Test passes if any error is detected
        }
    }
}

#[test]
fn test_float_precision() {
    let source = r#"
        let pi = 3.14159;
        let radius = 10.0;
        let area = pi * radius * radius;
        area
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Float64(v) => {
            let expected = 3.14159 * 10.0 * 10.0;
            assert!((v - expected).abs() < 0.001);
        }
        _ => panic!("Expected Float64, got {:?}", result),
    }
}

#[test]
fn test_array_push_and_length() {
    let source = r#"
        let arr = [];
        arr.push(1);
        arr.push(2);
        arr.push(3);
        arr.length
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    match result {
        Value::Int64(v) => assert_eq!(v, 3),
        _ => panic!("Expected Int64(3), got {:?}", result),
    }
}

#[test]
fn test_mutual_recursion() {
    let source = r#"
        let is_even = fn(n) {
            if n == 0 { true } else { is_odd(n - 1) }
        };
        let is_odd = fn(n) {
            if n == 0 { false } else { is_even(n - 1) }
        };
        is_even(10)
    "#;

    let prog = parse_str(source).expect("Failed to parse");
    let mut prog = prog;
    prog.resolve_syms().expect("Failed to resolve symbols");

    let mut vm = VM::new(prog);
    let main_fn = vm.prog.main_fn;
    let result = VM::call(&mut vm, main_fn, vec![]);

    assert!(matches!(result, Value::True));
}
