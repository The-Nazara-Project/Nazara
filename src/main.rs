//! Main module stub.

use nazara::Nazara;
use nazara::error::NazaraResult;

// ================================================
// =========NAZARA ENTRY POINT=====================
// ================================================
#[cfg(target_os = "linux")]
fn main() -> NazaraResult<()> {
    Nazara::new()?.run()
}
