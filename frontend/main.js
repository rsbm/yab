import init, { run_app } from "./pkg/yab_frontend.js";

async function main() {
  await init("/static/pkg/frontend_bg.wasm");
  run_app();
}

main();
