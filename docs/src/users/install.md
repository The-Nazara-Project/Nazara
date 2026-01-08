# Installing Nazara

You can install Nazara in a bunch of different ways.
Our recommended solutions is to either get it from [`crates.io`](https://crates.io/crates/nazara), or
build the latest release from source yourself.


~~~ admonish info title="Native Packages"
We are currently working on building distribution packages in the future with the first ones being
targeted at openSUSE Tumbleweed, Slowroll, Leap and SLES16.

If you would like to build a package for your distribution, please [refer to our packager's guide](../contributors/becoming_packager.md).
~~~

## Installing via `crates.io`

To install Nazara via Rust's package index, make sure you have `cargo` and a current Rust toolchain installed.

Then in your Terminal, run

```bash
cargo install --locked nazara
```

After installation you should be able to run Nazara just like you would any other program. If it doesn't work, it is likely
that cargo's bin directory is not in your path. Refer to cargo's documentation for help with that problem.

## Building from Source

For this, please make sure you have cargo, `libopenssl` and a current Rust toolchain installed. (The last of which should be 
compatible with Rust edition `2024`).

Simply clone the repository and run `cargo build` to build it yourself.

```bash
git clone https://github.com/The-Nazara-Project/Nazara && cd Nazara
cargo build --release
```

This process may take a while, mainly thanks to our API client library
[thanix_client](https://codeberg.org/nazara-project/thanix_client).

Once completed you have a portable binary at `./target/release/nazara`.

## Native Packages

We are currently at work to provide native Linux packages starting with the openSUSE family of Linux distributions.

Once we make progress on this front, you will find this information here.

### Release Attachments

We currently provide a pre-built binary as well as a Debian package attached to the latest release
and also plan on attaching these to every release going forward as an easy way to download and install Nazara.

Visit [our release page](https://codeberg.org/nazara-project/Nazara/releases) to find these for yourself.

#### Installing the .deb Package

To install the `.deb` package you downloaded from the Release, use the `dpkg` command.

```bash
sudo dpkg -i nazara_0.1.0-1_amd64.deb
```

