# Yab

built with [Yew](https://github.com/yewstack/yew) and [Axum](https://github.com/tokio-rs/axum).

## Build

### Frontend

```shell
cd frontend
wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js
cd ../
rm -rf static
mkdir static
cp -r ./frontend/pkg ./static/pkg
cp ./frontend/index.html ./static/index.html
```

Then place `./static` directory along with the backend executable.

### Backend

```shell
cargo build -p backend --release
```
