# Contributing Workflow

## Setup

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

```admonish note
Note: If `pre-commit` fails on execution, be sure to run `cargo format` and `cargo clippy` on your code and fix any issues
raised by these tools.
```

### Changing Documentation

In case you want - or need - to do changes to this documentation, you need to install both `mdbook` and
`mdbook-admonish` via `cargo`.

Simply run these commands:

```bash
cargo install mdbook mdbook-admonish
```

To build the documentation run these commands from the repo's root:

```bash
mdbook build docs && mdbook serve docs
```

This will create a local deployment of the documentation at `localhost:3000` with automatic rebuilds upon change.

## Making changes

Once you have set up the project your can start working on your contribution.

### 1. Create a new branch for your changes

Note that working branches should have one of these prefixes.

- `feature/`: For adding new features to Nazara
- `docs/`: For changing documentation
- `ci/`: For CI/CD maintenance
- `fix/`: For bugfixes
- `dep/`: For deprecations
- `tests/`: For branches that add/change tests

```admonish example
Examples of good or bad branch names would look like this:

`feature/add-vm-creation`, rather than `add-vm`

```

### 2. Make meaningful committs

It is important to pay attention to the scope of your contribution. To this end please only make changes in one Pull Request which are related to your specific contribution.

This makes it easier for us to review your PR and to keep the commit history clean. (If you encounter something else you want to change,
which is not directly linked to your contribution, please open a PR on a separate branch for this change.)

~~~admonish hint
Please refer to our [Code Style Guide](../design/002-code_style.md) to find out how our code should be formatted and documented.
~~~

### 3. Include tests in your code 

Automated tests are essential to ensure the correctness and reliability of the codebase.
Therefore **it is required that Pull Requests, which change the existing behaviour of the codebase (e.g by adding features),
must be covered with tests** by the contributor whenever possible in the same PR as the contribution itself.
Code without tests might be rejected or take longer to process.

### 4. **Push your branch to your fork**.

### 5. **Open a PR against the main repository**.

Fill out the PR form and provide a detailed description of what your PR does and the reason or motivation behind the change.

~~~admonish hint
To make it easier for us to process your contribution, **please stick to the PR template**.
~~~

### 6. **Wait for CI to pass**.

Our CI workflows run on pushes and PRs and will check for code quality, format and vulnerabilities. They might also execute all tests they find. It is imperative that all checks
are green before a contribution is green. Please check and fix any errors the workflows find.

### 7. **Wait for review**.

That's it, now you can, if not already automatically done so, request a review by one of the repository maintainers. We will come back to you as quickly as we can.

## Pay Attention To

1. To ensure that all code is properly formatted, please run `cargo format` on you code before submitting it. `pre-commit` will tell you when your code is not properly formatted.
2. **Documentation is key**. We require, that all code contributions are properly documented not only with commit messages and meaningful PR descriptions, but also that the code itself is properly documented with docstrings. This ensures that new contributors and maintainers alike can navigate their way through the codebase. This has the added benefit that your PR can be accepted much quicker too.

```admonish important
For any other questions regarding style, please refer to the [Code Style Guide](../design/002-code_style.md).
```

## Introducing a Dependency

While we would prefer contributors not to introduce new dependencies, we acknowledge that this is not always possible.

Therefore, please refer to our [Dependency Policy](./dependencies.md) to see which dependencies we accept, and also please be ready
to explain why introducing this dependency was necessary.

## A word on vulnerabilities

If you discover a security vulnerability in our code, please inform us privately immediately according to our [Security Policy](security.md).

If you wish to fix a vulnerability, please also inform us and stand by for our green light. We would still like to investigate the vulnerability for ourselves to get an overview over the severity and scope of the problem. *Please refrain from publishing a fix until we came back to you or have published a security advisory*.

## License Disclaimer

By submitting a contribution to The Nazara Project, you agree that your contribution shall be licensed under the same
license(s) as the project at the time of contribution.

You also grant the project maintainers the right to relicense the project, including your contribution, under any
uture version of those license(s), or under any other license that is free and open source software (FOSS) and
compatible with the current license(s).
