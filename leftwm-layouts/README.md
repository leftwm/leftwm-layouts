<div align="center">
  <h1><strong>LeftWM Layouts</strong></h1>
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

This library encapsulates layout calculations from external dependencies and display servers,
so that it can be used by window managers for X.Org, Wayland, or whatever else. It's all about
splitting up rectangles, the library has no concept of "windows".

Some default layouts are provided, but custom layouts can be defined. Custom layout
definitions allow some flexibility but are still fairly limited by design, as the target
audience of this library are **list-based** / **dynamic-tiling** window managers (the likes of `leftwm`, `dwm`, ...) as opposed to manual tilers like `i3`.

## Features

- Already provides widely known default layouts
- Custom layouts can be defined
- Supports multiple main windows
- Ultrawide monitor friendly
- Zero dependencies (*if you ignore serde :eyes:*)

## Pre-defined Layouts

The following layouts are already provided by the library.

### Stack / Single-Column

Those layouts have only a single stack and no main column.

**Monocle**

> **Note**: Only ever display one window at maximum

![Monocle](/icons/yellow/monocle.svg)

**EvenHorizontal**

![EvenHorizontal](/icons/yellow/even-horizontal.svg)

**EvenVertical**

![EvenVertical](/icons/yellow/even-vertical.svg)


**Grid**

![Grid](/icons/yellow/grid.svg)


### Main and Stack / Two-Column

Those layouts have a main and one stack column

**MainAndDeck**

> **Note**: Only ever displays two windows at maximum

![MainAndDeck](/icons/yellow/main-and-deck.svg)

**MainAndVertStack**

![MainAndVertStack](/icons/yellow/main-and-vert-stack.svg)

**MainAndHorizontalStack**

![MainAndHorizontalStack](/icons/yellow/main-and-horizontal-stack.svg)

**RightMainAndVertStack**

![RightMainAndVertStack](/icons/yellow/right-main-and-vert-stack.svg)

**Fibonacci**

![Fibonacci](/icons/yellow/fibonacci.svg)

**Dwindle**

![Dwindle](/icons/yellow/dwindle.svg)

### Center Main / Three-Column

Those layouts have the main column in the center
and two stack columns surrounding it.

**CenterMain**

![CenterMain](/icons/yellow/center-main.svg)

**CenterMainBalanced**

![CenterMainBalanced](/icons/yellow/center-main-balanced.svg)

**CenterMainFluid**

> **Note**: Unoccupied column space is reserved

![CenterMainFluid](/icons/yellow/center-main-fluid.svg)

---

## Build

### Build and test the project

> **Note** Requires Rust >= 1.56.0

```shell
make
```

### Run the demo application

> **Note** Running the demo requires you to have GTK (`gtk3`) and pango (`?`) installed

```shell
make dev
```

