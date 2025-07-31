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
    /// Rust couldn't convert a byte sequence to a UTF-8 string.
    UnableToParseUTF8(std::string::FromUtf8Error),
    InvalidPluginOutput(serde_json::Error),
    PluginExecution(String),
    /// Used for handling errors during file operations.
    FileOpError(std::io::Error),
    /// Indicates that no config file has been found, or it has been moved or deleted during program startup.
    NoConfigFileError(String),
    /// Indicates that a required config option is missing from the config file.
    MissingConfigOptionError(String),
    /// The Deserialization of a buffer to a type failed.
    DeserializationError(toml::de::Error),
    /// An error occured during the serialization of config parameters to a TOML value.
    SerializationError(toml::ser::Error),
    /// An error has occured while accessing data returned by NetBox.
    NetBoxApiError(String),
    /// Data returned by NetBox is incomplete.
    NetBoxMissingField(String, String),
    /// Wraps a `reqwest::Error`. Used for handling failures with requests and responses.
    Reqwest(reqwest::Error),
    /// NetBox returned a response with an unexpected code.
    UnexpectedResponse(reqwest::blocking::Response),
    /// Used to indicate the `thanix_client` version is incompatible with NetBox.
    VersionMismatch,
    /// Used to indicate that NetBox's initial response does not contain the application version.
    MissingVersion,
    /// Wraps a `serde_json::Error`. Used to handle failures with response serialization.
    JsonParse(serde_json::Error),
    NetlinkError(String),
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
                write!(f, "Failed to perform an HTTP request: {err}")
            }
            NazaraError::NetBoxApiError(err) => {
                write!(f, "NetBox API Error: {err}")
            }
            NazaraError::NetBoxMissingField(struc, field) => {
                write!(
                    f,
                    "NetBox returned incomplete data: Structure \"{struc}\" is missing field expected field \"{field}\""
                )
            }
            NazaraError::UnexpectedResponse(err) => {
                let status = err.status();
                write!(
                    f,
                    "Got an unexpected HTTP response {} from NetBox: {:?}",
                    status, err
                )
            }
            NazaraError::NetlinkError(err) => {
                write!(f, "Netlink Error: {err}")
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
                write!(f, "Error while parsing JSON: {error}")
            }
            NazaraError::Other(msg) => f.write_str(&msg),
        }
    }
}

impl From<std::io::Error> for NazaraError {
    fn from(value: std::io::Error) -> Self {
        Self::FileOpError(value)
    }
}

impl From<serde_json::Error> for NazaraError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonParse(value)
    }
}

impl From<std::string::FromUtf8Error> for NazaraError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::UnableToParseUTF8(value)
    }
}

impl From<reqwest::Error> for NazaraError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<dmidecode::InvalidEntryPointError> for NazaraError {
    fn from(value: dmidecode::InvalidEntryPointError) -> Self {
        Self::Dmi(value)
    }
}

impl From<&str> for NazaraError {
    fn from(value: &str) -> Self {
        Self::Other(value.to_owned())
    }
}

impl From<toml::de::Error> for NazaraError {
    fn from(value: toml::de::Error) -> Self {
        Self::DeserializationError(value)
    }
}
