// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(test)]
mod tests {
    use super::super::r820t::{R82xxVariant, TUNER_INFO, R828D_TUNER_INFO};
    use crate::tuners::KNOWN_TUNERS;

    #[test]
    fn test_r82xx_variant_properties() {
        let r820t = R82xxVariant::R820T;
        let r828d = R82xxVariant::R828D;

        // Test I2C addresses
        assert_eq!(r820t.i2c_addr(), 0x34);
        assert_eq!(r828d.i2c_addr(), 0x74);

        // Test VCO power reference
        assert_eq!(r820t.vco_power_ref(), 2);
        assert_eq!(r828d.vco_power_ref(), 1);

        // Test xtal check requirement
        assert!(!r820t.needs_xtal_check());
        assert!(r828d.needs_xtal_check());

        // Test tuner info
        let r820t_info = r820t.tuner_info();
        let r828d_info = r828d.tuner_info();
        
        assert_eq!(r820t_info.id, "r820t");
        assert_eq!(r828d_info.id, "r828d");
        assert_eq!(r820t_info.i2c_addr, 0x34);
        assert_eq!(r828d_info.i2c_addr, 0x74);
    }

    #[test]
    fn test_known_tuners_includes_both() {
        assert_eq!(KNOWN_TUNERS.len(), 2);
        
        let tuner_ids: Vec<&str> = KNOWN_TUNERS.iter().map(|t| t.id).collect();
        assert!(tuner_ids.contains(&"r820t"));
        assert!(tuner_ids.contains(&"r828d"));
        
        // Verify the tuner info constants match
        assert_eq!(KNOWN_TUNERS[0], TUNER_INFO);
        assert_eq!(KNOWN_TUNERS[1], R828D_TUNER_INFO);
    }

    #[test]
    fn test_tuner_info_constants() {
        // Test R820T tuner info
        assert_eq!(TUNER_INFO.id, "r820t");
        assert_eq!(TUNER_INFO.name, "Rafael Micro R820T");
        assert_eq!(TUNER_INFO.i2c_addr, 0x34);
        assert_eq!(TUNER_INFO.check_addr, 0x00);
        assert_eq!(TUNER_INFO.check_val, 0x69);

        // Test R828D tuner info  
        assert_eq!(R828D_TUNER_INFO.id, "r828d");
        assert_eq!(R828D_TUNER_INFO.name, "Rafael Micro R828D");
        assert_eq!(R828D_TUNER_INFO.i2c_addr, 0x74);
        assert_eq!(R828D_TUNER_INFO.check_addr, 0x00);
        assert_eq!(R828D_TUNER_INFO.check_val, 0x69);
    }

    #[test]
    fn test_r82xx_variants_match_constants() {
        // Verify variant tuner_info methods match the constants
        assert_eq!(R82xxVariant::R820T.tuner_info(), TUNER_INFO);
        assert_eq!(R82xxVariant::R828D.tuner_info(), R828D_TUNER_INFO);
    }
}