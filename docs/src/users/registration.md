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
of allowed config parameters, please refer to our [template](https://github.com/The-Nazara-Project/Nazara/blob/main/src/configuration/config_template.toml).

You can verify the integrity of your config by running `nazara check-config`.
~~~

### 2. Run `nazara register`

To register your device, simply run

```bash
nazara register
```

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

This could be a bug! Please [report it](http://github.com/The-Nazara-Project/Nazara/issues).
~~~
