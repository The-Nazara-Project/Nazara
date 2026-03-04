//! Main module stub.

use nazara::Nazara;

// ================================================
// =========NAZARA ENTRY POINT=====================
// ================================================
#[cfg(target_os = "linux")]
fn main() {
    let nazara_new = Nazara::new();
    match nazara_new {
        Ok(mut v) => {
            match v.run() {
                Err(e) => {
                    // Print error
                    println!("{}", e);
                }
                _ => {}
            }
        }
        Err(e) => {
            // Print error
            println!("{}", e);
        }
    }
}
