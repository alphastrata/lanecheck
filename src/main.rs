use colored::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod lib_x86;

#[cfg(target_arch = "aarch64")]
mod lib_arm;

fn main() {
    println!("{}", "CPU SIMD Capabilities".green().bold());

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    lib_x86::display_features();

    #[cfg(target_arch = "aarch64")]
    lib_arm::display_features();

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    println!("{} architecture is not supported.", std::env::consts::ARCH);
}
