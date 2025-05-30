```
███╗   ██╗ █████╗ ███████╗ █████╗ ██████╗  █████╗
████╗  ██║██╔══██╗╚══███╔╝██╔══██╗██╔══██╗██╔══██╗
██╔██╗ ██║███████║  ███╔╝ ███████║██████╔╝███████║
██║╚██╗██║██╔══██║ ███╔╝  ██╔══██║██╔══██╗██╔══██║
██║ ╚████║██║  ██║███████╗██║  ██║██║  ██║██║  ██║
╚═╝  ╚═══╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝
```

Nazara is an experimental Rust program that automates the collection of system information for NetBox, using NetBox's
API. It enables the automatic creation of new machines in NetBox or population of information fields for existing ones.

**Nazara is in the early stages of its development. Please note that the information listed below is subject to change.**

> [!Note]
> Nazara is currently in an alpha state. Bugs are bound to happen. If you encounter any, please [report them](https://github.com/The-Nazara-Project/Nazara/issues).
>
> Furthermore, **Nazara currently does not support custom fields for any NetBox object**. Though, this is the next item on our agenda.

- [Installation](#installation)
  - [Building from source](#building-from-source)
    - [Installation via `crates.io`](#installation-via-cratesio)
- [Usage](#usage)
- [Configuration](#configuration)
  - [Configuring via CLI](#configuring-via-cli)
  - [Configuring via `~/.nazara/config.toml`file.](#configuring-via-nazaraconfigtomlfile)
  - [Configuring custom fields using user plugins](#configuring-custom-fields-using-user-plugins)
- [Contributing](#contributing)
- [License](#license)


# Installation

## Building from source

To use Nazara, you will need to have the Rust programming language and `cargo` installed. If you do not have them
installed already, you can follow the instructions provided in the [official Rust documentation](https://www.rust-lang.org/tools/install).

*Please note that this program only works on Linux systems.*

Once you have everything installed, you can clone this repository and build the program by running the following commands:

```bash
git clone https://github.com/The-Nazara-Project/Nazara.git
cd Nazara
cargo build --release
```

This will create an executable file in the `target/release` directory.

> [!Important]
> Running Nazara stock will cause it to use our NetBox API reference client library [`thanix_client`](https://github.com/The-Nazara-Project/thanix_client).
> This client was generated from the API spec of a stock NetBox instance (1.x from NetBox v3.6.9 and 2.x from NetBox 4.1.0).
> If you encounter API request issues with your NetBox instance, you may need to generate your own using [`Thanix`](https://github.com/The-Nazara-Project/Thanix).


## Installation via `crates.io`

Nazara is published on `crates.io`. If your operating system permits cargo to install packages globally, simply run `cargo install nazara` to install it.

# Usage

To use Nazara, you will need to configure the URL of your NetBox API and provide an API token to the program by
configuring all of these parameters inside the [configuration file](#configuring-via-nazaraconfigtomlfile).

After that, simply run

```bash
nazara
```

in your terminal. Nazara will automatically collect all required system information and decide whether to create a new device, or update an existing entry.

# Configuration

Nazara supports two ways of providing configuration parameters: CLI arguments and a configuration file.

Nazara accepts these parameters from you:

- `-u, --uri`: URI to your NetBox instance.
- `-t, --token`: Your API authentication token.
- `-n, --name`: The name of the device.
- `-p, --plugin`: The path to a plugin script you want to use to fill in custom fields.

## Configuring via CLI

Here is an example for passing these parameters on using the CLI:

```bash
./target/release/Nazara --uri <API_URL> --token <API_TOKEN> --name test_device
```

## Configuring via `~/.nazara/config.toml`file (recommended)

Nazara's configuration must be located in the user's home directory at `~/.nazara/config.toml`.

When launching Nazara for the first time, it will write a stock config file to that path. Certain parameters are required to be configured there manually.
You recognize them by their line not being commented out.

When providing CLI parameters, Nazara will write them into the config file for you.

> [!Note]
> Currently, configuration by config file is the proper way to use Nazara given the amount of data required to register a machine.
> We are investigating possibilities to make this less of a hassle. In the meantime, we suggest you copy-paste the config between machines of the same
> type and function.

```toml
# A default configuration file looks like this:
[netbox]
netbox_uri = ""
netbox_api_token = ""

[system]
name = "some_name" # Required for virtual machines!
site_id = 0 # The ID of the site this device is located at.
description = ""
comments = "Automatically registered using Nazara."
device_type = 0
role = 0
# Name of the network interface to set. (e.g eth0, etc)
# If not set, the first active interface will be selected.
primary_network_interface = ""
face = "" # Direction this device may face (e.g front or rear)
status = "active" # Status of the device. 'active' by default.
airflow = "front-to-rear" # Direction of airflow.
# Optional data of your device
# This section may be empty
[[system.optional]]

# tenant_group = 0 # The ID of the department this device belongs to.
# tenant = 0 # ID of the team or individual this device blongs to.
# location = 0 # ID of the location of the device.
# rack = 0 # ID of the Rack this device sits in.
# position = 0 # Position of the device within the Rack.
platform = "x86_64" # Name of the paltform of this device.
# These will be parsed into a single HashMap. You must provide
# the correct field labels as there is no way for Nazara to know.
# These values are purely exemplary.
[system.custom_fields]

# Network Interfaces Configuration (optional)
#[[nwi]]
#name = "" # Required. Must match interface that exists on the machine.
#enabled = true
#rtype = "type1"
#parent = 1
#bridge = 1
#lag = 1
#mtu = 1500
#duplex = "full"
#wwn = "wwn12345"
#mgmt_only = false
#description = "Automatically created by Nazara."
#mode = ""
#rf_role = ""
#rf_channel = ""
#poe_role = ""
#poe_channel = ""
#rf_channel_frequency = 2400.0
#rf_channel_width = 20.0
#tx_power = 20
#untagged_vlans = [10, 20]
#tagged_vlans = [30, 40]
#mark_connected = true
#wireless_lans = [50, 60]
#vrf = 1
# Custom fields specific for this interface
#[nwi.custom_fields]
# ...
```

*Please note that this section is still a work in progress and all information is subject to change.*

## Configuring custom fields using user plugins

Users are able to fill `custom_fields` parameters in their NetBox objects using custom bash scripts.
These scripts should be placed inside the `~/.nazara/scripts/` directory.

These scripts can collect the desired information and output *a valid JSON representation* to `stdout`.
Nazara then reads this output, validates it, and attempts to parse it to a `HashMap` of values.

If everything works out, this will populate all of your custom fields no matter what fields you specified, as long as your script
is correct.

> [!Warning]
> Users must make sure that the output of their scripts matches the name of their desired custom fields they specified
> in NetBox.
>
> Currently, **we only support text fields** as all the other field types would require smart parsing on our end.
> We are currently investigating on how to achieve this.

# Contributing

If you would like to contribute to Nazara, feel free to check the [contributing guide](./CONTRIBUTING.md) for
information on our workflow and check the issues section for any open issue.

# License

Nazara is released under the terms of the [GPL-v3.0](./LICENSE).
