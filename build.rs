use std::env;

fn main() {
    if cfg!(target_arch = "x86_64") {
        env::set_var("RUSTFLAGS", "-C target-feature=+avx2,+fma");
    }
}