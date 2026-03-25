# Configuration

~~~admonish danger title="`v0.2.0`: Breaking Change in Configuration with NetBox `v4.5.x`"
Since NetBox `v4.5.x`, NetBox requires a different format of API token in the authentication header.
This applies for any new `v2` token. `v1` tokens are unaffected by this.

In your config file, instead of:

    netbox_api_token = "$TOKEN"

you have to write:

    netbox_api_token = "nbt_$KEY.$TOKEN"

The `$KEY` value can be found in the corresponding "Key" field in the Netbox WebUI when looking at the
API key entry.

We are currently looking into updating our documentation and API client.

Codeberg Issue Reference: [#179](https://codeberg.org/nazara-project/Nazara/issues/179)
~~~

Nazara supports two ways of providing configuration parameters: CLI arguments and a configuration file.

Nazara accepts these parameters from you:

- `-d, --dry-run`: Print all collected information without committing it to NetBox.
- `-u, --uri <URI>`: URI to your NetBox instance.
- `-t, --token <TOKEN>`: Your API authentication token.
- `-p, --plugin <PLUGIN>`: The path to a plugin script you want to use to fill in custom fields.
- `-h, --help`: Print help.
- `-V, --version`: Print version.
- `--log_level`: Changes the log level. Defaults to `debug`

Afterwards, Nazara expects one of the following operation types to be specified:

- `register`: Register a new device or vm in NetBox.
- `update --id <device_id>`: To update an existing device or vm.
- `auto`: Let Nazara decide based on the machine's name and serial number whether
          registration or update is needed.

## Configuring via CLI

For the most detailed guide on how to configure nazara via CLI, please make use of the `help` function.

```bash
sudo nazara --help
```

When launching Nazara for the first time, a configuration file will be written at `$HOME/.config/nazara/config.toml`.
You can use CLI parameters to override your settings in the config file.

## Configuring via TOML file (recommended)

Nazara's configuration must be located in the root user's home directory at `$HOME/.config/nazara/config.toml`.

When launching Nazara for the first time, it will write a stock config file to that path. Certain parameters are
required to be configured there manually.

Aside from the NetBox system parameters, configuration via the `config.toml` also allows you to add certain
custom fields to your system information that cannot be automatically collected.
Please check the example file below for exact information about which options are possible.

~~~admonish note
Currently, configuration by config file is the proper way to use Nazara given the amount of data required to register
a machine.
We are investigating possibilities to make this less of a hassle. In the meantime, we suggest you copy-paste the
config between machines of the same type and function.
~~~

```toml
{{#include ../../../src/configuration/config_template.toml}}
```

For VMs, the `name` parameter **is required** to be able to distinguish them cleanly.

~~~ admonish tip
The `name` parameter is optional for devices. If left empty, Nazara will assume the system's hostname as the name
value for the entry. You can combine both your custom name and the machine's hostname by fixing a `@`
symbol to the end of the name value. This works on both VMs and devices.

This way a config entry like this:

```toml
[common]
name = "aurora@"
```

turns into:

```toml
aurora@linux.fritz.box
```
in the final entry.
~~~

### The `config` commands

Nazara provides you with several commands to manage your configuration files:

- `write-config`: Write a new config file or overwrite an existing one.
- `check-config`: Validate if your config is still valid.
- `view-config`: Print config to console.

The `write-config` allows you to change individual parameters, or perform a bulk
update by passing a `JSON` structure via CLI. **These options are exclusive.
Passing both is disallowed.**

~~~admonish example collapsible=false title="Example: Using `write-config` to change config parameters"
These examples show you how you can edit your config file from the command line.

```bash
# Pass arguments individually
sudo nazara write-config --uri https://netbox.sampleorg.com
```

Or in batches using JSON:

```bash
sudo nazara write-config --json '{
  "netbox": {
    "netbox_uri": "https://netbox.example.com",
    "netbox_api_token": "abcd1234"
  },
  "common": {
    "name": "test-device",
    "description": "A physical test machine",
    "comments": "Created for testing purposes",
    "status": "active"
  },
  "device": {
    "device_type": 1,
    "role": 2,
    "site": 3
  }
}'
```
~~~

*Please note that this section is still a work in progress and all information is subject to change.*

