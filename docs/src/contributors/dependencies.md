# Dependencies

To keep Nazara secure, lightweight, and maintainable, we follow a strict policy on introducing external dependencies.

We only accept external dependencies that meet all of these criteria:

- Are actively maintained
- Are essential to solving the given issue
- Have permissive or at least compatible licenses (MIT, BSD, LGPL, etc.)
- Are reviewed by maintainers before inclusion

```admonish info
When submitting a PR with a new dependency, please explain why it is needed and why no other alternative was suitable.
```

## Preferance Guidelines

We prefer:

- **Standard Library** functionality wherever possible
- **Zero-dependency alternatives** over large general-purpose crates
- **Lean and well-maintained** crates over obscure or overly complex ones

The goal is not to avoid dependencies at all ocsts - only to avoid unnecessary, unstable, insecure or high-maintenance ones.

~~~admonish info
In addition to manual review, we use `cargo audit` to automatically check, whether our dependencies have known
vulnerabilities.
~~~
