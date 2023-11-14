# Creative Computing Website

**Getting started**

Install necessary CLIs like Spin and wasm-pack:

```bash
./scripts setup
```

Start a development server, watching for changes:

```bash
spin watch
```

Build the site and run a preview:

```bash
spin build --up
```

Deploy:

```bash
spin deploy
```

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

This sub-folder contains crates for handling asset preparation like running the Tailwind CLI and resizing images.

`/assets/mod`

Provides structs and functions for representing assets like CSS, images, and wasm. Also where we define our global list of assets.

Collects all of our assets and saves them to `/built_assets` at build time.

`/assets/build_browser`

Exports a `build_browser!` macro that runs wasm-pack on the crate you pass to it, returning a struct that includes the resulting generated JS and wasm bytes.

`/assets/build_tailwind`

Exports a `build_tailwind!` macro that runs the Tailwind CLI. Returns a string with the built CSS.

`/assets/build_images`

Exports a `build_images!` macro that loads images from the given path and generates resized variants.

`/assets/build_images_macro`

Cargo requires that you define procedural macros in their own crate. This is the one for `build_images!`.

`/assets/build_images_runtime`

Runtime types that represent built images. We had to extract this into a separate crate because we wanted to use these types in the `build_images!` macro itself, as well as export them from the `build_images` crate.

(If we put them directly in the `build_images` crate, we'd end up with a circular dependency between `build_images` and `build_images_macro`.)

`spin.toml`

Our Spin configuration file. Also sets up the static file server that serves our assets.
