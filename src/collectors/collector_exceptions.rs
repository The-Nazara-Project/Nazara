//! ## Collector Exception Module
//!
//! This module provides exceptions to the information collectors.
use std::error::Error;

/// This exception shall be raised whenever a Network Interface cannot be identified.
///
/// Usually because some or all parameters such as name, addr or mac_addr are missing.
pub struct InvalidNetworkInterfaceException {}
