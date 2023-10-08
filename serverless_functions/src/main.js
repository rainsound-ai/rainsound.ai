// This file imports and boots the JS wrapper around our wasm code.
// If we exported specific functions (like `greet`) from our wasm code,
// we could import them like this:
// import init, { greet } from './browser.js'
import wasmMain from './browser.js'

async function main () {
  // Run our wasm code's main function.
  await wasmMain()
}

main()
