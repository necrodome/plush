# WebAssembly (Wasm) Target for Plush

This document describes how to build and use Plush as a WebAssembly module that can run in web browsers or other Wasm runtimes.

## Overview

Plush now supports compilation to WebAssembly, allowing you to run Plush programs in web browsers without needing native compilation for each platform. The Wasm target exposes a JavaScript-friendly API through `wasm-bindgen`.

## Architecture

The Wasm build:
- **Includes**: Core VM, parser, bytecode compiler, runtime, and actors
- **Excludes**: SDL2-dependent features (audio I/O, window/graphics)
- **Exposes**: JavaScript API for parsing and executing Plush code

### Limitations

The following features are **not available** in the Wasm build:
- `$window_create()` - Graphics/windowing (requires SDL2)
- `$window_draw_frame()` - Drawing operations
- `$audio_open_input()` - Audio input (microphone access)
- `$audio_open_output()` - Audio output playback
- `$audio_read_samples()` - Reading audio samples
- `$audio_write_samples()` - Writing audio samples

Plush programs using these features will fail at runtime with "unknown host constant" errors in Wasm.

### Available Features in Wasm

The following work normally:
- All core language features (variables, functions, objects, arrays)
- Actors and message passing
- Math operations
- String manipulation
- `print()` and `println()` (outputs to browser console)
- `actor_spawn()`, `actor_send()`, `actor_recv()` - Concurrency
- All language control flow

## Prerequisites

1. **Rust toolchain** with wasm32 target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **wasm-pack** (recommended) or **wasm-bindgen-cli**:
   ```bash
   cargo install wasm-pack
   # OR
   cargo install wasm-bindgen-cli
   ```

## Building for WebAssembly

### Option 1: Using wasm-pack (Recommended)

```bash
# Build for web (generates ES modules)
wasm-pack build --target web --no-default-features

# Build for Node.js
wasm-pack build --target nodejs --no-default-features

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler --no-default-features

# Release build (optimized)
wasm-pack build --target web --release --no-default-features
```

The output will be in `pkg/`:
- `plush_bg.wasm` - The WebAssembly binary
- `plush.js` - JavaScript bindings
- `plush.d.ts` - TypeScript definitions
- `package.json` - NPM package metadata

### Option 2: Using cargo directly

```bash
# Build the wasm file
cargo build --target wasm32-unknown-unknown --no-default-features --release

# Generate JS bindings
wasm-bindgen target/wasm32-unknown-unknown/release/plush.wasm \
  --out-dir pkg \
  --target web
```

## JavaScript API

### Loading the Module

```javascript
// For web target
import init, { PlushVM, plush_eval } from './pkg/plush.js';

await init(); // Initialize the Wasm module

// Now you can use the API
```

### API Reference

#### `plush_eval(source: string): string`

Evaluate a Plush expression and return the result as a string.

```javascript
const result = plush_eval("1 + 2 * 3");
console.log(result); // "Int64(7)"

const hello = plush_eval('let x = "Hello, "; let y = "Wasm!"; $println(x + y)');
```

#### `PlushVM` Class

For more control, use the `PlushVM` class:

```javascript
// Create a VM instance with source code
const vm = new PlushVM(`
  let factorial = fn(n) {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
  };

  factorial(10)
`);

// Run the program
const result = vm.run();
console.log(result); // "Int64(3628800)"

// Evaluate additional code
const result2 = vm.eval("2 + 2");
console.log(result2); // "Int64(4)"
```

## Usage Examples

### Example 1: Simple HTML Page

```html
<!DOCTYPE html>
<html>
<head>
    <title>Plush REPL</title>
</head>
<body>
    <h1>Plush WebAssembly REPL</h1>
    <textarea id="code" rows="10" cols="80">
let factorial = fn(n) {
  if n <= 1 { 1 } else { n * factorial(n - 1) }
};

factorial(10)
    </textarea>
    <br>
    <button onclick="runCode()">Run</button>
    <pre id="output"></pre>

    <script type="module">
        import init, { plush_eval } from './pkg/plush.js';

        await init();

        window.runCode = function() {
            const code = document.getElementById('code').value;
            try {
                const result = plush_eval(code);
                document.getElementById('output').textContent = result;
            } catch (e) {
                document.getElementById('output').textContent = 'Error: ' + e;
            }
        };
    </script>
</body>
</html>
```

### Example 2: Node.js Usage

```javascript
// For Node.js, use the nodejs target
const { PlushVM, plush_eval } = require('./pkg/plush.js');

// Simple evaluation
console.log(plush_eval("1 + 2 + 3")); // Int64(6)

// Using VM for multiple evaluations
const vm = new PlushVM("let x = 42; x");
console.log(vm.run()); // Int64(42)
```

### Example 3: Actor-based Concurrency

```javascript
const code = `
  let worker = fn() {
    loop {
      let msg = $actor_recv();
      $println("Worker received: " + msg);
    }
  };

  let worker_id = $actor_spawn(worker);
  $actor_send(worker_id, "Hello from main!");
  $actor_send(worker_id, "Another message");
`;

const vm = new PlushVM(code);
vm.run();
// Check browser console for output
```

## Building and Serving Locally

1. Build the Wasm module:
   ```bash
   wasm-pack build --target web --no-default-features
   ```

2. Serve with a local HTTP server (required for ES modules):
   ```bash
   # Python 3
   python3 -m http.server 8000

   # Or use any other static file server
   ```

3. Open `http://localhost:8000/your-page.html`

## Optimization

### Size Optimization

To reduce the Wasm binary size:

```bash
# 1. Build with optimizations
wasm-pack build --target web --release --no-default-features

# 2. Use wasm-opt (from binaryen)
wasm-opt -Oz -o pkg/plush_bg_opt.wasm pkg/plush_bg.wasm

# 3. Enable link-time optimization in Cargo.toml (already configured):
# [profile.release]
# lto = true
# opt-level = 3
# codegen-units = 1
```

Expected sizes:
- Debug build: ~1-2 MB
- Release build: ~200-500 KB
- Release + wasm-opt: ~150-300 KB
- Gzipped: ~50-100 KB

## Troubleshooting

### "Unknown host constant" errors

If you see errors like `unknown host constant 'window_create'`, the Plush code is trying to use SDL2-dependent features. These are not available in Wasm builds.

**Solution**: Modify your Plush code to avoid graphics/audio features, or use conditional compilation if supported.

### CORS errors in browser

When loading `.wasm` files, browsers require proper CORS headers.

**Solution**: Use a proper HTTP server (not `file://` URLs). The Python HTTP server works well for local development.

### Module initialization fails

**Solution**: Make sure to call `await init()` before using any Plush functions.

## Performance

The Wasm build provides good performance for:
- Compute-intensive tasks (math, algorithms)
- String processing
- Actor-based concurrent programs

Performance notes:
- Wasm is typically 1.5-2x slower than native code
- Actor message passing has some overhead due to JavaScript interop
- String operations are efficient (no unnecessary copying)

## Future Enhancements

Potential future additions:
- Web Audio API integration (to replace SDL2 audio)
- Canvas API integration (to replace SDL2 graphics)
- WebGL support for 3D graphics
- File system access through browser APIs
- WebSocket support for networking

## License

Same as the Plush project.
