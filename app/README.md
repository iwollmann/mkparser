# changelog-builder

You can access the application here: [ChangeLog Builder]()

A tool to automatically generate pretty changelog for share on #checkout-web-changelog channel in slack
<img width="817" alt="image" src="">

## Quick start

Follow [Rust](https://www.rust-lang.org/en-US/install.html) installation instructions.

To build the WASM based [yew](https://yew.rs/) UI, further wasm tooling is required

```
$ rustup target add wasm32-unknown-unknown
$ cargo install --locked trunk
$ cargo install wasm-bindgen-cli
```

## Development

For development, start the web server with

```
$ RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve
```

This should make the UI available at 0.0.0.0:8080 with hot reload on code changes.

To change the default port, use

```
$ RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve --port=9090
```

## Deployment

We need to compile the tailwindcss file using their cli and change the asset link manually in the `index.html` file:

```
<!-- THIS IS FOR PRODUCTION -->
<link data-trunk href="./tailwind.css" rel="css" />
<!-- THIS IS FOR DEVELOPMENT PURPOSES -->
<!-- <script src="https://cdn.tailwindcss.com"></script> -->
```

Generate the `tailwind.css` file with the CLI

`NODE_ENV=production npx tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify

After that we can generate the build:

```
$ RUSTFLAGS=--cfg=web_sys_unstable_apis trunk build
```

## TODO

- [ ] Validations
- [ ] Loadings
- [ ] Css
