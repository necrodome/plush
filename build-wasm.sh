#!/bin/bash
# Build script for Plush WebAssembly target

set -e

echo "Building Plush for WebAssembly..."
echo

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Error: wasm-pack is not installed."
    echo "Install it with: cargo install wasm-pack"
    echo
    echo "Alternatively, you can use cargo + wasm-bindgen directly:"
    echo "  1. cargo install wasm-bindgen-cli"
    echo "  2. rustup target add wasm32-unknown-unknown"
    echo "  3. cargo build --target wasm32-unknown-unknown --no-default-features --release"
    echo "  4. wasm-bindgen target/wasm32-unknown-unknown/release/plush.wasm --out-dir pkg --target web"
    exit 1
fi

# Check if wasm32 target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
    echo
fi

# Parse command line arguments
TARGET="web"
PROFILE="release"

while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            TARGET="$2"
            shift 2
            ;;
        --dev)
            PROFILE="dev"
            shift
            ;;
        --release)
            PROFILE="release"
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo
            echo "Options:"
            echo "  --target <TARGET>   Build target: web, nodejs, bundler, or no-modules (default: web)"
            echo "  --dev               Build in development mode (faster build, larger output)"
            echo "  --release           Build in release mode (slower build, optimized output) [default]"
            echo "  --help              Show this help message"
            echo
            echo "Examples:"
            echo "  $0                          # Build for web (release mode)"
            echo "  $0 --dev                    # Build for web (dev mode)"
            echo "  $0 --target nodejs          # Build for Node.js"
            echo "  $0 --target bundler --dev   # Build for webpack/bundler (dev mode)"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Build with wasm-pack
echo "Building with wasm-pack..."
echo "  Target: $TARGET"
echo "  Profile: $PROFILE"
echo

if [ "$PROFILE" = "release" ]; then
    wasm-pack build --target "$TARGET" --no-default-features --release
else
    wasm-pack build --target "$TARGET" --no-default-features --dev
fi

echo
echo "Build complete!"
echo "Output directory: pkg/"
echo

# Show generated files
echo "Generated files:"
ls -lh pkg/ | grep -v "^d" | awk '{print "  " $9 " (" $5 ")"}'
echo

# Provide usage instructions based on target
case $TARGET in
    web)
        echo "To use in a web page:"
        echo "  <script type=\"module\">"
        echo "    import init, { plush_eval } from './pkg/plush.js';"
        echo "    await init();"
        echo "    console.log(plush_eval('1 + 2'));"
        echo "  </script>"
        ;;
    nodejs)
        echo "To use in Node.js:"
        echo "  const { plush_eval } = require('./pkg/plush.js');"
        echo "  console.log(plush_eval('1 + 2'));"
        ;;
    bundler)
        echo "To use with a bundler (webpack, etc.):"
        echo "  import init, { plush_eval } from 'plush';"
        echo "  await init();"
        echo "  console.log(plush_eval('1 + 2'));"
        ;;
esac

echo
echo "See WASM.md for complete documentation and examples."
