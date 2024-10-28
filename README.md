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

## Building from Source

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

## Installation

### Installation via `crates.io`

Nazara is published on `crates.io`. If your operating system permits cargo to install packages globally, simply run `cargo install nazara` to install it.

## Usage (WIP)

To use Nazara, you will need to configure the URL of your NetBox API and provide an API token to the program.

After that, simply run

```bash
nazara
```

in your terminal. Nazara will automatically collect all required system information and decide whether to create a new device, or update an existing entry.

> [!Note]
> Nazara is currently in an alpha state. Bugs are bound to happen. If you encounter any, please [report them](https://github.com/The-Nazara-Project/Nazara/issues).
>
> Furthermore, **Nazara currently does not support custom fields for any NetBox object**. Though, this is the next item on our agenda.

### Configuration (WIP)

Nazara supports two ways of providing configuration parameters: CLI arguments and a configuration file.

Nazara requires two parameters from you:

- `API_URL`: The URL of your NetBox API
- `API_TOKEN`: The authentication token for the NetBox API

#### Configuring via CLI

Here is an example for passing these parameters on using the CLI:

```bash
./target/release/Nazara --api-url <API_URL> --api-token <API_TOKEN>
```

#### Configuring via `nbs-config.toml`file.

Alternatively, you can provide the configuration parameters in a config file named `nazara-config.toml`, located in the same
directory as the executable file. Here is an example how the config file should look like:

```toml
[netbox]
netbox_api_token = "$API_TOKEN"
netbox_uri = "$API_URI"
```

Aside from the NetBox system parameters, configuration via the `.nazara-config.toml` also allows you to add certain
custom fields to your system information that cannot be automatically selected. A great example would be the
`System Location` entry. To specify that, simply add the parameter under the `[system]` block in your configuration file.

```toml
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

## Custom Plugins

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
> We are currently investiating on how to achieve this.

# Contributing

If you would like to contribute to Nazara, feel free to check the [contributing guide](./CONTRIBUTING.md) for
information on our workflow and check the issues section for any open issue.

# License

Nazara is released under the terms of the [GPL-v3.0](./LICENSE).
