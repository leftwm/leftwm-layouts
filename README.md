<div align="center">
  <h1><strong>LeftWM Layouts</strong> (beta)</h1>
  <p>
    <strong>The independent library of the LeftWM layouts</strong>
  </p>
  <p>
    <a href="https://github.com/leftwm/leftwm-layouts/actions/workflows/ci.yml">
        <img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/leftwm/leftwm-layouts/CI">
    </a>
    <a href="#">
        <img alt="Libraries.io dependency status for GitHub repo" src="https://img.shields.io/librariesio/github/leftwm/leftwm-layouts" />
    </a>
    <a href="https://crates.io/crates/leftwm-layouts">
        <img alt="Crates.io" src="https://img.shields.io/crates/v/leftwm-layouts">
    </a>
    <a href="https://crates.io/crates/leftwm-layouts">
        <img alt="Crates.io" src="https://img.shields.io/crates/d/leftwm-layouts">
    </a>
  </p>
</div>

> :warning: This is a work in progress and in early development. Package may be discontinued and instead become directly integrated into [leftwm-core](https://github.com/leftwm/leftwm/tree/main/leftwm-core)

## Requirements

### leftwm-layouts
- Rust >= 1.56.0

### demo
- GTK (`gtk3`)
- pango (?)

## Usage

### Build and test the project
```shell
make
```

### Run the demo application
```shell
cargo run --package demo
```