# Nazara Error Handling Guide

## Philosophy

Nazara follows a **"fail gracefully"** philosophy. We prefer returning descriptive errors over panicking, reserving panics only for truly catastrophic and unfixable situations.

**Core Principles:**

- **Single Source of Truth**: Error messages are defined once in `NazaraError` variants and displayed consistently
- **Immediate Feedback**: Errors are logged immediately using the `failure!` macro for user visibility
- **Clean Propagation**: Most functions use the `?` operator without logging noise
- **Context When Needed**: Optional context prefixes indicate which module or operation triggered the error

## The NazaraError Enum

All errors in Nazara are represented by the `NazaraError` enum in `src/error.rs`:

```rust
pub enum NazaraError {
    /// Something went wrong trying to parse DMI tables.
    Dmi(dmidecode::InvalidEntryPointError),
    /// Used to indicate that the collection of system data failed.
    UnableToCollectData(String),
    /// Used for handling errors during file operations.
    FileOpError(std::io::Error),
    /// Indicates that a required config option is missing from the config file.
    MissingConfigOptionError(String),
    /// An error occurred while accessing data returned by NetBox.
    NetBoxApiError(String),
    /// Expects a `String` message. Used for edge cases and general purpose error cases.
    Other(String),
    // ... other variants
}
```

### Display Trait Implementation

Every `NazaraError` variant implements `std::fmt::Display` to provide user-friendly error messages:

```rust
impl std::fmt::Display for NazaraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NazaraError::FileOpError(err) => {
                write!(f, "File operation failed: {err}")
            }
            NazaraError::NetBoxApiError(msg) => {
                write!(f, "NetBox API Error: {msg}")
            }
            NazaraError::Other(msg) => f.write_str(&msg),
            // ... other variants
        }
    }
}
```


~~~admonish important
The `Display` implementation should **never** call `failure!` or any other logging macro. It should only format the error message.
~~~

## Convenience Methods

The `NazaraError` enum provides two convenience methods for common error handling patterns:

### `fail()` - Log and Return

```rust
impl NazaraError {
    /// Log this error with failure! and return it wrapped in Err(...)
    pub fn fail<T>(self) -> NazaraResult<T> {
        failure!("{}", self);
        Err(self)
    }
}
```

**Purpose:** Combines logging and returning into a single operation.

~~~admonish example collapsible=true title="Example: Using the fail-function to log and return an error"
```rust
return NazaraError::Other("Config file contains invalid entries".to_owned()).fail();
```
~~~

### `log()` - Log with Context

```rust
impl NazaraError {
    /// Log this error with additional context prefix.
    /// Used when we need to log multiple errors but continue processing to fail in the end
    /// either with the calling function or some other higher instance.
    ///
    /// The optional context in this case refers to the module or program part that the error
    /// occurred in. For example: [DHCP-Mode].
    pub fn log(&self, context: Option<&str>) {
        if let Some(ctx) = context {
            failure!("[{}] {}", ctx, self);
            return;
        }
        failure!("{}", self);
    }
}
```

**Purpose:** Log errors with optional context when you need to accumulate multiple errors before failing.

~~~admonish example collapsible=true title="Example: Logging an error but continue processing"
```rust
// Accumulate errors, fail at the end
for validation in validations {
    if !validation.is_valid() {
        let err = NazaraError::Other(format!("Validation '{}' failed", validation.name()));
        err.log(None);
        error_count += 1;
    }
}

if error_count > 0 {
    return NazaraError::Other("Multiple validations failed".to_owned()).fail();
}
```
~~~

~~~admonish example collapsible=true title="Example: Logging an error with context"
`context` indicates where or why this error has occurred. For example when working with the `ip-mode` flags,
errors rooted in the changes of IP addresses by DHCP or something similar have the context "DHCP-Mode".
```rust
let err = NazaraError::NetBoxApiError("IPv4 not found".to_owned());
err.log(Some("DHCP-Mode"));
return Err(err);
```
~~~

## Examples

~~~admonish example collapsible=true title="Example: Logging with Return"
Use when an error is terminal and should be returned immediately.

```rust
return NazaraError::Other("Config file contains invalid entries".to_owned()).fail();
```
~~~

~~~admonish example collapsible=true title="Example: Log with Context, Then Return"

Use when the error needs additional module/operation context.

```rust
let err = NazaraError::NetBoxApiError(
    format!("IPv4 address \"{}\" was not registered in NetBox", ip)
);
err.log(Some("DHCP-Mode"));
return Err(err);
```

Or in a closure (for use with `ok_or_else`):

```rust
let ipv4_id = search_ip(client, &ip.to_string(), None)?.ok_or_else(|| {
    let err = NazaraError::NetBoxApiError(format!("IPv4 \"{}\" not found", ip));
    err.log(Some("DHCP-Mode"));
    err
})?;
```
~~~

~~~admonish example collapsible=true title="Example: Accumulate Errors, Fail Later"
Use when processing multiple items and collecting all errors before failing.

```rust
let mut config_errors = Vec::new();

for field in required_fields {
    if config.get(field).is_none() {
        let err = NazaraError::MissingConfigOptionError(field.to_string());
        err.log(None);
        config_errors.push(err);
    }
}

if !config_errors.is_empty() {
    return NazaraError::Other("Missing required config fields".to_owned()).fail();
}
```
~~~

~~~admonish example collapsible=true title="Example: Custom User-Facing Messages"
When you need to provide specific user guidance, use `failure!` directly.

```rust
failure!(
    "Tag '{}' does not exist. Use --prepare-environment to create it.",
    tag_name
);
```

This is clearer than trying to fit the guidance into a `NazaraError` variant's display message.
~~~

## When to Use What

### Use `.fail()` when:

- The error is terminal (nothing else can be done)
- The error message is sufficient (no additional context needed)
- You want clean, single-line error handling
- This is the 90% common case

### Use `.log()` when:

- You need to accumulate multiple errors before failing
- You want to add module/operation context
- The error should be logged but processing should continue
- You need to track multiple issues

### Use `failure!` directly when:

- You need a custom message with user guidance
- The message is temporary/debugging (will be replaced later)
- You're providing feedback before returning a different error

### Don't use either when:

- You can simply return the error with `?` (no logging needed)
- The caller will handle the error (low-level utilities)
- The error will be logged by the caller anyway

## Best Practices

### 1. Choose the Right Error Variant

Prefer specific variants over `Other` when possible:

```rust
// GOOD
return NazaraError::MissingConfigOptionError("netbox_uri".to_owned()).fail();

// OK (when no specific variant fits)
return NazaraError::Other("Custom error message".to_owned()).fail();
```

### 2. Include Context in Error Messages

When using `Other`, make the message descriptive:

```rust
// GOOD
return NazaraError::Other("Config file contains invalid entries".to_owned()).fail();

// BAD
return NazaraError::Other("Error".to_owned()).fail();
```

### 3. Use Context for Module Identification

When logging with context, use module or operation names:

```rust
err.log(Some("DHCP-Mode"));
err.log(Some("Config-Parser"));
```

### 4. Don't Duplicate Error Messages

The error message should be defined once—in the `Display` implementation or the error construction:

```rust
// GOOD
let msg = "Config file contains invalid entries".to_owned();
failure!("{}", msg);
return Err(NazaraError::Other(msg));

// REDUNDANT (duplicates the message)
failure!("Config file contains invalid entries");
return NazaraError::Other("Config file contains invalid entries".to_owned()).fail();
```

### 5. Let Errors Propagate

For low-level utility functions, just return errors. Let the caller decide whether to log:

```rust
// In a low-level utility
pub fn read_config_file() -> NazaraResult<ConfigData> {
    let mut file = File::open(get_config_path(true))?;  // Just propagate
    // ...
}

// In the calling function
let config = read_config_file()
    .map_err(|e| e.log(None))?;  // Log here
```

## Error Flow

```
[Deep Function] → Creates/Returns NazaraError
        ↓ (via ? operator)
[Middle Function] → Propagates with ?
        ↓ (via ? operator)
[Application Logic] → Logs with .fail() or .log()
        ↓
[main.rs] → Catches error, logs with failure!
        ↓
[stderr] → User sees formatted error message
```

## Example: Complete Error Handling Flow

Here's how error handling works in a real scenario:

```rust
// 1. Low-level function (no logging, just propagation)
pub fn read_config_file() -> NazaraResult<ConfigData> {
    let mut file = File::open(get_config_path(true))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    toml::from_str(&contents).map_err(NazaraError::DeserializationError)
}

// 2. Mid-level function (propagates errors)
pub fn validate_config(config: &ConfigData) -> NazaraResult<()> {
    if config.netbox_uri.is_empty() {
        return Err(NazaraError::MissingConfigOptionError("netbox_uri".to_owned()));
    }
    Ok(())
}

// 3. Application logic (logs and returns)
pub fn setup_configuration() -> NazaraResult<ConfigData> {
    let config = read_config_file()?;
    validate_config(&config)?;
    
    if config.is_invalid() {
        return NazaraError::Other("Config validation failed".to_owned()).fail();
    }
    
    Ok(config)
}

// 4. Main entry point (logs final error)
fn main() {
    match nazara::run() {
        Ok(_) => success!("All done!"),
        Err(e) => failure!("{}", e),  // Uses failure! for consistency
    }
}
```

Remember: **Errors should be logged once, at the appropriate level, with sufficient context.**
