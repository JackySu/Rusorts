use std::env;

fn main() {
    if cfg!(target_arch = "x86_64") {
        println!("cargo:warning=Enabling AVX2 and FMA for x86_64 target");
        env::set_var("RUSTFLAGS", "-C target-feature=+avx2,+fma");
    }
}