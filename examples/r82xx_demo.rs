// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Example demonstrating R820T and R828D tuner support
// This shows how the unified R82xx implementation handles both variants

use rtl_sdr_rs::tuners::r820t::{R820T, R82xxVariant, TUNER_INFO, R828D_TUNER_INFO};
use rtl_sdr_rs::tuners::KNOWN_TUNERS;

fn main() {
    println!("RTL-SDR R82xx Tuner Support Demo");
    println!("=================================");
    
    // Display supported tuners
    println!("\nSupported tuners:");
    for tuner in KNOWN_TUNERS.iter() {
        println!("  - {} ({})", tuner.name, tuner.id);
        println!("    I2C Address: 0x{:02x}", tuner.i2c_addr);
        println!("    Check Address: 0x{:02x}", tuner.check_addr);
        println!("    Check Value: 0x{:02x}", tuner.check_val);
        println!();
    }
    
    // Display variant-specific properties
    println!("R82xx Variant Properties:");
    println!("-------------------------");
    
    let variants = [R82xxVariant::R820T, R82xxVariant::R828D];
    
    for variant in variants.iter() {
        let info = variant.tuner_info();
        println!("{}:", info.name);
        println!("  I2C Address: 0x{:04x}", variant.i2c_addr());
        println!("  VCO Power Ref: {}", variant.vco_power_ref());
        println!("  Needs Xtal Check: {}", variant.needs_xtal_check());
        println!();
    }
    
    // Demonstrate constructor usage
    println!("Constructor Examples:");
    println!("--------------------");
    
    println!("// Create R820T tuner (default)");
    println!("let r820t = R820T::new(&mut device);");
    println!();
    
    println!("// Create R828D tuner (explicit)");
    println!("let r828d = R820T::new_r828d(&mut device);");
    println!();
    
    println!("// Create with explicit variant");
    println!("let r820t = R820T::new_with_variant(&mut device, R82xxVariant::R820T);");
    println!("let r828d = R820T::new_with_variant(&mut device, R82xxVariant::R828D);");
    println!();
    
    // Show key differences
    println!("Key Differences between R820T and R828D:");
    println!("----------------------------------------");
    println!("Feature             | R820T    | R828D");
    println!("------------------- | -------- | --------");
    println!("I2C Address         | 0x34     | 0x74");
    println!("VCO Power Reference | 2        | 1");
    println!("Crystal Check       | No       | Yes");
    println!("Register Layout     | Same     | Same");
    println!();
    
    println!("Both tuners share the same R82xx register layout and most functionality,");
    println!("with differences handled automatically based on the detected variant.");
}