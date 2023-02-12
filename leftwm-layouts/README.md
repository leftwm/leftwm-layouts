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

```txt
+-----------+
|           |   only ever displays
|           |   one window at
|           |   maximum
+-----------+
```

**EvenHorizontal**

```txt
+--+--+--+--+
|  |  |  |  |
|  |  |  |  |
|  |  |  |  |
+--+--+--+--+
```

**EvenVertical**

```txt
+-----------+
|-----------|
|-----------|
|-----------|
+-----------+
```

**Grid**

```txt
+-----+-----+   +---+---+---+   +---+---+---+   +---+---+---+
|     |     |   |   |   |   |   |   |   |   |   |   |   |   |
|     |     |   |   |   |   |   |   |   |   |   |   |   +---+
+-----+-----+   |   +---+---+   +---+---+---+   +---+---|   |
|     |     |   |   |   |   |   |   |   |   |   |   |   +---+
|     |     |   |   |   |   |   |   |   |   |   |   |   |   |
+-----+-----+   +---+---+---+   +---+---+---+   +---+---+---+
  4 windows       5 windows       6 windows       7 windows
```

### Main and Stack / Two-Column

Those layouts have a main and one stack column

**MainAndDeck**

```txt
+-------+-----+
|       |     |   only ever displays
|       |     |   two windows at
|       |     |   maximum
+-------+-----+
  main   stack
```

**MainAndVertStack**

```txt
+-------+-----+
|       |     |
|       +-----+
|       |     |
+-------+-----+
  main   stack
```

**MainAndHorizontalStack**

```txt
+-------+--+--+
|       |  |  |
|       |  |  |
|       |  |  |
+-------+--+--+
  main   stack
```

**RightMainAndVertStack**

```txt
+-----+-------+
|     |       |
+-----+       |
|     |       |
+-----+-------+
 stack   main
```

**Fibonacci**

```txt
+-------+-----+
|       |     |
|       +--+--+
|       |--|  |
+-------+--+--+
  main   stack
```

**Dwindle**

```txt
+-------+-----+
|       |     |
|       +--+--+
|       |  |--|
+-------+--+--+
  main   stack
```

### Center Main / Three-Column

Those layouts have the main column in the center
and two stack columns surrounding it.

**CenterMain**

```text
+-----+-----------+-----+
|     |           |     |
|     |           +-----+
|     |           |     |
|     |           +-----+
|     |           |     |
+-----+-----------+-----+
  1st      main     2nd
 stack             stack

+-----------+-----------+
|           |           |
|           |           |  unoccupied
|           |           |  space is
|           |           |  taken over
|           |           |
+-----------+-----------+
  1st stack      main

+-----------------------+
|                       |
|                       |  unoccupied
|                       |  space is
|                       |  taken over
|                       |
+-----------------------+
           main
```

**CenterMainBalanced**

```text
+-----+-----------+-----+
|     |           |     |
|     |           |     |
+-----+           +--+--+
|  |__|           |  |__|
|  |  |           |  |  |
+-----+-----------+--+--+
  1st      main     2nd
 stack             stack

+-----------+-----------+
|           |           |
|           |           |  unoccupied
|           |           |  space is
|           |           |  taken over
|           |           |
+-----------+-----------+
  1st stack      main

+-----------------------+
|                       |
|                       |  unoccupied
|                       |  space is
|                       |  taken over
|                       |
+-----------------------+
           main
```

**CenterMainFluid**

```text
 1st               2nd
 stack     main    stack
+-----+-----------+-----+
|     |           |     |
|     |           +-----+
|     |           |     |
|     |           +-----+
|     |           |     |
+-----+-----------+-----+
  1st      main     2nd
 stack             stack

+-----+-----------+-----+
|     |           |.....|
|     |           |.....|  unoccupied
|     |           |.....|  space is
|     |           |.....|  reserved
|     |           |.....|
+-----+-----------+-----+
  1st      main
 stack

+-----+-----------+-----+
|.....|           |.....|
|.....|           |.....|  unoccupied
|.....|           |.....|  space is
|.....|           |.....|  reserved
|.....|           |.....|
+-----+-----------+-----+
           main
```

---

## Build

### Build and test the project

> **Info** Requires Rust >= 1.56.0

```shell
make
```

### Run the demo application

> **Info** Running the demo requires you to have GTK (`gtk3`) and pango (`?`) installed

```shell
make dev
```

