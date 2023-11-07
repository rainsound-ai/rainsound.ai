# Creative Computing Website

**Architecture overview**

Our site is basically a traditional multi-page, vanilla-JS web site, but with some modern niceties.

Most notably, we use Rust across the entire stack:

- At build time to resize images and do other asset pre-processing.

- At runtime on the server to generate HTML, handle form submissions, and do other server-side processing.

- At runtime in the browser to add bits of interactivity where necessary.

We use [Spin](https://www.fermyon.com/spin)'s wasm-powered, quick-booting serverless functions to minimize carbon emissions and maximize performance.

**Design principles**

_Carbon negative_. Reduce emissions as much as possible, automatically measure and offset the rest.

_Minimal bundle size_. Automatically enforced performance budgets. Progressive enhancement. Client-side code is optional.

_Full-stack type safety_. Use Rust's type system to make invalid states unrepresentable, including across the client-server boundary.

_Pit of success_. The easiest way to build our site should also be the best, for the planet and for our visitors.

**Important tools**

[Spin](https://www.fermyon.com/spin) for running our serverless functions in development and production.

[Maud](https://maud.lambda.xyz/) for un-complicated html templates on the server and client. Doesn't blow up our bundle size.

[Tailwind](https://tailwindcss.com/) for CSS.

[Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) for resizing images and other asset pre-processing.

[wasm-pack](https://github.com/rustwasm/wasm-pack) for compiling our client-side Rust to wasm.

**Files and folders**

`/serverless_functions`

This is our most important sub-crate. It's where we server-side render our Maud templates and handle form submissions.

`/browser`

Rust that runs in the browser, analogous to client-side JS.

`/shared`

Types and functions shared between the server and browser.

`/assets`

Provides structs and functions for representing assets like CSS, images, and wasm. Also where we define our global list of assets.

Collects all of our assets and saves them to `/built_assets` at build time.

`/build_browser`

The build script for this sub-crate runs wasm-pack. `lib.rs` exposes static variables containing wasm-pack's output.

`/build_tailwind`

The build script for this sub-crate runs the Tailwind CLI. `lib.rs` exposes a static variable containing the contents of our built CSS file.

`spin.toml`

Our Spin configuration file. Also sets up the static file server that serves our assets.
