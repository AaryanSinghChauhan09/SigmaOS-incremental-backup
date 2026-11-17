//! SigmaOS-incremental-backup — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_incremental_backup_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-incremental-backup: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-incremental-backup: ok");
}
