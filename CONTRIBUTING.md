# Contributing to Netbox Sync

Thank you for considering contributing to Netbox Sync.

To ensure a smooth collaboration, we suggest you follow the steps below:

# Setup

1. **Fork the project repository** by clicking on the "Fork" button on the project's GitHub page.
This will create a copy of the repository under your GitHub account.
2. **Clone your forked repository** to your local machine using the git clone command.
This will create a local copy of the repository that you can work on.
3. **Install the project dependencies** by following the instructions provided in the project's README file.
This will ensure that you have all the necessary tools and libraries to build and run the project.
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

#### Thank you for your contribution!