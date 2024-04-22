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

## Installation

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

## Usage (WIP)

To use Nazara, you will need to configure the URL of your NetBox API and provide an API token to the program.

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
netbox_api_token = "$API_TOKEN"
netbox_uri = "$NETBOX_URI"

[system]
system_location = "$SYSTEM_LOCATION"
```

An example file would look like this:

```toml
# Template nazara-config.toml file for v0.1.0

# Configuration parameters for the NetBox connection
[netbox]
netbox_api_token = "{NETBOX_TOKEN}"
netbox_uri = "{NETBOX_URI}"

# Mandatory information about the system
[system]
name = "{SYSTEM_NAME}" # Name of the machine or VM. **Required** when device is a VM
site_id = 0 # The id of the site this device is located at. (Stored in NetBox)
# site_name = "" # Name of the site this device is located at. (May take longer to search for) (Mutually exclusive with ID!)
description = ""
comments = "Automatically registered by Nazara."
device_type = 0 # ID of the type of the Device Type in NetBox
device_role = 0 # ID of the device role in NetBox
# Name of the network interface to set. (e.g eth0, etc)
# If not set, the first active interface will be selected.
primary_network_interface = "eth0"
face = "" # Direction this device may face in (e.g front or rear)
status = "active" # status of the device. default: active
airflow = "front-to-rear" # Direction of airflow

# Optional data of your device.
# This section may be empty
[[system.optional]]
# tenant_group = 0 # ID of the department this device belongs to
# tenant_group_name = "" # Name of the department this device belongs to (mutually exclusive with ID!)
# tenant = 0 # ID of the team or individual this device belongs to
# tenant_name = "" # Name of the individual or team this device belongs to (mutually exclusive with ID!)
# location = 0 # ID of the location of the device
# rack = 0 # ID of the rack the device is mounted in if any
# position = 0 # Position of the device within the rack if any
platform = "x86_64"

# Custom parameters about the system, which fall under the "custom_fields" section.
# Basically anything can go here, these are example values Nazara collects.
[system.custom_fields]
# cpu_count = 1 # overriden by collector
# config_template = 0
##  ...
```

*Please note that this section is still a work in progress and all information is subject to change.*

# Contributing

If you would like to contribute to Nazara, feel free to check the [contributing guide](./CONTRIBUTING.md) for
information on our workflow and check the issues section for any open issue.

# License

Nazara is released under the terms of the [GPL-v3.0](./LICENSE).
