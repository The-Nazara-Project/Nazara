# Configuration

Nazara supports two ways of providing configuration parameters: CLI arguments and a configuration file.

Nazara accepts these parameters from you:

- `-d, --dry-run`: Print all collected information without committing it to NetBox.
- `-u, --uri <URI>`: URI to your NetBox instance.
- `-t, --token <TOKEN>`: Your API authentication token.
- `-n, --name <NAME>`: The name of the device.
- `-p, --plugin <PLUGIN>`: The path to a plugin script you want to use to fill in custom fields.
- `-h, --help`: Print help.
- `-V, --version`: Print version.

## Configuring via CLI

Here is an example for passing these parameters on using the CLI:

```bash
sudo nazara --uri <API_URL> --token <API_TOKEN> --name test_device
```

When launching Nazara for the first time, a configuration file will be written at `$HOME/.config/nazara/config.toml`. If you pass CLI parameters, these will be automatically
transfered into the config file as well.

## Configuring via TOML file (recommended)

Nazara's configuration must be located in the root user's home directory at `$HOME/.config/nazara/config.toml`.

When launching Nazara for the first time, it will write a stock config file to that path. Certain parameters are required to be configured there manually.
You recognize them by their line not being commented out.

Aside from the NetBox system parameters, configuration via the `config.toml` also allows you to add certain
custom fields to your system information that cannot be automatically selected. A great example would be the
`System Location` entry. To specify that, simply add the parameter under the `[system]` block in your configuration file.

~~~admonish note
Currently, configuration by config file is the proper way to use Nazara given the amount of data required to register a machine.
We are investigating possibilities to make this less of a hassle. In the meantime, we suggest you copy-paste the config between machines of the same
type and function.
~~~

```toml
{{#include ../../../src/configuration/config_template.toml}}
```

*Please note that this section is still a work in progress and all information is subject to change.*

