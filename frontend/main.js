import init, { run_app } from "./pkg/yab_frontend.js";

async function main() {
  await init("/static/pkg/yab_frontend_bg.wasm");
  run_app();
}

main();
