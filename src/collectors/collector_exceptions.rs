/*
This module provides custom exceptions to handle unforseen errors
during the system information collection process.
*/

use std::error::Error;

// This exception shall be raised whenever a Network Interface cannot be identified.
// Usually because some or all paramters such as name, addr or mac_addr are missing.
pub struct InvalidNetworkInterfaceException {}
