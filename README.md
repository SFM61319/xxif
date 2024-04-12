# `xxif`

_A Rust program to generate image hashes and add them to the images' EXIF data._

## Installation

1. Using `cargo` (from [GitHub](https://github.com/SFM61319/xxif)):

```sh
cargo install --git https://github.com/SFM61319/xxif
```

2. Using `cargo` from ([crates.io](https://crates.io/)):

```sh
cargo install xxif
```

3. From [GitHub Releases](https://github.com/SFM61319/xxif/releases/tag/latest):

## Usage

_Run this command for the complete help text._

```sh
xxif --help
```

1. Run in the current directory using all cores:

```sh
xxif
```

2. Run in a specific path (directory or file) using all cores:

```sh
xxif -p path/to/images
```

3. Run in the current directory using a specific number of cores:

```sh
xxif -n 4
```

4. Run in a specific path (directory or file) using a specific number of cores:

```sh
xxif -p path/to/images -n 4
```

> Additionally, you can use the `-v` flag to enable verbose mode.

## License

[Licensed](LICENSE "Click to open the license file") under the [MIT License](https://opensource.org/license/MIT "Click to view the license text").
