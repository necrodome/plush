# Plush Tests

This directory contains tests for the Plush programming language and virtual machine.

## Test Types

### 1. Plush Script Tests (`.psh` files)

These are Plush programs used for testing various language features. They are executed by the test suite to verify correct behavior.

**Examples:**
- `fact.psh` - Factorial calculation
- `closures.psh` - Closure functionality
- `actor_send.psh` - Actor message passing
- `int_arith.psh` - Integer arithmetic
- `floats.psh` - Floating point operations
- `bytearray.psh` - Byte array operations

**Running script tests:**
```bash
# Run with the Plush VM
cargo run tests/fact.psh

# Or directly with the built binary
./target/release/plush tests/fact.psh
```

### 2. Rust Integration Tests (`.rs` files)

Integration tests written in Rust that test the library API.

**Files:**
- `lib_api.rs` - Tests for the public library API (works without SDL)

**Running integration tests:**
```bash
# Run all integration tests
cargo test --test lib_api

# Run with no default features (tests Wasm compatibility)
cargo test --test lib_api --no-default-features

# Run a specific test
cargo test --test lib_api test_factorial
```

### 3. Unit Tests

Unit tests are embedded in the source files (in `src/`):
- `src/lib.rs` - Core library tests
- `src/vm.rs` - VM-specific tests
- Other source files may contain module-level tests

**Running unit tests:**
```bash
# Run all unit tests
cargo test

# Run only library unit tests (no SDL)
cargo test --lib --no-default-features

# Run tests with backtraces
RUST_BACKTRACE=1 cargo test
```

## CI/CD Testing

### GitHub Actions Workflows

**`.github/workflows/test.yml`** - Native build testing
- Builds with SDL2
- Runs all tests
- Tests command-line interface
- Runs benchmarks
- **New:** Also tests library build without SDL for Wasm compatibility

**`.github/workflows/wasm.yml`** - WebAssembly testing
- Builds library without SDL
- Builds for wasm32-unknown-unknown target
- Tests with wasm-pack (web, nodejs, bundler targets)
- Runs Node.js integration tests
- Checks Wasm file size
- Tests build script

## Running Different Test Configurations

### Native Build (with SDL)
```bash
# Full test suite
cargo test

# Release mode
cargo test --release
```

### Library Only (without SDL, Wasm-compatible)
```bash
# Build library
cargo build --lib --no-default-features

# Test library
cargo test --lib --no-default-features

# Test integration tests
cargo test --no-default-features --test lib_api
```

### WebAssembly
```bash
# Install wasm32 target
rustup target add wasm32-unknown-unknown

# Build for Wasm
cargo build --target wasm32-unknown-unknown --lib --no-default-features

# Or use wasm-pack
wasm-pack build --target web --no-default-features

# Test in Node.js (requires building with wasm-pack first)
# See .github/workflows/wasm.yml for the test script
```

## Test Coverage

### Core Language Features (tested without SDL)
✓ Arithmetic operations
✓ Function definitions and calls
✓ Recursion (factorial, fibonacci)
✓ Closures and nested functions
✓ String operations
✓ Array operations (creation, indexing, push, length)
✓ Object creation and property access
✓ Loops and control flow
✓ Boolean logic and comparisons
✓ Float operations
✓ Error handling

### SDL-Dependent Features (tested with SDL)
✓ Window creation and graphics
✓ Audio input/output
✓ Event handling
✓ Actor-based concurrency with SDL callbacks

### WebAssembly-Specific
✓ wasm-pack builds (web, nodejs, bundler)
✓ JavaScript API (PlushVM class, plush_eval function)
✓ Node.js integration
✓ Error propagation to JavaScript
✓ Binary size optimization

## Adding New Tests

### Adding a Plush Script Test

1. Create a `.psh` file in this directory
2. Add test execution to `.github/workflows/test.yml` if needed
3. Document expected behavior

### Adding a Rust Integration Test

1. Create or modify `.rs` files in `tests/`
2. Use the public API from `plush` crate:
   ```rust
   use plush::{parse_str, VM, Value};

   #[test]
   fn my_test() {
       let prog = parse_str("1 + 1").unwrap();
       let mut prog = prog;
       prog.resolve_syms().unwrap();
       let mut vm = VM::new(prog);
       let result = VM::call(&mut vm, vm.prog.main_fn, vec![]);
       // Assert result
   }
   ```

### Adding a Unit Test

Add tests to the relevant source file in `src/`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code
    }
}
```

## Continuous Testing

Tests run automatically on:
- Every push to main/master branches
- Every pull request
- Changes to source files, Cargo.toml, or workflow files

## Troubleshooting Tests

### SDL2 Not Found
```
error: failed to run custom build command for `sdl2-sys`
```
**Solution:** Install SDL2 development libraries
- Ubuntu/Debian: `sudo apt-get install libsdl2-dev`
- macOS: `brew install sdl2`
- Windows: Copy `SDL2.dll` to project root

### Wasm Build Fails
```
error: target 'wasm32-unknown-unknown' not found
```
**Solution:** Install the Wasm target
```bash
rustup target add wasm32-unknown-unknown
```

### Test Timeout
Some recursive tests may be slow in debug mode.
**Solution:** Run in release mode: `cargo test --release`

### Specific Test Failing
```bash
# Run with backtrace
RUST_BACKTRACE=full cargo test failing_test_name

# Run with output
cargo test failing_test_name -- --nocapture

# Run single test
cargo test --test lib_api test_factorial -- --exact
```
