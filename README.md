# libxml2-rs

[![Rust][rust-badge]][rust]
[![C][c-badge]][libxml2]
[![github][github-badge]][github]
[![cmake][cmake-badge]][cmake]
[![xml][xml-badge]][libxml2]
[![github actions][actions-badge]][actions]
[![Test status][test-badge]][test]
[![Coffee][buy-me-coffee]][coffee]
[![license][license-badge]][license]

Rust bindings for the [libxml2][libxml2] C library.

## Installation

On UNIX based systems such as macOS and Linux, the installation is simple.
Assuming that both Cargo and libxml2 is installed.
If not, then Cargo can be installed using [Rustup][rustup].

### libxml2 (Linux/macOS)

On Ubuntu/Debian systems:

```sh
apt-get install libxml2-dev
```

On macOS:

- Homebrew:

```sh
brew install libxml2
```

- MacPorts:

```sh
sudo port install libxml2
```

- Conda

```sh
conda install libxml2
```

### Build libxml2-rs (Linux/macOS)

To build and test the library, use:

```sh
cargo build
cargo test
```

### Windows installation

Just using the standard `cargo build` command on Windows may result in an error, here is a guide on installing libxml2 and `libxml2-rs` on Windows:

#### Install libxml2 (vcpkg)

The best way to install libxml2 on Windows is with [vcpkg][vcpkg], using:

```pwsh
vcpkg install libxml2
```

The result of this will provide the path to a toolchain file `vcpkg.cmake`.
Since libxml2-rs uses CMake to build the C bindings,
it is necessary to pass the toolchain to CMake using an environment variable

```pwsh
$Env:CMAKE_TOOLCHAIN_FILE = "$Env:VCPKG_INSTALLATION_ROOT\scripts\buildsystems\vcpkg.cmake"
```

#### Include files

Using the above command allows CMake to find libxml2,
but running `cargo build` fails to find the libxml2 header files.
These can be specified using:

```pwsh
# For Windows (x64)
$Env:LIBXML2_INCLUDE_DIR = "$Env:VCPKG_INSTALLATION_ROOT\installed\x64-windows\include\libxml2\"
```

This allows `cargo build` to run correctly.
However, the `cargo test` command and any executable built with this library will fail.

#### Linking and running tests

The `cargo test` command fails since the library file `libxml2.lib` is not found. The location for this may be specified using:

```pwsh
# x64
$Env:LIBXML2_LIBRARY_DIR = "$Env:VCPKG_INSTALLATION_ROOT\installed\x64-windows\lib\"
```

Building the test executables then succeeds, but the tests may still fail if the vcpkg binary directory is not in your `PATH` variable, e.g.

```pwsh
# x64
$Env:PATH = "$Env:PATH;$Env:VCPKG_INSTALLATION_ROOT\installed\x64-windows\bin\;"
```

Assuming that these variables are set correctly, building and testing should work.

<!-- Links -->

[rust]: https://rust-lang.org/
[github]: https://github.com/zwill22/libxml2-rs
[cmake]: https://cmake.org
[libxml2]: https://gitlab.gnome.org/GNOME/libxml2
[license]: https://github.com/zwill22/libxml2-rs/LICENSE
[test]: https://github.com/zwill22/libxml2-rs/actions/workflows/test.yml
[coffee]: https://coff.ee/zmwill
[actions]: https://github.com/zwill22/libxml2-rs/actions
[rustup]: https://rustup.rs/
[vcpkg]: https://github.com/microsoft/vcpkg

<!-- Badges -->

[c-badge]: https://img.shields.io/badge/C-00599C?logo=c&logoColor=white&style=for-the-badge
[rust-badge]: https://img.shields.io/badge/Rust-AA0000?logo=rust&logoColor=white&style=for-the-badge
[github-badge]: https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white
[cmake-badge]: https://img.shields.io/badge/CMake-%23008FBA.svg?style=for-the-badge&logo=cmake&logoColor=white
[buy-me-coffee]: https://img.shields.io/badge/Buy_Me_A_Coffee-FFDD00?logo=buy-me-a-coffee&logoColor=black&style=for-the-badge
[actions-badge]: https://img.shields.io/badge/GitHub_Actions-2088FF?logo=github-actions&logoColor=white&style=for-the-badge
[test-badge]: https://img.shields.io/github/actions/workflow/status/zwill22/libxml2-rs/test.yml?style=for-the-badge&logo=github
[license-badge]: https://img.shields.io/github/license/zwill22/libxml2-rs?style=for-the-badge
[xml-badge]: https://img.shields.io/badge/XML-767C52?logo=xml&logoColor=fff&style=for-the-badge
