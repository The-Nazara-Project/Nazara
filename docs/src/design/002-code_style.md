# Nazara Code Style Guide

## General Guide

In regards to formatting, we follow the current Rust code style set by the Cargo formatter (`cargo fmt`).

The easiest way to enforce this, is using `pre-commit` to automatically format and check any code before it is committed.

> [!Note]
> This style is enforced by our CI as well. PRs will not be accepted unless the formatting issues are fixed.

## Focus on Readability

When writing code, we value readability above complexity.
