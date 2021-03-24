#[cfg(not(feature = "dox"))]
extern crate system_deps;

#[cfg(not(feature = "dox"))]
use std::process;

#[cfg(feature = "dox")]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(feature = "dox"))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        let _ = eprintln!("{}", s);
        process::exit(1);
    }
}
