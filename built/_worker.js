import wasmModule from './serverless_functions_test/serverless_functions_test_bg.wasm'

export default {
  async fetch (request, env) {
    console.debug('wasmModule', wasmModule)
    const moduleInstance = await WebAssembly.instantiate(wasmModule)

    console.debug('moduleInstance', moduleInstance)

    const url = new URL(request.url)
    if (url.pathname.startsWith('/api/')) {
      // TODO: Add your custom /api/* logic here.
      return new Response('Ok')
    }

    // Otherwise, serve the static assets.
    // Without this, the Worker will error and no assets will be served.
    return env.ASSETS.fetch(request)
  }
}
