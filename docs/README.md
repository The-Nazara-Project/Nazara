# Nazara Documentation

In this directory you can find several guides and docs regarding Nazara.

- [Architecture Guide](./src/design/001-architecture.md)
- [Code Style Guide](./src/design/002-code_style.md)

You can build this documentation with `mdbook` locally:

> [!Important]
> For everything to function you should have the [admonish](https://github.com/tommilligan/mdbook-admonish) preprocessor installed.

```bash
cargo install mdbook mdbook-admonish
```

```bash
mdbook build docs
```

```bash
mdbook serve docs
```

# Credits

We use `mdBook-pagetoc` for the right hand side in-page scrolling solution as this is not supported by mdbook by default.
Visit [github.com/JorelAli/mdBook-pagetoc](https://github.com/JorelAli/mdBook-pagetoc) for more info.
