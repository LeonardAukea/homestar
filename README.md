<div align="center">
  <a href="https://github.com/ipvm-wg/homestar" target="_blank">
    <img src="https://raw.githubusercontent.com/ipvm-wg/homestar/main/assets/mascot_full_transparent.png" alt="Homestar logo" width="400"></img>
  </a>

  <h1 align="center">Homestar</h1>

  <p>
    <a href="https://crates.io/crates/homestar-invocation">
      <img src="https://img.shields.io/crates/v/homestar-invocation?label=crates" alt="Crate">
    </a>
    <a href="https://crates.io/crates/homestar-runtime">
      <img src="https://img.shields.io/crates/v/homestar-runtime?label=crates" alt="Crate">
    </a>
    <a href="https://crates.io/crates/homestar-wasm">
      <img src="https://img.shields.io/crates/v/homestar-wasm?label=crates" alt="Crate">
    </a>
    <a href="https://crates.io/crates/homestar-workflow">
      <img src="https://img.shields.io/crates/v/homestar-workflow?label=crates" alt="Crate">
    </a>
    <a href="https://codecov.io/gh/ipvm-wg/homestar">
      <img src="https://codecov.io/gh/ipvm-wg/homestar/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/ipvm-wg/homestar/actions/workflows/tests_and_checks.yml">
      <img src="https://github.com/ipvm-wg/homestar/actions/workflows/tests_and_checks.yml/badge.svg" alt="Tests and Checks Status">
    </a>
    <a href="https://github.com/ipvm-wg/homestar/actions/workflows/docker.yml">
      <img src="https://github.com/ipvm-wg/homestar/actions/workflows/docker.yml/badge.svg" alt="Build Docker Status">
    </a>
    <a href="https://github.com/ipvm-wg/homestar/actions/workflows/audit.yml">
      <img src="https://github.com/ipvm-wg/homestar/actions/workflows/audit.yml/badge.svg" alt="Cargo Audit Status">
    </a>
    <a href="https://github.com/ipvm-wg/homestar/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
    </a>
    <a href="https://docs.rs/homestar-invocation">
      <img src="https://img.shields.io/static/v1?label=Docs&message=invocation.docs.rs&color=pink" alt="Docs">
    </a>
    <a href="https://docs.rs/homestar-runtime">
      <img src="https://img.shields.io/static/v1?label=Docs&message=runtime.docs.rs&color=pink" alt="Docs">
    </a>
    <a href="https://docs.rs/homestar-wasm">
      <img src="https://img.shields.io/static/v1?label=Docs&message=wasm.docs.rs&color=pink" alt="Docs">
    </a>
    <a href="https://docs.rs/homestar-workflow">
      <img src="https://img.shields.io/static/v1?label=Docs&message=workflow.docs.rs&color=pink" alt="Docs">
    </a>
    <a href="https://fission.codes/discord">
      <img src="https://img.shields.io/static/v1?label=Discord&message=join%20us!&color=mediumslateblue" alt="Discord">
    </a>
  </p>
</div>

##

## Outline

- [Quickstart](#quickstart)
- [Packages](#packages)
- [Running Examples](#running-examples)
- [Workspace](#workspace)
- [Contributing](#contributing)
- [Releases and Builds](#releases-and-builds)
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)

## Quickstart

If you're looking to help develop `Homestar`, please dive right into our
[development](./DEVELOPMENT.md) guide.

Otherwise, the easiest way to get started and see `Homestar` in action is to
follow-along with our [examples](./examples). To start, try out our
image-processing [websocket relay](./examples/websocket-relay) example, which
integrates `Homestar` with a browser application to run a
statically-configured workflow. The associated `README.md` walks through
what to install (i.e. `rust`, `node/npm`, `ipfs`), what commands
to run, and embeds a video demonstrating its usage.

Throughout the `Homestar` ecosystem and documentation, we'll draw a distinction
between the [host runtime][host-runtime] and the support for different
[guest languages and bindings][guest].

If you're mainly interested in learning how to write and build-out Wasm
components (currently focused on authoring in Rust), please jump into
our [`homestar-functions`](./homestar-functions) directory and check out
our examples there.

## Packages

Each `Homestar` release will also build packages for distribution across
different platforms.

- [homebrew][homebrew]: `brew install fission-codes/fission/homestar`
  This includes `ipfs` in the install by default.
- [npm](https://www.npmjs.com/package/homestar-runtime): `npm install homestar-runtime -g` Wraps the `homestar-runtime` binary in a node script.

## Running Examples

All [examples](./examples) contain instructions for running
them, including what to install and how to run them. Please clone this repo,
and get started!

Each example showcases something specific and interesting about `Homestar`
as a system.

Our current list includes:

- [websocket relay](./examples/websocket-relay/README.md) - An example
  (browser-based) application that connects to the `homestar-runtime` over a
  WebSocket connection in order to run a couple static Wasm-based, image
  processing workflows that chain inputs and outputs.

## Workspace

This repository is comprised of a few library packages and a library/binary that
represents the `Homestar` runtime. We recommend diving into each package's own
`README.md` for more information when available.

### Core Crates

- [homestar-invocation](./homestar-invocation)

  The *invocation* library implements much of the [Ucan Invocation][ucan-invocation]
  specification and is used as the foundation for other packages in this
  `workspace` and within the runtime engine.

- [homestar-wasm](./homestar-wasm)

  This *wasm* library manages the [wasmtime][wasmtime] runtime, provides the
  [IPLD][ipld] to/from [WIT][wit] interpreter/translation-layer, and implements
  the input interface for working with Ipvm's standard Wasm tasks.

  You can find the spec for translating between IPLD and WIT runtime values
  based on WIT types [here](./homestar-wasm/README.md#interpreting-between-ipld-and-wit).

- [homestar-workflow](./homestar-workflow)

  The *workflow* library implements workflow-centric [Ipvm features][ipvm-spec]
  and is used as the foundation for other packages in this `workspace` and
  within the runtime engine.

### Runtime Crate

- [homestar-runtime](./homestar-runtime)

  The *runtime* is responsible for bootstrapping and running nodes, scheduling
  and executing workflows as well as tasks within workflows, handling retries
  and failure modes, etc.

### Non-published Crates

- [homestar-functions/*](./homestar-functions)

  `homestar-functions` is a directory of helper, test, and example crates for
  writing and compiling [Wasm component][wasm-component] modules using
  [wit-bindgen][wit-bindgen].

- [homestar-schemas](./homestar-schemas)

`homestar-schemas` is a crate for generating OpenRPC docs and JSON Schemas that document the [homestar-runtime](./homestar-runtime) JSON-RPC API, workflows, and receipts.

- [examples/*](./examples)

  `examples` contains examples and demos showcasing `Homestar` packages
  and the `Homestar` runtime. Each example is set up as its own crate,
  demonstrating the necessary dependencies and setup(s).

## Contributing

:balloon: We're thankful for any feedback and help in improving our project!
We have a focused [development](./DEVELOPMENT.md) guide, as well as a
more general [contributing](./CONTRIBUTING.md) guide to help you get involved.
We always adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Releases and Builds

### Crates, Tags, and GitHub Releases

Homestar uses [release-plz][release-plz] to publish [crates][rel-crates],
[tags][rel-tags], changelogs, and [GitHub Releases][rel-gh]. Upon merging,
a `release-plz` bot PR, four crates are continuously published,
**all tied to the same cargo version currently** (though this may change in the
future):

- [homestar-runtime][crate-runtime]
- [homestar-invocation][crate-invocation]
- [homestar-workflow][crate-workflow]
- [homestar-wasm][crate-wasm]

### Build Targets

Every [GitHub release of the homestar-runtime][rel-latest] contains build assets
for running the `homestar-runtime` on different target architectures, as well as
[DEB][deb] and [RPM][rpm] packages (tagged with the architectured they were
compiled for). Our homebrew package for the runtime is also tied to releases
and can be installed with `brew install fission-codes/fission/homestar`.

We also leverage [cross][cross-rs] for [locally cross-compiling](./Cross.toml)
to varying Linux and Apple target platforms.

### NPM Packages

We also release some of our cross-compiled runtime binaries as
[npm binary packages](./homestar-runtime/npm/README.md):

- [homestar-runtime](https://www.npmjs.com/package/homestar-runtime) - This is
  the main package that installs the os specific binary package and runs it.
- [homestar-darwin-arm64](https://www.npmjs.com/package/homestar-darwin-arm64)
- [homestar-darwin-x64](https://www.npmjs.com/package/homestar-darwin-x64)
- [homestar-linux-arm64](https://www.npmjs.com/package/homestar-linux-arm64)
- [homestar-linux-x64](https://www.npmjs.com/package/homestar-linux-x64)
- [homestar-windows-x64](https://www.npmjs.com/package/homestar-windows-x64)

## Getting Help

For usage questions, usecases, or issues reach out to us in our [Discord channel](https://fission.codes/discord).

We would be happy to try to answer your question or try opening a new issue on GitHub.

## External Resources

- [What Is An IPVM][ipvm-wg]
- [IPVM: High-Level Spec][ipvm-spec]
- [Contributing Research][research]
- [Seamless Services for an Open World][seamless-services] by Brooklyn Zelenka
- [Foundations for Open-World Compute][foundations-for-openworld-compute] by Zeeshan Lakhani
- [IPVM: The Long-Fabled Execution Layer][cod-ipvm] by Brooklyn Zelenka
- [IPVM - IPFS and WASM][ipfs-thing-ipvm] by Brooklyn Zelenka
- [Breaking Down the Interplanetary Virtual Machine][blog-1]
- [Ucan Invocation Spec][ucan-invocation]

## License

This project is licensed under the [Apache License 2.0](./LICENSE), or
[http://www.apache.org/licenses/LICENSE-2.0][apache].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[apache]: https://www.apache.org/licenses/LICENSE-2.0
[blog-1]: https://fission.codes/blog/ipfs-thing-breaking-down-ipvm/
[cod-ipvm]: https://www.youtube.com/watch?v=3y1RB8wt_YY
[crate-runtime]: https://crates.io/crates/homestar-runtime
[crate-invocation]: https://crates.io/crates/homestar-invocation
[crate-workflow]: https://crates.io/crates/homestar-workflow
[crate-wasm]: https://crates.io/crates/homestar-wasm
[cross-rs]: https://github.com/cross-rs/cross
[deb]: https://www.debian.org/doc/manuals/debian-faq/pkg-basics.en.html
[demo-1]: https://www.loom.com/share/3204037368fe426ba3b4c952b0691c5c
[foundations-for-openworld-compute]: https://youtu.be/dRz5mau6fsY
[guest]: https://github.com/bytecodealliance/wit-bindgen#supported-guest-languages
[homebrew]: https://brew.sh/
[host-runtime]: https://github.com/bytecodealliance/wit-bindgen#host-runtimes-for-components
[ipfs-thing-ipvm]: https://www.youtube.com/watch?v=rzJWk1nlYvs
[ipld]: https://ipld.io/
[ipvm-spec]: https://github.com/ipvm-wg/spec
[ipvm-wg]: https://github.com/ipvm-wg
[ipvm-workflow-spec]: https://github.com/ipvm-wg/workflow
[mit]: http://opensource.org/licenses/MIT
[rel-crates]: https://crates.io/search?q=homestar
[rel-gh]: https://github.com/ipvm-wg/homestar/releases
[rel-latest]: https://github.com/ipvm-wg/homestar/releases/latest
[rel-tags]: https://github.com/ipvm-wg/homestar/tags
[release-plz]: https://release-plz.ieni.dev/
[rpm]: https://rpm.org/
[research]: https://github.com/ipvm-wg/research
[seamless-services]: https://youtu.be/Kr3B3sXh_VA
[ucan-invocation]: https://github.com/ucan-wg/invocation
[wasm-component]: https://github.com/WebAssembly/component-model
[wasmtime]: https://github.com/bytecodealliance/wasmtime
[wit]: https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md
[wit-bindgen]: https://github.com/bytecodealliance/wit-bindgen
