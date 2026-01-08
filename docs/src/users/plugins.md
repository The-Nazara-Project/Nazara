# The Plugin System


```admonish warning
This feature is highly experimental, but should work fine for simple custom fields.
```

Nazara allows you to fill out the "Custom Fields" section of your device or VM entries by using
a custom bash script.

This bash script should return your desired information as JSON compliant string.

~~~admonish example title="Using custom scripts with Nazara"

To do so, you can provide Nazara with the path to your custom script using the `--plugin` argument.

```bash
nazara --plugin ~/.config/nazara/scripts/custom_script.sh register
```

Nazara will run your script and expect to get a JSON string back, which will be parsed into a 
`HashMap<String, Value>`

Note, that we currently **only support parameters which have been specified in NetBox as string type.**
~~~

You can find an example script [here](https://codeberg.org/nazara-project/Nazara/src/branch/main/scripts/example.sh).

```admonish danger
Please make sure that your script models the custom fields exactly like they are in your NetBox
instance, as we have no way of verifying it.
```
