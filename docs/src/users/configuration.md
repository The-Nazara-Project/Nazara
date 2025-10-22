# Configuration

Nazara supports two ways of providing configuration parameters: CLI arguments and a configuration file.

Nazara accepts these parameters from you:

- `-d, --dry-run`: Print all collected information without committing it to NetBox.
- `-u, --uri <URI>`: URI to your NetBox instance.
- `-t, --token <TOKEN>`: Your API authentication token.
- `-p, --plugin <PLUGIN>`: The path to a plugin script you want to use to fill in custom fields.
- `-h, --help`: Print help.
- `-V, --version`: Print version.

Afterwards, Nazara expects one of the following operation types to be specified:

- `register`: Register a new device or vm in NetBox.
- `update --id <device_id>`: To update an existing device or vm.
- `auto`: Let Nazara decide based on the machine's name and serial number whether
          registration or update is needed.

## Configuring via CLI

Here is an example for passing these parameters on using the CLI:

```bash
sudo nazara --uri <API_URL> --token <API_TOKEN> <OPERATION> . . .
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

*Please note that this section is still a work in progress and all information is subject to change.*

