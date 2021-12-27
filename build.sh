cargo build -p yab-backend
cd frontend
wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js

cd ..
rm -rf static
mkdir static
cp -r ./frontend/pkg ./static/pkg
cp ./frontend/index.html ./static/index.html
cargo run
