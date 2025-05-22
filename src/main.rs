use colored::*;
use prettytable::{Table, format, row};
use raw_cpuid::CpuId;
use std::collections::BTreeMap;

fn main() {
    let cpuid = CpuId::new();
    println!("{}", "CPU SIMD Capabilities".green().bold());

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

    // Group features by category
    let mut features: BTreeMap<&str, Vec<(&str, bool, &str)>> = BTreeMap::new();

    // Basic and legacy features
    if let Some(feature_info) = cpuid.get_feature_info() {
        features.entry("Legacy").or_default().push((
            "mmx",
            feature_info.has_mmx(),
            "64-bit vector (integers)",
        ));
        features.entry("Basic").or_default().push((
            "sse",
            feature_info.has_sse(),
            "128-bit vector (4 x f32)",
        ));
        features.entry("Basic").or_default().push((
            "sse2",
            feature_info.has_sse2(),
            "128-bit vector (2 x f64)",
        ));
        features.entry("Basic").or_default().push((
            "sse3",
            feature_info.has_sse3(),
            "Additional horizontal operations",
        ));
        features.entry("Basic").or_default().push((
            "ssse3",
            feature_info.has_ssse3(),
            "Shuffle+arithmetic ops",
        ));
        features.entry("Basic").or_default().push((
            "sse4.1",
            feature_info.has_sse41(),
            "Dot product, streaming load",
        ));
        features.entry("Basic").or_default().push((
            "sse4.2",
            feature_info.has_sse42(),
            "String/text processing",
        ));
    }

    // AVX features
    if let Some(extended_features) = cpuid.get_extended_feature_info() {
        features.entry("AVX").or_default().push((
            "avx",
            extended_features.has_adx(),
            "256-bit vector (8 x f32)",
        ));
        features.entry("AVX").or_default().push((
            "avx2",
            extended_features.has_avx2(),
            "256-bit vector (integers)",
        ));
        features.entry("AVX").or_default().push((
            "fma",
            extended_features.has_avx_ifma(),
            "Fused multiply-add",
        ));

        // AVX-512 features
        features.entry("AVX-512").or_default().push((
            "avx512f",
            extended_features.has_avx512f(),
            "512-bit vector foundation",
        ));
        features.entry("AVX-512").or_default().push((
            "avx512bw",
            extended_features.has_avx512bw(),
            "Byte/word operations",
        ));
        features.entry("AVX-512").or_default().push((
            "avx512cd",
            extended_features.has_avx512cd(),
            "Conflict detection",
        ));
        features.entry("AVX-512").or_default().push((
            "avx512dq",
            extended_features.has_avx512dq(),
            "Doubleword/quadword ops",
        ));
        features.entry("AVX-512").or_default().push((
            "avx512vl",
            extended_features.has_avx512vl(),
            "Vector length extensions",
        ));
        features.entry("AVX-512").or_default().push((
            "avx512ifma",
            extended_features.has_avx512_ifma(),
            "Integer fused multiply-add",
        ));
        features.entry("AVX-512").or_default().push((
            "avx512vbmi",
            extended_features.has_avx512vbmi(),
            "Vector byte manipulation",
        ));
        features.entry("AVX-512").or_default().push((
            "avx512vpopcntdq",
            extended_features.has_avx512vpopcntdq(),
            "Population count",
        ));

        // Other SIMD-relevant features
        features.entry("Other").or_default().push((
            "bmi1",
            extended_features.has_bmi1(),
            "Bit manipulation set 1",
        ));
        features.entry("Other").or_default().push((
            "bmi2",
            extended_features.has_bmi2(),
            "Bit manipulation set 2",
        ));
        features.entry("Other").or_default().push((
            "popcnt",
            extended_features.has_avx512vpopcntdq(),
            "Population count",
        ));
    }

    // Display feature tables by category
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

    let avx512 = features
        .get("AVX-512")
        .and_then(|f| f.iter().find(|(name, _, _)| *name == "avx512f"))
        .map_or(false, |(_, s, _)| *s);
    let avx2 = features
        .get("AVX")
        .and_then(|f| f.iter().find(|(name, _, _)| *name == "avx2"))
        .map_or(false, |(_, s, _)| *s);
    let sse2 = features
        .get("Basic")
        .and_then(|f| f.iter().find(|(name, _, _)| *name == "sse2"))
        .map_or(false, |(_, s, _)| *s);

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
