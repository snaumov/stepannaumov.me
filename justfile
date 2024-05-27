build-tailwind:
    npx tailwindcss -o ./assets/dist/tailwind.css

build-wasm-animation:
    cargo build -p wasm_animation --release --target wasm32-unknown-unknown
    wasm-bindgen --out-name wasm_animation --out-dir assets/wasm --target web target/wasm32-unknown-unknown/release/wasm_animation.wasm