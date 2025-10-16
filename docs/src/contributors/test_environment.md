# Setting up a Test Environment

In case you don't have a dedicated test instance of NetBox, you can easily set up a local instance via `docker-compose`.

1. Simply clone the [netbox-docker](https://github.com/netbox-community/netbox-docker) repository
2. Modify the `docker-compose.yml` file to the required NetBox version

```admonish note
Depending on the version of Nazara you are working with, you may need to adjust the image version number in your `docker-compose.yaml` to fit your needs.
In case you are working on a specific issue, please make sure the Nazara version is compatible with the NetBox version you are using and also make sure that we
still support that version.
```

```yml
services:
    netbox: &netbox
        image: docker.io/netboxcommunity/netbox:v4.3.3
        ...
```

and execute these commands in accordance to [netbox-docker's setup guide](https://github.com/netbox-community/netbox-docker?tab=readme-ov-file#quickstart):

```bash
git clone -b release https://github.com/netbox-community/netbox-docker.git
cd netbox-docker
tee docker-compose.override.yml <<EOF
services:
  netbox:
    ports:
      - 8000:8080
EOF
```

3. Then build the environment by running `docker compose up`
4. When the container is built, you need to create a superuser test account

```
docker compose exec netbox /opt/netbox/netbox/manage.py createsuperuser
```

Simply select a username and password of your wishes.

5. When that is done, you need to create an API Token `username > API Tokens > Add a Token` and paste it, along with the container's URL
   into the Nazara config file at `~/.nazara/config.toml`
6. After that, you need to create a few dummy fields that are sadly required to create a device via API
    - Device Type
    - Device Role
    - Manufacturer
    - Site
    (And depending on what you want to work on, replicate the custom fields you need 1:1 from your production instance.)
    
    If you want to specifiy and play around with some optional fields, you must create the objects you reference (like e.g Tenants) first.

7. After that's done, take the IDs of these objects and place it into the corresponding fields in the `~/.nazara/config.toml`

```admonish important
Currently, the generation of the config file is still a bit wonky, so if it isnt generated upon first executing nazara, copy and paste
the template from the README or `src/configuration/config_template.toml`.
```

Now it should work, if you have trouble setting it up, please reach out in the discussion section.
