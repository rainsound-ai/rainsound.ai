let K = 0,
  M = `string`,
  L = 1,
  O = `Object`,
  I = `utf-8`,
  G = null,
  H = `undefined`,
  P = 4,
  N = `function`,
  E = Array,
  J = Error,
  F = undefined
var y = async (a, b) => {
  if (typeof Response === N && a instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === N) {
      try {
        return await WebAssembly.instantiateStreaming(a, b)
      } catch (b) {
        if (a.headers.get(`Content-Type`) != `application/wasm`) {
          console.warn(
            `\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,
            b
          )
        } else {
          throw b
        }
      }
    }
    const c = await a.arrayBuffer()
    return await WebAssembly.instantiate(c, b)
  } else {
    const c = await WebAssembly.instantiate(a, b)
    if (c instanceof WebAssembly.Instance) {
      return { instance: c, module: a }
    } else {
      return c
    }
  }
}
var l = a => {
  const b = typeof a
  if (b == `number` || b == `boolean` || a == G) {
    return `${a}`
  }
  if (b == M) {
    return `"${a}"`
  }
  if (b == `symbol`) {
    const b = a.description
    if (b == G) {
      return `Symbol`
    } else {
      return `Symbol(${b})`
    }
  }
  if (b == N) {
    const b = a.name
    if (typeof b == M && b.length > K) {
      return `Function(${b})`
    } else {
      return `Function`
    }
  }
  if (E.isArray(a)) {
    const b = a.length
    let c = `[`
    if (b > K) {
      c += l(a[K])
    }
    for (let d = L; d < b; d++) {
      c += `, ` + l(a[d])
    }
    c += `]`
    return c
  }
  const c = /\[object ([^\]]+)\]/.exec(toString.call(a))
  let d
  if (c.length > L) {
    d = c[L]
  } else {
    return toString.call(a)
  }
  if (d == O) {
    try {
      return `Object(` + JSON.stringify(a) + `)`
    } catch (a) {
      return O
    }
  }
  if (a instanceof J) {
    return `${a.name}: ${a.message}\n${a.stack}`
  }
  return d
}
var A = (a, b) => {}
var v = (a, b) => {
  a = a >>> K
  const c = u()
  const d = c.subarray(a / P, a / P + b)
  const e = []
  for (let a = K; a < d.length; a++) {
    e.push(f(d[a]))
  }
  return e
}
var k = a => {
  if (d === b.length) b.push(b.length + L)
  const c = d
  d = b[c]
  b[c] = a
  return c
}
var f = a => {
  const b = c(a)
  e(a)
  return b
}
function x (b, c) {
  try {
    return b.apply(this, c)
  } catch (b) {
    a.__wbindgen_exn_store(k(b))
  }
}
var r = () => {
  if (q === G || q.byteLength === K) {
    q = new Int32Array(a.memory.buffer)
  }
  return q
}
var w = a => a === F || a === G
var c = a => b[a]
var D = async b => {
  if (a !== F) return a
  if (typeof b === H) {
    b = new URL(`browser_bg.wasm`, import.meta.url)
  }
  const c = z()
  if (
    typeof b === M ||
    (typeof Request === N && b instanceof Request) ||
    (typeof URL === N && b instanceof URL)
  ) {
    b = fetch(b)
  }
  A(c)
  const { instance: d, module: e } = await y(await b, c)
  return B(d, e)
}
var z = () => {
  const b = {}
  b.wbg = {}
  b.wbg.__wbindgen_object_drop_ref = a => {
    f(a)
  }
  b.wbg.__wbindgen_string_new = (a, b) => {
    const c = j(a, b)
    return k(c)
  }
  b.wbg.__wbg_debug_783a3d4910bc24c7 = (b, c) => {
    var d = v(b, c).slice()
    a.__wbindgen_free(b, c * P)
    console.debug(...d)
  }
  b.wbg.__wbg_body_674aec4c1c0910cd = a => {
    const b = c(a).body
    return w(b) ? K : k(b)
  }
  b.wbg.__wbg_instanceof_Window_9029196b662bc42a = a => {
    let b
    try {
      b = c(a) instanceof Window
    } catch {
      b = !1
    }
    const d = b
    return d
  }
  b.wbg.__wbg_document_f7ace2b956f30a4f = a => {
    const b = c(a).document
    return w(b) ? K : k(b)
  }
  b.wbg.__wbg_insertAdjacentHTML_04bc2b21165e1256 = function () {
    return x((a, b, d, e, f) => {
      c(a).insertAdjacentHTML(j(b, d), j(e, f))
    }, arguments)
  }
  b.wbg.__wbindgen_object_clone_ref = a => {
    const b = c(a)
    return k(b)
  }
  b.wbg.__wbg_newnoargs_581967eacc0e2604 = (a, b) => {
    const c = new Function(j(a, b))
    return k(c)
  }
  b.wbg.__wbg_call_cb65541d95d71282 = function () {
    return x((a, b) => {
      const d = c(a).call(c(b))
      return k(d)
    }, arguments)
  }
  b.wbg.__wbg_self_1ff1d729e9aae938 = function () {
    return x(() => {
      const a = self.self
      return k(a)
    }, arguments)
  }
  b.wbg.__wbg_window_5f4faef6c12b79ec = function () {
    return x(() => {
      const a = window.window
      return k(a)
    }, arguments)
  }
  b.wbg.__wbg_globalThis_1d39714405582d3c = function () {
    return x(() => {
      const a = globalThis.globalThis
      return k(a)
    }, arguments)
  }
  b.wbg.__wbg_global_651f05c6a0944d1c = function () {
    return x(() => {
      const a = global.global
      return k(a)
    }, arguments)
  }
  b.wbg.__wbindgen_is_undefined = a => {
    const b = c(a) === F
    return b
  }
  b.wbg.__wbindgen_debug_string = (b, d) => {
    const e = l(c(d))
    const f = p(e, a.__wbindgen_malloc, a.__wbindgen_realloc)
    const g = m
    r()[b / P + L] = g
    r()[b / P + K] = f
  }
  b.wbg.__wbindgen_throw = (a, b) => {
    throw new J(j(a, b))
  }
  return b
}
var u = () => {
  if (t === G || t.byteLength === K) {
    t = new Uint32Array(a.memory.buffer)
  }
  return t
}
var e = a => {
  if (a < 132) return
  b[a] = d
  d = a
}
var C = b => {
  if (a !== F) return a
  const c = z()
  A(c)
  if (!(b instanceof WebAssembly.Module)) {
    b = new WebAssembly.Module(b)
  }
  const d = new WebAssembly.Instance(b, c)
  return B(d, b)
}
var p = (a, b, c) => {
  if (c === F) {
    const c = n.encode(a)
    const d = b(c.length, L) >>> K
    i()
      .subarray(d, d + c.length)
      .set(c)
    m = c.length
    return d
  }
  let d = a.length
  let e = b(d, L) >>> K
  const f = i()
  let g = K
  for (; g < d; g++) {
    const b = a.charCodeAt(g)
    if (b > 127) break
    f[e + g] = b
  }
  if (g !== d) {
    if (g !== K) {
      a = a.slice(g)
    }
    e = c(e, d, (d = g + a.length * 3), L) >>> K
    const b = i().subarray(e + g, e + d)
    const f = o(a, b)
    g += f.written
  }
  m = g
  return e
}
var i = () => {
  if (h === G || h.byteLength === K) {
    h = new Uint8Array(a.memory.buffer)
  }
  return h
}
var j = (a, b) => {
  a = a >>> K
  return g.decode(i().subarray(a, a + b))
}
var B = (b, c) => {
  a = b.exports
  D.__wbindgen_wasm_module = c
  q = G
  t = G
  h = G
  a.__wbindgen_start()
  return a
}
let a
const b = new E(128).fill(F)
b.push(F, G, !0, !1)
let d = b.length
const g =
  typeof TextDecoder !== H
    ? new TextDecoder(I, { ignoreBOM: !0, fatal: !0 })
    : {
        decode: () => {
          throw J(`TextDecoder not available`)
        }
      }
if (typeof TextDecoder !== H) {
  g.decode()
}
let h = G
let m = K
const n =
  typeof TextEncoder !== H
    ? new TextEncoder(I)
    : {
        encode: () => {
          throw J(`TextEncoder not available`)
        }
      }
const o =
  typeof n.encodeInto === N
    ? (a, b) => n.encodeInto(a, b)
    : (a, b) => {
        const c = n.encode(a)
        b.set(c)
        return { read: a.length, written: c.length }
      }
let q = G
function s () {
  a.main()
}
let t = G
export default D
export { s as main, C as initSync }
