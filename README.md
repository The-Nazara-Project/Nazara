# Netbox Sync

Netbox Sync is an experimental Rust program that automates teh collection of system information for NetBox, using NetBox's
API. It enables the automatic creation of new machines in NetBox or population of information fields for existing ones.

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
The form of these parameters is influenced by the [rust-netbox](https://github.com/peltzi/rust-netbox) project which we
use to create an API client for the NetBox API.

Netbox Sync requires three parameters from you:

- `API_URL`: The URL of your NetBox API
- `API_TOKEN`: The authentication token for the NetBox API
- `INTERVAL`: The interval (in seconds) in which NetBox sync should fetch and upload information (optional)

#### Configuring via CLI

Here is an example for passing these parameters on using the CLI:

```bash
./target/release/netbox-sync --api-url <API_URL> --api-token <API_TOKEN> --interval <INTERVAL>
```

#### Configuring via `nbs-config.toml`file.

Alternatively, you can provide the configuration parameters in a config file named `nbs-config.toml`, located in the same
directory as the executable file. Here is an example how the config file should look like:

```toml
[netbox]
api_url = "<API_URL>"
api_token = "<API_TOKEN>"
interval = <INTERVAL>
```

*Please note that this section is still a work in progress and all information is subject to change.*

# Contributing

If you would like to contribute to Netbox Sync, feel free to check the [contributing guide](./CONTRIBUTING.md) for
information on our workflow and check the issues section for any open issue.

# License

Netbox Sync is released under the [MIT License](./LICENSE).
