# Becoming a Packager

Projects like Nazara live from people who volunteer to maintain packages for their distributions.

System settings or security policies may prohibit package managers like `cargo`, `pip` or `npm`
from installing executables on a system. Or maybe company policies require that all software
must only come from OS repos, where they can be monitored and audited.

That's why we need you to help to bring Nazara to new platforms.

If you have a distribution that we do not provide a package for, please follow these steps.

---

1. Open a new discussion stating what you want to package for.

2. Clone the repo and add yourself to the PACKAGERS.md in the project root

```admonish example
Your example entry can look like this:

|Distro|Name/Nickname|Contact Info (optional)|Notes|
|-|-|-|-|
|Arch Linux|`@urgithub`|urmail@domain.com|AUR maintainer|
|NixOS|Sam| none (via GitHub issues)|Maintains Nix flake|
|Fedora|Bob|@bob:somematrix.org|Fedora packaging|

If you prefer not to share contact details, that's totally fine.
We will handle requests regarding your package as GitHub issues.
```

```admonish danger
Outside contributors: Please be aware that we will list your package as `unofficial`
until you become part of the organization.

If you stop maintaining your package, we reserve the right to remove it from our list of supported distributions.
```

3. Open a PR with your changes, and link your request in the PR.

4. A maintainer will eventually approve or deny your request.
