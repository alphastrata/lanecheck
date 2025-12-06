use colored::*;
use prettytable::{Table, format, row};
use std::collections::BTreeMap;

pub fn display_features() {
    println!("\n{} {}", "CPU Architecture:".yellow().bold(), "AArch64");
    println!(
        "{} {}",
        "Note:".yellow().bold(),
        "CPU model and vendor are not available on AArch64 without OS-specific calls."
    );

    let mut features: BTreeMap<&str, Vec<(&str, bool, &str)>> = BTreeMap::new();

    // NEON features
    features.entry("NEON (Advanced SIMD)").or_default().push((
        "neon",
        std::arch::is_aarch64_feature_detected!("neon"),
        "128-bit vectors (4x f32, 16x u8)",
    ));
    features.entry("NEON (Advanced SIMD)").or_default().push((
        "dotprod",
        std::arch::is_aarch64_feature_detected!("dotprod"),
        "Vector Dot-Product instructions",
    ));
    features.entry("NEON (Advanced SIMD)").or_default().push((
        "fhm",
        std::arch::is_aarch64_feature_detected!("fhm"),
        "f16 multiplication instructions",
    ));

    // Scalable Vector Extension
    features.entry("SVE").or_default().push((
        "sve",
        std::arch::is_aarch64_feature_detected!("sve"),
        "Scalable Vector Extension (128-2048 bits)",
    ));
    features.entry("SVE").or_default().push((
        "sve2",
        std::arch::is_aarch64_feature_detected!("sve2"),
        "SVE2 extensions (more integer ops)",
    ));

    // Other useful features
    features.entry("Other").or_default().push((
        "crc",
        std::arch::is_aarch64_feature_detected!("crc"),
        "CRC32 instructions",
    ));
    features.entry("Other").or_default().push((
        "aes",
        std::arch::is_aarch64_feature_detected!("aes"),
        "AES instructions",
    ));

    // Display feature tables
    for (category, features) in features.iter() {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row![
            format!("{} Features", category).yellow().bold(),
            "Supported".yellow().bold(),
            "Usage".yellow().bold()
        ]);

        for (feature_name, supported, usage) in features {
            let support_status = if *supported {
                "✓".green().bold()
            } else {
                "✗".red()
            };
            table.add_row(row![feature_name, support_status, usage]);
        }

        println!();
        table.printstd();
    }

    // Recommendation section
    println!("\n{}", "SIMD Programming Recommendations:".green().bold());

    let sve = std::arch::is_aarch64_feature_detected!("sve");
    let neon = std::arch::is_aarch64_feature_detected!("neon");

    if sve {
        println!("✓ Optimal: Use SVE for scalable vectorization.");
        println!("  • Best for: High-performance computing, future-proof code.");
        println!("  • Lanes: Vector size is runtime detectable (128 to 2048 bits).");
    } else if neon {
        println!("✓ Optimal: Use NEON for 128-bit SIMD.");
        println!("  • Best for: General-purpose media processing, mobile apps.");
        println!("  • Lanes: 4 (float), 2 (double), 16 (8-bit), 8 (16-bit), 4 (32-bit)");
    } else {
        println!("⚠ No common SIMD features detected. Vectorization may not be possible.");
    }
}
