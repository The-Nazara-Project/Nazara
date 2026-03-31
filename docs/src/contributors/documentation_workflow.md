# Documenting Nazara

~~~admonish hint collapsible=true
This information is pretty much applicable to all repositories of the Nazara project.
~~~

This document will give you an overview about our documentation process. What needs to be documented,
where does it go and who is responsible for keeping documentation clean is all part of this guide.

## What needs to be documented?

Anything that has implication for people outside the immediate contributor team needs clear documentation.
This includes function behaviour users rely on, steps packagers follow to produce artifacts, and processes maintainers
use to operate or fix the project.

Specifically, document:

- **User-facing Features**: Installation steps, configuration methods and their parameters, upgrade instructions,
    error reporting check list, or Nazara's behaviour like CLI commands, etc.
- **Contributor Workflows**: Everything from code quality, to documentation or steps to contribute, how to set 
    up a dev environment or choose and issue to work on needs to be documented if the process changes.
- **Maintainer & Packaging Procedures:** Release workflow and how to update `thanix_client` in case NetBox changes
    their API again are the key points here. Also approval criteria for PRs or anything else Maintainers should know.

## Who needs to document things?

Documentation is a shared responsibility of all project contributors. Anyone who changes behaviour, packaging, or
operations must update relevant documentation as part of the same change. This keeps docs accurate and prevents
silent divergence between code and documentation.

Authors should include any relevant documentation updates in their contribution or link to a separate
PR that include said updates.

~~~admonish important title="Important for Maintainers" collapsible=true
In the event that a contributor links their documentaion changes in a separate PR, both PRs are to be
treated as one.

Meaning: If the code PR is approved but the docs PR needs changes, neither one is to be merged and vise versa.
~~~

Cosmetic changes or strictly code internal ones (like code cleanup, performance improvements, etc.) are generally
not covered by the duty to document unless explicitly told otherwise by a maintainer or such a time
where architectual decision forms (ADFs) are implemented.

## Formatting and Style

Write documentation that is both readable and actionable. Aim for clear, direct sentences and break
long topics into smaller pages using headings.

Use examples liberally: A short snipped demonstrating the most common use case is a good actionable example.

## The Review Process

Any changes to documentation will be reviewed by maintainers for the quality criteria mentioned below.
Only after the review is successfuly will the changed be merged. The review validates:

- **Technical accuracy and sufficient depth:** The document matches implemented behaviour and guides are executable.
- **Completeness:** All affected changes are documented.
- **Readability:** The documentation is free of typos or grammatical errors.
- **Style and Inclusivity**: Language follows the project's expectations and quality criteria,
  all images have appropriate alt text descriptions for people using screen readers.

## Quality Criteria

All documentation should have a professional and inclusive tone. Gender-neutral language (like they/them or role
based nouns like "maintainer" or "developer") should be used. Idioms or jokes should be avoided alongside
terms that may be interpreted offensively.

Documentation should go into approriate detail for the intended target group (i.e. Users or Developers) and
not omit any details or become vague when talking about specific topics.

If a feature is not yet implemented or there are currently known bugs or Issues, a "Known Issues" section
should be appended at the bottom of the document listing these issues and their corresponding Codeberg Issue
if applicable.

Include expected output or other steps which the reader can use to verfiy operation outcomes in any instructional guide.

## Version Compatiblity

Currently, the documentation cannot be hosted for individual specific versions of Nazara. Instead,
the currently public documentation of Nazara is always valid for the latest release.

Our CI will check for newly created releases and build all changes to the `docs/` directory and publish them here.

## Examples

~~~admonish example title="Example: Gender Inclusive Language" collapsible=true
Instead of
```
When a user finds a bug, he should open a. issue.
```
write
```
If a user discovers a bug, they should open an issue.
```
~~~

~~~admonish example title="Example: Documenting Breaking Changes" collapsible=true

Breaking changes should be documented at the top of the file in a warning box.
```
~~~admonish warning title="v0.2.0: Breaking Change in Configuration"
Since NetBox `v4.5.x`, NetBox requires a different format of API token in the authentication header.
This applies for any new `v2` token. `v1` tokens are unaffected by this.

In your config file, instead of:

    netbox_api_token = "$TOKEN"

you have to write:

    netbox_api_token = "nbt_$KEY.$TOKEN"

Codeberg Issue Refernce: [#179](https://codeberg.org/nazara-project/Nazara/issues/179)
```

The Codeberg Issue Reference can also be a link to a commit that introduced a breaking change between Nazara releases.
~~~

~~~admonish example title="Example: Documenting Known Issues" collapsible=true
**This should be used sparingly** as we should focus on fixing these issues instead of documenting them.

However, for anything that a user can face during the use of Nazara or its side projects, which is expected but currently
not fixed we should document it at the relevant passage with in a similar style as used above while referencing
the corresponding issue where possible.

```
~~~admonish warning title="v0.2.0: Breaking Change in Configuration"

You could face X here, if so, please do Y. We are working on a fix.

Codeberg Issue Refernce: [#000](LINK)
```
~~~
