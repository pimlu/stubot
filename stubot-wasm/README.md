## What's this?
This builds wasm bindings into `./pkg`.

First, idk how to conditionally do features/optimizations, so comment or uncomment `[profile.release]` optimizations in `/Cargo.toml`.

Run one of these two:
`wasm-pack build -t web --dev`
`wasm-pack build -t web --release`
