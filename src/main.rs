//! Main module stub.

use nazara::Nazara;

// ================================================
// =========NAZARA ENTRY POINT=====================
// ================================================
#[cfg(target_os = "linux")]
fn main() {
    let nazara = Nazara::new();
    match nazara {
        Ok(mut v) => match v.run() {
            Err(e) => {
                eprintln!("{}", e);
            }
            _ => {}
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
