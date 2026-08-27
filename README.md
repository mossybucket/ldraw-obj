# LDraw OBJ Converter

A lightweight Rust converter for turning **LDraw `.dat` files** into standard **Wavefront `.obj` 3D models**.

## Usage

```bash
ldraw-obj <dat-folder> <output-folder>
```

Or run directly with Cargo:

```bash
cargo run -- <dat-folder> <output-folder>
```

### Example

```bash
ldraw-obj ./ldraw/p ./output
```

This converts the `.dat` files in the specified folder and writes the resulting `.obj` files to the output folder.

## Requirements

* [Rust](https://www.rust-lang.org/)
* Cargo
* An LDraw library containing the required `.dat` files and sub-file definitions

You can download the complete LDraw library [here](https://library.ldraw.org/library/updates/complete.zip).

## Installation

Clone the repository:

```bash
git clone https://github.com/mossybucket/ldraw-obj.git
cd ldraw-obj
```

Build the converter in release mode:

```bash
cargo build --release
```

The compiled executable will be located at:

```text
target/release/ldraw-obj
```

## Supported Geometry

The converter currently handles the main LDraw geometry primitives:

* `0` — Comments / metadata
* `1` — Sub-file references
* `2` — Lines
* `3` — Triangles
* `4` — Quadrilaterals
* `5` — Conditional lines

## Output

The generated OBJ files use standard Wavefront OBJ geometry:

* `v` — Vertices
* `vt` — Texture coordinates, when available
* `vn` — Vertex normals, when generated
* `f` — Faces

The resulting models can be opened in applications such as **Blender**, **MeshLab**, and other software supporting the Wavefront OBJ format.

## Limitations

This project is primarily intended for converting LDraw geometry rather than being a complete LDraw renderer.

Some LDraw-specific features may not translate perfectly to OBJ.

In particular:

* Stud logos are not currently included.
* Pieces are not heavily rounded and may appear more angular than real LEGO pieces.
* Some LDraw rendering features may be lost during conversion.
* Conditional lines and other rendering-specific geometry may not have an equivalent in OBJ.

## Project

Package name:

```toml
[package]
name = "ldraw-obj"
```

Repository:

https://github.com/mossybucket/ldraw-obj

## License

See the `LICENSE` file for licensing information.
