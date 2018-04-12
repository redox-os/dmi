extern crate dmi;

#[test]
fn table_checksum_is_valid() {
    let s = dmi::Smbios {
        anchor: *b"_SM_",
        checksum: 0xc2,
        length: 0x1f,
        major_version: 0x02,
        minor_version: 0x07,
        max_structure_size: 0xb8,
        revision: 0x00,
        formatted: [0x00, 0x00, 0x00, 0x00, 0x00],
        inter_anchor: *b"_DMI_",
        inter_checksum: 0x3e,
        table_length: 0x0c15,
        table_address: 0x000e92f0,
        structure_count: 0x0052,
        bcd_revision: 27,
    };

    assert!(s.is_valid());
}

#[test]
fn table_checksum_is_invalid() {
    let s = dmi::Smbios {
        anchor: *b"_SM_",
        checksum: 0x00,
        length: 0x1f,
        major_version: 0x02,
        minor_version: 0x07,
        max_structure_size: 0xb8,
        revision: 0x00,
        formatted: [0x00, 0x00, 0x00, 0x00, 0x00],
        inter_anchor: *b"_DMI_",
        inter_checksum: 0x3e,
        table_length: 0x0c15,
        table_address: 0x000e92f0,
        structure_count: 0x0052,
        bcd_revision: 27,
    };

    assert!(!s.is_valid());
}
