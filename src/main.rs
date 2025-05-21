use colored::*;
use prettytable::{Table, format, row};
use raw_cpuid::CpuId;
use std::collections::BTreeMap;

fn main() {
    let cpuid = CpuId::new();
    println!("{}", "CPU SIMD Capabilities Detector".green().bold());

    // Display processor info
    if let Some(vendor_info) = cpuid.get_vendor_info() {
        println!(
            "\n{} {}",
            "CPU Vendor:".yellow().bold(),
            vendor_info.as_str()
        );
    }

    if let Some(processor_brand) = cpuid.get_processor_brand_string() {
        println!(
            "{} {}",
            "CPU Model:".yellow().bold(),
            processor_brand.as_str()
        );
    }

    // Create SIMD features table
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row![
        "Category".yellow().bold(),
        "Feature".yellow().bold(),
        "Supported".yellow().bold(),
        "Usage".yellow().bold()
    ]);

    let mut features: BTreeMap<&str, (&str, bool, &str)> = BTreeMap::new();

    // Basic and legacy features
    if let Some(feature_info) = cpuid.get_feature_info() {
        features.insert(
            "mmx",
            ("Legacy", feature_info.has_mmx(), "64-bit vector (integers)"),
        );
        features.insert(
            "sse",
            ("Basic", feature_info.has_sse(), "128-bit vector (4 x f32)"),
        );
        features.insert(
            "sse2",
            ("Basic", feature_info.has_sse2(), "128-bit vector (2 x f64)"),
        );
        features.insert(
            "sse3",
            (
                "Basic",
                feature_info.has_sse3(),
                "Additional horizontal operations",
            ),
        );
        features.insert(
            "ssse3",
            ("Basic", feature_info.has_ssse3(), "Shuffle+arithmetic ops"),
        );
        features.insert(
            "sse4.1",
            (
                "Basic",
                feature_info.has_sse41(),
                "Dot product, streaming load",
            ),
        );
        features.insert(
            "sse4.2",
            ("Basic", feature_info.has_sse42(), "String/text processing"),
        );
    }

    // AVX features
    if let Some(extended_features) = cpuid.get_extended_feature_info() {
        features.insert(
            "avx",
            (
                "AVX",
                extended_features.has_adx(),
                "256-bit vector (8 x f32)",
            ),
        );
        features.insert(
            "avx2",
            (
                "AVX",
                extended_features.has_avx2(),
                "256-bit vector (integers)",
            ),
        );
        features.insert(
            "fma",
            (
                "AVX",
                extended_features.has_avx_ifma(),
                "Fused multiply-add",
            ),
        );

        // AVX-512 features
        features.insert(
            "avx512f",
            (
                "AVX-512",
                extended_features.has_avx512f(),
                "512-bit vector foundation",
            ),
        );
        features.insert(
            "avx512bw",
            (
                "AVX-512",
                extended_features.has_avx512bw(),
                "Byte/word operations",
            ),
        );
        features.insert(
            "avx512cd",
            (
                "AVX-512",
                extended_features.has_avx512cd(),
                "Conflict detection",
            ),
        );
        features.insert(
            "avx512dq",
            (
                "AVX-512",
                extended_features.has_avx512dq(),
                "Doubleword/quadword ops",
            ),
        );
        features.insert(
            "avx512vl",
            (
                "AVX-512",
                extended_features.has_avx512vl(),
                "Vector length extensions",
            ),
        );
        features.insert(
            "avx512ifma",
            (
                "AVX-512",
                extended_features.has_avx512_ifma(),
                "Integer fused multiply-add",
            ),
        );
        features.insert(
            "avx512vbmi",
            (
                "AVX-512",
                extended_features.has_avx512vbmi(),
                "Vector byte manipulation",
            ),
        );
        features.insert(
            "avx512vpopcntdq",
            (
                "AVX-512",
                extended_features.has_avx512vpopcntdq(),
                "Population count",
            ),
        );

        // Other SIMD-relevant features
        features.insert(
            "bmi1",
            (
                "Other",
                extended_features.has_bmi1(),
                "Bit manipulation set 1",
            ),
        );
        features.insert(
            "bmi2",
            (
                "Other",
                extended_features.has_bmi2(),
                "Bit manipulation set 2",
            ),
        );
        features.insert(
            "popcnt",
            (
                "Other",
                extended_features.has_avx512vpopcntdq(),
                "Population count",
            ),
        );
    }

    // Display feature table
    let mut last_category = "";
    for (feature_name, (category, supported, usage)) in features.iter() {
        if *category != last_category {
            table.add_row(row![category.to_string().bold(), "", "", ""]);
            last_category = category;
        }

        let support_status = if *supported {
            "✓".green().bold()
        } else {
            "✗".red()
        };

        table.add_row(row!["", feature_name, support_status, usage]);
    }

    table.printstd();

    // Recommendation section
    println!("\n{}", "SIMD Programming Recommendations:".green().bold());

    let avx512 = features.get("avx512f").map_or(false, |&(_, s, _)| s);
    let avx2 = features.get("avx2").map_or(false, |&(_, s, _)| s);
    let sse2 = features.get("sse2").map_or(false, |&(_, s, _)| s);

    if avx512 {
        println!("✓ Optimal: Use 512-bit vectors (16 x f32, 8 x f64)");
        println!("  • Best for: Large data processing, scientific computing");
        println!(
            "  • Lanes: 16 (float), 8 (double), 64 (8-bit), 32 (16-bit), 16 (32-bit), 8 (64-bit)"
        );
    } else if avx2 {
        println!("✓ Optimal: Use 256-bit vectors (8 x f32, 4 x f64)");
        println!("  • Best for: Graphics, physics, machine learning");
        println!(
            "  • Lanes: 8 (float), 4 (double), 32 (8-bit), 16 (16-bit), 8 (32-bit), 4 (64-bit)"
        );
    } else if sse2 {
        println!("✓ Optimal: Use 128-bit vectors (4 x f32, 2 x f64)");
        println!("  • Best for: Basic vector operations");
        println!(
            "  • Lanes: 4 (float), 2 (double), 16 (8-bit), 8 (16-bit), 4 (32-bit), 2 (64-bit)"
        );
    } else {
        println!("⚠ Limited SIMD support. Consider scalar operations.");
    }

    // Check for masked operations support
    if avx512 {
        println!("\n{}", "Mask Operations:".green().bold());
        println!("✓ This CPU supports AVX-512 mask registers and masked operations");
        println!("  • Use mask registers (k0-k7) for branchless programming");
        println!("  • Conditional execution without branches via masking");
        println!("  • Zero or merge masking modes available");
    } else if avx2 {
        println!("\n{}", "Mask Operations:".green().bold());
        println!("✓ Use AVX2 blend/select operations with predicate masks");
        println!("  • Branchless programming via comparison + blend operations");
    }
}
