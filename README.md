<div align="center">
  <h1><strong>LeftWM Layouts</strong> (beta)</h1>
  <p>
    <strong>Library providing fixed but parameterized window layout calculations</strong>
  </p>
  <p>
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

## Description

```txt
+--------+        +--+--+--+        +---+---+        +---+---+        +---+---+
|        |        |  |  |  |        |   |   |        |   |   |        |   |   |
+--------+        |  |  |  |        |   |   |        |   |   |        |   |   |
|        |        |  |  |  |        |   +---+        |   +-+-+        |   +-+-+
+--------+        |  |  |  |        |   |   |        |   |_| |        |   | |_|
|        |        |  |  |  |        |   |   |        |   | | |        |   | |||
+--------+        +--+--+--+        +---+---+        +---+---+        +---+---+
                            
horizontal         vertical           grid           fibonacci         dwindle
```

---

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
make dev
```