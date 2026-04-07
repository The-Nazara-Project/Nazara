<div align="center">
<img src="https://github.com/user-attachments/assets/1a0ab81a-544f-45ef-a95d-46689a5ec922" alt="Alt Text" width="250" height="300">
<h3>Nazara</h3>
<hr>
</div>

Nazara is an experimental Rust program that automates the collection of system
information for NetBox, using NetBox's API. It enables the automatic creation of
new machines in NetBox or population of information fields for existing ones.

**Nazara is in the early stages of its development. Please note that the
information listed below is subject to change.**

- [Compatibility](#compatibility)
- [Installation](#installation)
  - [Building from source](#building-from-source)
  - [Installation via `crates.io`](#installation-via-cratesio)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

# Compatibility

We strive to make sure to stay compatible with the most recent NetBox version.
Here you can see which version of Nazara is compatible with which version of
NetBox. When major ports to newer NetBox versions happen - which usually include
breaking changes - the old version of Nazara will be moved to its own branch and
tagged accordingly.

| Nazara Version   | NetBox Version       | Branch            | maintained?        |
| ---------------- | -------------------- | ----------------- | ------------------ |
| `v0.2.0`         | `v4.3.x`, and newer  | `main`            | :white_check_mark: |
| `v0.1.1`         | `v4.3.x`, and newer* | `version/0.1.1`   | :x:                |
| `v0.1.0`         | `v4.3.x`, `v4.4.x`   | `version/0.1.0`   | :x:                |
| `v0.1.0_beta.3`  | `v4.3.x`, `v4.4.x`   | `version/beta-3`  | :x:                |
| `v0.1.0_beta.2`  | `v4.3.x`, `v4.4.x`   | `version/beta-2`  | :x:                |
| `v0.1.0_beta.1`  | `v4.3.x`             | `version/beta-1`  | :x:                |
| `v0.1.0_alpha.2` | `v3.6.x`             | `version/alpha-2` | :x:                |

Maintenance work on these older versions is not planned.

# Installation

## Building from source

To use Nazara, you will need to have the Rust programming language and `cargo`
installed. If you do not have them installed already, you can follow the
instructions provided in the
[official Rust documentation](https://www.rust-lang.org/tools/install).

_Please note that this program only works on Linux systems._

Once you have everything installed, you can clone this repository and build the
program by running the following commands:

```bash
git clone https://codeberg.org/nazara-project/Nazara.git
cd Nazara
cargo build --release
```

This will create an executable file in the `target/release` directory.

> [!IMPORTANT]
> Running Nazara stock will cause it to use our NetBox API reference client
> library [`thanix_client`](https://codeberg.org/nazara-project/thanix_client).
> This client was generated from the API spec of a stock NetBox instance (1.x
> from NetBox v3.6.9 and 2.x from NetBox 4.1.0). If you encounter API request
> issues with your NetBox instance, you may need to generate your own using
> [`Thanix`](https://codeberg.org/nazara-project/Thanix).

## Installation via `crates.io`

Nazara is published on `crates.io`. If your operating system permits cargo to
install packages globally, simply run `cargo install nazara` to install it.

# Documentation

The best place to find information about how to set up and use Nazara is our
[User Documentation](https://nazara-project.codeberg.page/Nazara/users/).

The online version is always up-to-date with the latest released Nazara version.

# Contributing

If you would like to contribute to Nazara, feel free to check the
[contributing guide](https://nazara-project.codeberg.page/Nazara/contributors/)
for information on our workflow and check the issues section for any open
issues.

> [!TIP]
> Issues tagged `good first issue` are specifically written for and targeted
> towards first time outside contributors.

# License

Nazara is released under the terms of the [GPL-v3.0](./LICENSE).

By submitting a contribution to The Nazara Project, you agree that your
contribution shall be licensed under the same license(s) as the project at the
time of contribution.

You also grant the project maintainers the right to relicense the project,
including your contribution, under any future version of those license(s), or
under any other license that is free and open source software (FOSS) and
compatible with the current license(s).
