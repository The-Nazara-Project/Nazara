//! Main module stub.

use nazara::{Nazara, failure};

// ================================================
// =========NAZARA ENTRY POINT=====================
// ================================================
#[cfg(target_os = "linux")]
fn main() {
    let nazara = Nazara::new();
    match nazara {
        Ok(mut v) => match v.run() {
            Err(e) => failure!("{}", e),
            _ => {}
        },
        Err(e) => failure!("{}", e),
    }
}
