#[derive(Debug)]
pub enum NazaraError {
    /// Something went wrong trying to parse DMI tables.
    Dmi(dmidecode::InvalidEntryPointError),
    /// Used to indicate that the collection of system data failed.
    UnableToCollectData(String),
    /// Used to indicate that one of the collected NWIs might be malformed or invalid.
    InvalidNetworkInterface(String),
    /// Used in case the NWI collector crate cannot find any interfaces.
    NoNetworkInterfaces(String),
    UnableToParseUTF8(std::string::FromUtf8Error),
    InvalidPluginOutput(serde_json::Error),
    PluginExecution(String),
    /// Used for handling errors during file operations.
    FileOpError(std::io::Error),
    /// Indicates that no config file has been found, or it has been moved or deleted during program startup.
    NoConfigFileError(String),
    /// Indicates that a required config option is missing from the config file.
    MissingConfigOptionError(String),
    /// Indicates an error during deserialization of the TOML config file.
    DeserializationError(toml::de::Error),
    /// Indicates an error during Serialization of config parameters to a TOML value.
    SerializationError(toml::ser::Error),
    /// Wraps a `reqwest::Error`. Used for handling failures with requests and responses.
    Reqwest(reqwest::Error),
    /// Used to indicate the `thanix_client` version is incompatible with NetBox.
    VersionMismatch,
    /// Used to indicate that NetBox's initial response does not contain the application version.
    MissingVersion,
    /// Wraps a `serde_json::Error`. Used to handle failures with response serialization.
    JsonParse(serde_json::Error),
    /// Expects a `String` message. Used for edge cases and general purpose error cases.
    Other(String),
}

pub type NazaraResult<T> = Result<T, NazaraError>;

impl std::fmt::Display for NazaraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NazaraError::Dmi(err) => {
                write!(f, "DMI Error: {err}")
            }
            NazaraError::UnableToCollectData(err) => {
                write!(f, "Collector Error: {err}")
            }
            NazaraError::InvalidNetworkInterface(err) => {
                write!(f, "Network Collector Error: {err}")
            }
            NazaraError::NoNetworkInterfaces(err) => {
                write!(f, "Network Collector Error: {err}")
            }
            NazaraError::UnableToParseUTF8(err) => {
                write!(f, "Unable to parse stdout from UTF8 to string: {err}")
            }
            NazaraError::InvalidPluginOutput(err) => {
                write!(f, "Plugin returned invalid JSON: {err}")
            }
            NazaraError::PluginExecution(err) => {
                write!(f, "Plugin execution failed: {err}")
            }
            NazaraError::FileOpError(err) => {
                write!(f, "File operation failed: {err}")
            }
            NazaraError::NoConfigFileError(err) => {
                write!(f, "No config file found: {err}")
            }
            NazaraError::MissingConfigOptionError(err) => {
                write!(f, "Missing required config parameter: {err}")
            }
            NazaraError::DeserializationError(err) => {
                write!(f, "Invalid config file: {err}")
            }
            NazaraError::SerializationError(err) => {
                write!(f, "Serialization error: {err}")
            }
            NazaraError::Reqwest(err) => {
                write!(f, "")
            }
            NazaraError::VersionMismatch => {
                write!(
                    f,
                    "Client version incompatible with NetBox version! Use client v1.x for NetBox v3.6.x and above, and v2.x for NetBox 4.x.",
                )
            }
            NazaraError::MissingVersion => {
                write!(
                    f,
                    "NetBox version missing from response. Please check your installation.",
                )
            }
            NazaraError::JsonParse(error) => {
                write!(f, "")
            }
            NazaraError::Other(msg) => f.write_str(msg),
        }
    }
}

impl From<std::io::Error> for NazaraError {
    fn from(value: std::io::Error) -> Self {
        NazaraError::FileOpError(value)
    }
}

impl From<serde_json::Error> for NazaraError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonParse(value)
    }
}

impl From<std::string::FromUtf8Error> for NazaraError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        NazaraError::UnableToParseUTF8(value)
    }
}

impl From<reqwest::Error> for NazaraError {
    fn from(value: reqwest::Error) -> Self {
        NazaraError::Reqwest(value)
    }
}

impl From<dmidecode::InvalidEntryPointError> for NazaraError {
    fn from(value: dmidecode::InvalidEntryPointError) -> Self {
        NazaraError::Dmi(value)
    }
}

impl From<&str> for NazaraError {
    fn from(value: &str) -> Self {
        Self::Other(value.to_owned())
    }
}
