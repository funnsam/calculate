#[cfg(feature = "num_rational")]
use std::process::*;

fn main() {
    #[cfg(feature = "num_rational")] {
        println!("cargo::rerun-if-changed=ln_const_gen.py");

        let py = Command::new("python3")
            .arg("ln_const_gen.py")
            .output()
            .unwrap();
        std::fs::write("src/rational/ln_const.rs", py.stdout).unwrap();
    }
}
