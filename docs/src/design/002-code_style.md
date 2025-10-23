# Nazara Code Style Guide

## General Guide

In regards to formatting, we follow the current Rust code style set by the Cargo formatter (`cargo fmt`).

The easiest way to enforce this, is using `pre-commit` to automatically format and check any code before it is committed.

```admonish info 
This style is enforced by our CI as well. PRs will not be accepted unless the formatting issues are fixed.
```

## Panics

We heavily discourage the use of the `panic!()` macro when aborting the process. Instead, we prefer to handle an error
and abort gracefully with a helpful error message and error code.

We only want to panic, when the issue we encounter is **both catastrophic and unfixable by the user**.

~~~admonish example
These are szenarios in which we **don't want to panic**:

- When input or config parameters are missing
- When connection to NetBox fails
- When an API request returns a different error code that Ok

While we want to panic in these cases:

- When filesystem operations fail
- When Nazara is not run as root
~~~

Instead of panicking, we write custom error types to wrap errors of certain functions and include a well formatted error message alongside the error.
For an exampel on how that looks, please check and of the `error.rs` files found in any of Nazara's packages.

~~~admonish example collapsible=true title="Example:`src/publisher/error.rs`"
In this example, you can see how we write our custom error types and are able to wrap errors that we receive from other functions
in order to escalate them properly.
```rust
{{#include ../../../src/error.rs}}
```
~~~

## Focus on Readability

When writing code, we value readability above complexity.

Rust offers a lot of possibilities when it comes to reducing code down into a one-line statement. And that's nice,
and sometimes even necessary.

However, we want to take a **readability first** approach with our codebase. Both to aid maintainability and
increase accessibility for newcomers.

Of course, sometimes using Rust's more "advanced" features will drastically improve performance. So a balance has
to be struck between readability and performance. It is very hard to define a solid policy for this, so 
this is a decision every developer and contributor has to do for themselves and - when necessary - engage
in discussion with maintainers, devs and other contributors about their approach and possible alternatives.

## Documenting Code

For a smilar reason, we encourage devs to properly document their contributions.
This includes but is not limited to:

- Using inline comments to explain possibly hard to understand syntax
- Using Rust's powerful docstring feature to properly document functions, structs and modules
- Adding to this documentation when applicable
- Filling out Issue and PR templates appropriately to aid maintainers review their changes

The following examples will show you an ideal docstring style.

~~~admonish example title="Example: Documenting Functions" collapsible=true
**Documenting Functions:**
```rust
/// This function does X.
///
/// This function does X by doing Y using Z. (Detailed explanation optional)
///
/// # Parameters
/// * `arg: str` - A string argument to process
///
/// # Returns
/// * `Ok(str)` - Returns A, if ...
/// * `Err` - Returns an `ErrType`
pub fn foo(arg: str) -> Result<str, Err> {
 // ...
}
```
While not universally used by all projects, we use `# Parameters` and `# Returns`
sections, especially for larger functions.

If a function does not take arguments, the `# Parameters` section can be omitted.
However, if the function does not return (`!` type) - or returns `()`, this has to be indicated
in the `# Returns` section.

Other sections we use are:

* `# Aborts` - If the function aborts the program. (E.g When input parameters are missing)
* `# Panics` - If the function can cause a `panic!()`.

For both of these sections, list all - or at least the most common - reasons this behaviour can
occur. This can help debugging immensely.

```rust
/// This function does X.
///
/// # Paramters
/// - `path: &str` - The path to a file
///
/// # Returns
/// - `String` - The contents of the file.
///
/// # Aborts
/// This function will exit the process if the file cannot be found.
pub fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("[Error] File '{}' does not exist: {}", path, err);
            process::exit(1);
        }
    }
}
```
~~~

~~~ admonish example title="Example: Documenting Structs" collapsible=true
**Documenting Structs:**
```rust
/// Information about a Person.
pub struct Person {
    /// The name of the Person.
    pub name: String,
    /// The age of the Person.
    pub age: i64,
}
```
It is encouraged to briefly document every field of your struct, whether they are `pub`
or not does not matter.
~~~

~~~ admonish example title="Example: Documenting Modules" collapsible=true
**Documenting Modules:**
```rust
//! This module handles X.
```
You can go into depth here about what a module does. This is encouraged for larger modules.
~~~

~~~admonish hint
Please be aware, that a one-liner docstring above a function **will not suffice**.
Maintainers may ask you to stick to the given format for your contribution.
~~~

We are aware that to some of you, this feels like cluttering the code files.
However, we believe that properly documenting our code is the key to providing a more
inclusive and more maintainable development experience for all.

## Terminal Output/User Interface

When it is necessary to inform the user about a process, we want to make it short, but as expressive as possible.
For this, the following styles apply:

* `Process X has been started...` - Basic white text indicates the current process.
* `\x1b[32m[success]\x1b[0m Something has succeeded` - Green colored `[success]` prefix before the message.
* `\x1b[31m[error]\x1b[0m Something has failed!` - Red colored `[error]` prefix before error message. *This should
automatically be added by our custom error types when they are given a error message.*
* `\ẋ1b[36m[info]\x1b[0m Information level message.` - Light blue colored `[info]` prefix.
* `\x1b[33m[warning]\x1b[0m Something went wrong, but we can continue...` - Yellow colored `[warning]` prefix.

To unify this coloring we have implemented several macros to be used for these status messages. These apply
formatting and colors automatically and disable colors when the host's terminal does not support it.

The macros are called `success!`, `warn!`, `failure!`, `info!` you can find a usage example below.

~~~admonish example title="Example: Status Message Macros" collapsible=true
```rust
match some_func(x) => {
    Ok(_) => {
        success!("This worked!");
    },
    Err(e) => {
        failure!("An error occured: {}", e);
        // Handle the error.
    }
}
```
~~~
