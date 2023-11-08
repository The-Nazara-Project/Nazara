```
░█▀█░█▀▀░▀█▀░█▀▄░█▀█░█░█░░░░░█▀▀░█░█░█▀█░█▀▀
░█░█░█▀▀░░█░░█▀▄░█░█░▄▀▄░▄▄▄░▀▀█░░█░░█░█░█░░
░▀░▀░▀▀▀░░▀░░▀▀░░▀▀▀░▀░▀░░░░░▀▀▀░░▀░░▀░▀░▀▀▀
```

Netbox Sync is an experimental Rust program that automates the collection of system information for NetBox, using NetBox's
API. It enables the automatic creation of new machines in NetBox or population of information fields for existing ones.

**Netbox Sync is in the early stages of its development. Please note that the information listed below is subject to change.**

## Installation

To use Netbox Sync, you will need to have the Rust programming language and `cargo` installed. If you do not have them
installed already, you can follow the instructions provided in the [official Rust documentation](https://www.rust-lang.org/tools/install).

*Please note that this program only works on Linux systems.*

Once you have everything installed, you can clone this repository and build the program by running the following commands:

```bash
git clone https://github.com/ByteOtter/netbox-sync.git
cd netbox-sync
cargo build --release
```

This will create an executable file in the `target/release` directory.

## Usage (WIP)

To use Netbox Sync, you will need to configure the URL of your NetBox API and provide an API token to the program.

### Configuration (WIP)

Netbox Sync supports two ways of providing configuration parameters: CLI arguments and a configuration file.

Netbox Sync requires two parameters from you:

- `API_URL`: The URL of your NetBox API
- `API_TOKEN`: The authentication token for the NetBox API

#### Configuring via CLI

Here is an example for passing these parameters on using the CLI:

```bash
./target/release/netbox-sync --api-url <API_URL> --api-token <API_TOKEN>
```

#### Configuring via `nbs-config.toml`file.

Alternatively, you can provide the configuration parameters in a config file named `nbs-config.toml`, located in the same
directory as the executable file. Here is an example how the config file should look like:

```toml
[netbox]
netbox_api_token = "$API_TOKEN"
netbox_uri = "$API_URI"
```

Aside from the NetBox system parameters, configuration via the `.nbs-config.toml` also allows you to add certain
custom fields to your system information that cannot be automatically selected. A great example would be the
`System Location` entry. To specify that, simply add the parameter under the `[system]` block in your configuration file.

```toml
[netbox]
netbox_api_token = "$API_TOKEN"
netbox_uri = "$NETBOX_URI"

[system]
system_location = "$SYSTEM_LOCATION"
```

An example file would look like this:

```toml
[netbox]
netbox_api_token = "fj453534898235jg8tg24g43hgh9438489453h4kgi3gksd483uggmn"
netbox_uri = "http://netbox.organization.com/api/"

[system]
system_location = "Nuremberg"
```

*Please note that this section is still a work in progress and all information is subject to change.*

# Contributing

If you would like to contribute to Netbox Sync, feel free to check the [contributing guide](./CONTRIBUTING.md) for
information on our workflow and check the issues section for any open issue.

# License

Netbox Sync is released under the [MIT License](./LICENSE).
