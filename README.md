<div align="center" dir="auto">
    <img src="https://via.placeholder.com/50" alt="SVGO Rust Logo"/>
</div>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">SvgOptimizer</h1>

<p align="center">
  <a href="https://crates.io/crates/svgo" target="_blank"><img src="https://img.shields.io/crates/v/svgo"/></a>
  <a href="https://docs.rs/svgo" target="_blank"><img src="https://img.shields.io/docsrs/svgo/0.1.0"/></a>
</p>

<p align="center">
    SVG Optimizer for Rust & WebAssembly (WASM)
</p>

<p align="center" dir="auto">
    <img src="assets/early_development.svg" alt="Early development notice"/>
</p>

## Motivation

Have a SVGO Rust implementation to optimize SVG files, which also exposes a WebAssembly (WASM) interface
so its functionality can be used in the browser.

## Features

The following list provides an overview of the features that are planned to be
implemented as well as the ones that are already implemented.

Feel free to open an issue if you want to request a feature or if you want to
contribute to the project by implementing any of these features.

### Behavior

- [ ] Multipass

### Source Code

- [ ] Prettify
- [ ] Uglify

### General

- [ ] Clean up Attribute Whitespace
- [ ] Clean IDs
- [ ] Round/Rewrite Number Lists
- [ ] Round/Rewrite Numbers
- [ ] Collapse useless groups
- [ ] Minify Colors
- [ ] Round/Rewrite Paths
- [ ] Shapes to (smaller) Paths
- [ ] Style to Attributes
- [ ] Round/Rewrite Transforms
- [ ] Merge Paths
- [ ] Move Attributes to Parent Group
- [ ] Move Group Attributes to Elements
- [ ] Prefer `viewBox` to `width`/`height`
- [ ] Sort Attributes
- [ ] Optimize single-path SVGs
- [ ] Tidy Enable Background

### Removes

- [x] Comments
- [ ] `<desc>` tags
- [ ] `Doctype`
- [ ] Editor Data
- [ ] Empty Attributes
- [ ] Empty Containers
- [ ] Empty Text
- [ ] Hidden Elements
- [ ] Metadata
- [ ] Unneeded Group Attributes
- [ ] Raster Images
- [ ] `<title>`
- [ ] unknowns and defaults
- [ ] unused `defs`
- [ ] unused namespaces
- [ ] useless `stroke` and `fill`
- [ ] `viewBox`
- [ ] XML instructions
- [ ] XMLNS Attribute
- [ ] `<style>` elements

## Inspiration

This project is inspired by the [SVGO](https://svgo.dev) project available for Node.js ecosystem.

## License

This project is licensed under the MIT license.
