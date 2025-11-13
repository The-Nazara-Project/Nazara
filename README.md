<img src="https://github.com/user-attachments/assets/1a0ab81a-544f-45ef-a95d-46689a5ec922" alt="Alt Text" style="width:20%; height:auto;">

Nazara is an experimental Rust program that automates the collection of system
information for NetBox, using NetBox's API. It enables the automatic creation of
new machines in NetBox or population of information fields for existing ones.

**Nazara is in the early stages of its development. Please note that the
information listed below is subject to change.**

- [Compatibility](#compatibility)
- [Installation](#installation)
  - [Building from source](#building-from-source)
  - [Installation via `crates.io`](#installation-via-cratesio)
- [Usage](#usage)
- [Configuration](#configuration)
  - [Configuring via CLI](#configuring-via-cli)
    - [The `config` commands](#the-config-commands)
  - [Configuring via `$HOME/.config/nazara/config.toml` (recommended)](#configuring-via-homeconfignazaraconfigtoml-recommended)
  - [Configuring custom fields using user plugins](#configuring-custom-fields-using-user-plugins)
- [Contributing](#contributing)
- [License](#license)

# Compatibility

We strive to make sure to stay compatible with the most recent NetBox version.
Here you can see which version of Nazara is compatible with which version of
NetBox. When major ports to newer NetBox versions happen - which usually include
breaking changes - the old version of Nazara will be moved to its own branch and
tagged accordingly.

| Nazara Version   | NetBox Version     | Branch            | maintained?        |
| ---------------- | ------------------ | ----------------- | ------------------ |
| `v0.1.0`         | `v4.3.x`, `v4.4.x` | `main`            | :white_check_mark: |
| `v0.1.0_beta.3`  | `v4.3.x`, `v4.4.x` | `version/beta-3`  | :x:                |
| `v0.1.0_beta.2`  | `v4.3.x`, `v4.4.x` | `version/beta-2`  | :x:                |
| `v0.1.0_beta.1`  | `v4.3.x`           | `version/beta-1`  | :x:                |
| `v0.1.0_alpha.2` | `v3.6.x`           | `version/alpha-2` | :x:                |

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
git clone https://github.com/The-Nazara-Project/Nazara.git
cd Nazara
cargo build --release
```

This will create an executable file in the `target/release` directory.

> [!Important] Running Nazara stock will cause it to use our NetBox API
> reference client library
> [`thanix_client`](https://github.com/The-Nazara-Project/thanix_client). This
> client was generated from the API spec of a stock NetBox instance (1.x from
> NetBox v3.6.9 and 2.x from NetBox 4.1.0). If you encounter API request issues
> with your NetBox instance, you may need to generate your own using
> [`Thanix`](https://github.com/The-Nazara-Project/Thanix).

## Installation via `crates.io`

Nazara is published on `crates.io`. If your operating system permits cargo to
install packages globally, simply run `cargo install nazara` to install it.

# Usage

To use Nazara, you will need to configure the URL of your NetBox API and provide
an API token to the program by configuring all of these parameters inside the
[configuration file](#configuring-via-nazaraconfigtomlfile).

After that, simply run

```bash
nazara register
```

to register a new machine, or run

```bash
nazara update $MACHINE_ID
```

to update an existing one.

in your terminal. Nazara will automatically collect all required system
information and decide whether to create a new device, or update an existing
entry.

# Configuration

Nazara supports two ways of providing configuration parameters: CLI arguments
and a configuration file.

Nazara accepts these parameters from you:

- `-d, --dry-run`: Print all collected information without committing it to
  NetBox.
- `-u, --uri <URI>`: URI to your NetBox instance.
- `-t, --token <TOKEN>`: Your API authentication token.
- `-p, --plugin <PLUGIN>`: The path to a plugin script you want to use to fill
  in custom fields.
- `-h, --help`: Print help.
- `-V, --version`: Print version.

## Configuring via CLI

Here is an example for passing these parameters on using the CLI:

```bash
sudo nazara --uri <API_URL> --token <API_TOKEN> --name test_device
```

When launching Nazara for the first time, a configuration file will be written
at `$HOME/.config/nazara/config.toml`. If you pass CLI parameters, these will be
automatically transfered into the config file as well.

### The `config` commands

Nazara provides you with several commands to manage your configuration files:

- `write-config`: Write a new config file or overwrite an existing one.
- `check-config`: Validate if your config is still valid.
- `view-config`: Print config to console.

The `write-config` allows you to change individual parameters, or perform a bulk
update by passing a `JSON` structure via CLI. **These options are exclusive.
Passing both is disallowed.**

For further information on how to configure Nazara, run `nazara --help` or visit
[our documentation](https://the-nazara-project.github.io/Nazara/users/configuration.html).

## Configuring via `$HOME/.config/nazara/config.toml` (recommended)

Nazara's configuration must be located in the root user's home directory at
`$HOME/.config/nazara/config.toml`.

When launching Nazara for the first time, it will write a stock config file to
that path. Certain parameters are required to be configured there manually. You
recognize them by their line not being commented out.

Aside from the NetBox system parameters, configuration via the `config.toml`also
allows you to add certain custom fields to your system information that cannot
be automatically selected. A great example would be the`System
Location`entry.
To specify that, simply add the parameter under the`[system]` block in your
configuration file.

> [!Note] Currently, configuration by config file is the proper way to use
> Nazara given the amount of data required to register a machine. We are
> investigating possibilities to make this less of a hassle. In the meantime, we
> suggest you copy-paste the config between machines of the same type and
> function.

For an example config, check out `src/configuration/config_template.toml`

_Please note that the following section is still a work in progress and all
information is subject to change._

## Configuring custom fields using user plugins

Users are able to fill `custom_fields` parameters in their NetBox objects using
custom bash scripts. These scripts should be placed inside the
`$HOME/.config/nazara/scripts/` directory.

These scripts can collect the desired information and output _a valid JSON
representation_ to `stdout`. Nazara then reads this output, validates it, and
attempts to parse it to a `HashMap` of values.

If everything works out, this will populate all of your custom fields no matter
what fields you specified, as long as your script is correct.

> [!Warning] Users must make sure that the output of their scripts matches the
> name of their desired custom fields they specified in NetBox.
>
> Currently, **we only support text fields** as all the other field types would
> require smart parsing on our end. We are currently investigating on how to
> achieve this.

# Contributing

If you would like to contribute to Nazara, feel free to check the
[contributing guide](./CONTRIBUTING.md) for information on our workflow and
check the issues section for any open issue.

# License

Nazara is released under the terms of the [GPL-v3.0](./LICENSE).

By submitting a contribution to The Nazara Project, you agree that your
contribution shall be licensed under the same license(s) as the project at the
time of contribution.

You also grant the project maintainers the right to relicense the project,
including your contribution, under any future version of those license(s), or
under any other license that is free and open source software (FOSS) and
compatible with the current license(s).
