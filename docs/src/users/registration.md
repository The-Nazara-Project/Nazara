# Registering a Device or VM

Registering a device with Nazara is as straight forward as
[setting up the config file](./configuration.md) and running
a simple command.

~~~admonish info 
Please make sure you have `sudo` privileges on the machine or VM you want to register, otherwise
DMI information collection **will fail**.
~~~

## Example Registration Workflow (Physical Device)

On a physical device, a example registration workflow may look like this:

### 1. Set up Configuration

The user wrote a configuration file at `/root/.config/nazara/config.toml`

~~~admonish example title="Example: Device Config" collapsible=true
```toml
# Configuration parameters for the NetBox connection
[netbox]
netbox_api_token = "XXXXXXXX"
netbox_uri = "https://netbox.organisation.com"

# Common settings that always have to be provided
[common]
# The name of the device/VM
name = "user@"
description = ""
# A comment left by Nazara if anything gets modified by it.
comments = "Automatically registered by Nazara."
# The current status of the device/VM
status = "active"
primary_ip4 = "192.168.0.1"
primary_ip6 = ""

# ---------------------------------------------------------------
# Use [device] for devices, or [vm] if this is a virtual machine.
# ---------------------------------------------------------------

[device]
device_type = 1
role = 1
site = 1
```

The configuration file for a VM looks quite similarly. For a list
of allowed config parameters, please refer to our [template](https://codeberg.org/nazara-project/Nazara/src/branch/main/src/configuration/config_template.toml).

You can verify the integrity of your config by running `nazara check-config`.
~~~

### 2. Run `nazara register`

To register your device, simply run

```bash
sudo nazara register
```

This will register your device, its interfaces and ip-addresses **statically** in NetBox.

If you are in an environment where IP addresses are managed by DHCP, Nazara offers several DHCP modes to handle this.

To do so, pass the `--ip-mode` to either the `register` or `update` command to switch between modes.

### DHCP Compatibility

#### `static` Mode

Default behaviour of Nazara. Simply registers everything without paying attention to any environments.

**This may crash if the device's IP addresses change, no reconcilliation will take place.**

```bash
sudo nazara register --ip-mode static
```

#### `dhcp-ignored` Mode

In cases where a DHCP servers syncs IP addresses with NetBox, **any registration or update of IP addresses will be skipped**.

```bash
sudo nazara register --ip-mode dhcp-ignored
```

#### `dhcp-observed` Mode

The most complex mode, this will register the device or VM with **all IP addresses, as they are currently discovered**.
Nazara will try to reconcile the IP addresses it discovers, with what is present in NetBox.

If the address does not exist, it will be created, if it is assigned to a different interface, it will be reassigned.

**The IP addresses will be tagged with `dhcp` tags.**

```bash
sudo nazara register --ip-mode dhcp-observed
```

This mode is to be used in cases where Netbox itself manages available IP addresses and a DHCP server syncs from that.

### Common Issues

#### TOML Deserialization Error

This is most likely an issue with your config file.
Make sure it has correct TOML syntax and all the fields
specified in the template are present in your example.

#### Unexpected Response

We know this output is ugly but please bear with us we are on it.
Check the output for a status code:

- **400 - Bad Request**:
  This usually means that Nazara's payload was rejected by NetBox. Most commonly
  this means that you want to assign this device to a role, device type or site
  that does not exist (invalid ID). It can also mean that the device already exists
  so check if a device with the configured name has already been registered.

#### File OP error

This usually indicates that Nazara cannot open the config file at `/root/.config/nazara/config.toml`
it's either missing, or you forgot to run Nazara as root.

~~~admonish bug title="Any missing?"
You have issues with registering your device and don't know how to proceed?

This could be a bug! Please [report it](http://codeberg.org/nazara-project/Nazara/issues).
~~~
