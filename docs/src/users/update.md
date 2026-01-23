# Updating a Device or VM

If you have already registered you Device or VM
in NetBox, you can update the entry using the
`nazara update` command.

For this, you need to check if your configuration is still valid and
up-to-date. Use the `write-config` command to change the info
you want to update.

~~~admonish info
This process **also requires `sudo` privileges**, please
make sure you have them before attempting an update.

This command will try to update **any information**
that has been changed in a PATCH request.
Information that has not changed will not be touched.
~~~

## Example Updating Workflow (Physical Device)

### 1. Update Configuration File

Make sure your config file at `/root/.config/nazara/config.toml`
is up to date.

If you want to change any parameter you can do so by using the
`write-config` command.

For example, we want to switch the device's site from `id=1` to `id=2`.

```bash
sudo nazara write-config --site 2
```

This will update the entry in the configuration file.

### 2. Update the Entry

Now, to update the entry, go and get the entry's ID from NetBox.
You can see it in the URL of your browser. In our case:
`http://localhost:8000/dcim/devices/57/` (device id: 57)

Then simply run

```
sudo nazara update --id 57
```

This will then update the entry for you.

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

#### IP "X.X.X.X" has not been registered in NetBox

This is a common issue with environments that use DHCP
with frequently changing IP addresses. In this case the new IP address
has not been registered with NetBox beforehand, so there is no IP entry to update.

In this case, you must register that new IP manually in NetBox
as the update process is forbidden from doing so.

Alternatively, you can delete and re-register the device.¹

We are actively working on a managed mode that forbids Nazara's
IP management alltogether in cases where you have a different source
of truth for them.

<small>
¹ Not recommended if you touch your device entries manually.
</small>
