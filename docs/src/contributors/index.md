# Contributing to Nazara

Thank you for considering contributing to Nazara.

# Set up a NetBox Test instance

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

Also, despite my best efforts, the `primary_network_interface` field is still mandatory, even though it's not in the template.
So, check your network interfaces and select the one you want to set as primary. Then add the config parameter to the `[system]`
section at the bottom and paste the name of the interface as its value.
```

Now it should work, if you have trouble setting it up, please reach out in the discussion section.

# Setup

1. **Fork the project repository** by clicking on the "Fork" button on the project's GitHub page.
This will create a copy of the repository under your GitHub account.
2. **Clone your forked repository** to your local machine using the git clone command.
This will create a local copy of the repository that you can work on.
3. **Install the project dependencies** by installing `libopenssl-dev` and `libdbus-sys` are installed on your system. Both are required by Nazara to compile.

```admonish note
The names of both of these libraries can vary depending on your distribution. The examples are for openSUSE Tumbleweed.
```

4. **Install and set up pre-commit for code quality checks**. This tool will automatically execute the `hooks` we implemented
which will check your code for formatting or styling issue before each commit.

Note: If `pre-commit` fails on execution, be sure to run `cargo format` and `cargo clippy` on your code and fix any issues
raised by these tools.

# Making changes

Once you have set up the project your can start working on your contribution.

1. **Create a new branch for your changes**. Note that working branches should have the `dev/` prefix.
2. **Make meaningful commits with clear messages for each change you make**. It is important to pay attention to the scope
of your contribution. To this end please only make changes in one Pull Request which are related to your specific contribution.
This makes it easier for us to review your PR and to keep the commit history clean. (If you encounter something else you want to change,
which is not directly linked to your contribution, please open a PR on a seperate branch for this change.)
3. **Include tests in your code**. Automated tests are essential to ensure the correctness and reliability of the codebase.
Therefore it is required that Pull Requests, which change the existing behaviour of the codebase (e.g by adding features),
must be covered with tests by the contributor in the same PR as the contribution itself.
Code without tests might be rejected or take longer to process.
4. **Push your branch to your fork**.
5. **Open a PR against the main repository**. Fill out the PR form and provide a detailed description of what your PR does and the reason or motivation behind the change.
6. **Wait for CI to pass**. Our CI workflows run on pushes and PRs and will check for code quality, format and vulnerabilities. They might also execute all tests they find. It is imperative that all checks are green before a contribution is green. Please check and fix any errors the workflows find.
7. **Wait for review**. That's it, now you can, if not already automatically done so, request a review by one of the repository maintainers. We will come back to you as quickly as we can.

# Pay Attention To

1. To ensure that all code is properly formatted, please run `cargo format` on you code before submitting it. `pre-commit` will tell you when your code is not properly formatted.
2. **Documentation is key**. We require, that all code contributions are properly documented not only with commit messages and meaningful PR descriptions, but also that the code itself is properly documented with docstrings. This ensures that new contributors and maintainers alike can navigate their way through the codebase. This has the added benefit that your PR can be accepted much quicker too.

# A word on vulnerabilities

If you discover a security vulnerability in our code, please inform us privately immediately according to our [Security Policy](./SECURITY.md).

If you wish to fix a vulnerability, please also inform us and stand by for our green light. We would still like to investigate the vulnerability for ourselves to get an overview over the severity and scope of the problem. *Please refrain from publishing a fix until we came back to you or have published a security advisory*.

# Disclaimer

By contributing to this project you agree to surrender your contribution to the Nazara Project.

Your contribution to this project will be subject to the same licensing terms as the rest of the project.

This is a standard practice in open-source projects to ensure that all contributions are compatible with the project's overall license and to maintain consistency in the project's legal framework. 

It's important for contributors to be aware of this agreement to ensure that their contributions are properly integrated into the project without any legal conflicts.
#### Thank you for your contribution!
