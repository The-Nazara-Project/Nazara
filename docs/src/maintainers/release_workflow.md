# Release Workflow

When it's time to make a new release of Nazara, this procedure should be followed.

~~~admonish info collapsible=true title="Thanix Maintainer Info"
Most of this information is also valid for maintainers of Thanix.
~~~

## 1. Update all Version numbers

Make sure to update all version numbers in the following places:

- `Cargo.toml`
- `Nazara.spec`
- `README.md` (Add to Compatibility List)
- `docs/src/intro.md` (Add to Compatibility List)

### 1.1 Version Numbers

We generally orient ourselves along the lines of semantic versioning.
The following guidelines apply for versioning:

* **Patch Versions**: Smaller changes/updates/bugfixes without updates to the API client
* **Minor Versions**: Updates to the API client without major reworks/breaking changes
* **Major Versions**: Breaking Changes with the API client **OR** completion of a large feature
  that significantly impacts Nazara's behaviour or expands scope.

~~~admonish info
In the end, decisions about versioning are made by the core maintainers.
When in doubt, [shoot an email to the project owner](mailto:tiara.dev@proton.me).
~~~

## 2. Tag New Version

Tag a new version on the latest commit on the `main` branch.

```bash
$ git tag -a v0.1.0_beta.1 -m "v0.1.0_beta.1: Feature X"
$ git push $REMOTE v0.1.0_beta.1
```

`$REMOTE` here is the name of the upstream Nazara remote. By default, when cloning upstream, it's `origin`.

## 3. Build and Publish to `crates.io`

~~~admonish info
We always release to [crates.io](https://crates.io/crates/nazara) first.
~~~

Publish the newest version of Nazara to [crates.io](https://crates.io/crates/Nazara).

```bash
$ cargo publish
```

~~~admonish warning
In order to be able to publish new versions, you must be given permissions to do so first.
If the publishing fails, please reach out to the Project Owner.
~~~

## 4. Updating Distribution Packages

-- Coming Soon --
