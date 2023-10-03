#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};
use plain::Plain;
use core::num::Wrapping;

pub trait TableKind: Plain {
    const KIND: u8;
}

#[repr(packed)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Smbios {
    pub anchor: [u8; 4],
    pub checksum: u8,
    pub length: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub max_structure_size: u16,
    pub revision: u8,
    pub formatted: [u8; 5],
    pub inter_anchor: [u8; 5],
    pub inter_checksum: u8,
    pub table_length: u16,
    pub table_address: u32,
    pub structure_count: u16,
    pub bcd_revision: u8
}

unsafe impl Plain for Smbios {}

impl Smbios {
    pub fn is_valid(&self) -> bool {
        let mut sum: Wrapping<u8> = self.anchor.iter().map(|x| Wrapping(*x)).sum();
        sum += self.checksum;
        sum += self.length;
        sum += self.major_version;
        sum += self.minor_version;
        sum += self.max_structure_size as u8;
        sum += self.revision;
        sum += self.formatted.iter().sum::<u8>();
        sum.0 == 0
    }
}

#[repr(packed)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Smbios3 {
    pub anchor: [u8; 5],
    pub checksum: u8,
    pub length: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub docrev: u8,
    pub revision: u8,
    _reserved: u8,
    pub table_length: u32,
    pub table_address: u64,
}

unsafe impl Plain for Smbios3 {}

impl Smbios3 {
    pub fn is_valid(&self) -> bool {
        //TODO: Checksum
        self.anchor == *b"_SM3_"
    }
}

#[repr(packed)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Header {
    pub kind: u8,
    pub len: u8,
    pub handle: u16
}

unsafe impl Plain for Header {}

#[derive(Clone)]
pub struct Table {
    pub header: Header,
    pub data: Vec<u8>,
    pub strings: Vec<String>
}

impl Table {
    pub fn get<T: TableKind>(&self) -> Option<&T> {
        if self.header.kind == T::KIND {
            T::from_bytes(&self.data).ok()
        } else {
            None
        }
    }

    pub fn get_str(&self, index: u8) -> Option<&String> {
        if index > 0 {
            self.strings.get((index - 1) as usize)
        } else {
            None
        }
    }
}

#[repr(packed)]
#[derive(Clone, Copy, Default, Debug)]
pub struct BiosInfo {
    pub vendor: u8,
    pub version: u8,
    pub address: u16,
    pub date: u8,
    pub size: u8,
    pub characteristics: u64,
}

unsafe impl Plain for BiosInfo {}

impl TableKind for BiosInfo {
    const KIND: u8 = 0;
}

#[repr(packed)]
#[derive(Default, Debug)]
pub struct SystemInfo {
    pub manufacturer: u8,
    pub name: u8,
    pub version: u8,
    pub serial: u8,
}

unsafe impl Plain for SystemInfo {}

impl TableKind for SystemInfo {
    const KIND: u8 = 1;
}

#[repr(packed)]
#[derive(Default, Debug)]
pub struct BaseBoardInfo {
    pub manufacturer: u8,
    pub product: u8,
    pub version: u8,
    pub serial: u8,
    pub asset_tag: u8,
}

unsafe impl Plain for BaseBoardInfo {}

impl TableKind for BaseBoardInfo {
    const KIND: u8 = 2;
}

#[repr(packed)]
#[derive(Default, Debug)]
pub struct ChassisInfo {
    pub manufacturer: u8,
    pub kind: u8,
    pub version: u8,
    pub serial: u8,
    pub asset_tag: u8,
}

unsafe impl Plain for ChassisInfo {}

impl TableKind for ChassisInfo {
    const KIND: u8 = 3;
}

#[repr(packed)]
#[derive(Clone, Copy, Default, Debug)]
pub struct ProcessorInfo {
    pub socket_designation: u8,
    pub processor_kind: u8,
    pub processor_family: u8,
    pub processor_manufacturer: u8,
    pub processor_id: u64,
    pub processor_version: u8,
    pub voltage: u8,
    pub external_clock: u16,
    pub max_speed: u16,
    pub current_speed: u16,
    pub status: u8,
    pub processor_upgrade: u8,
    pub l1_cache_handle: u16,
    pub l2_cache_handle: u16,
    pub l3_cache_handle: u16,
    pub serial_number: u8,
    pub asset_tag: u8,
    pub part_number: u8,
    pub core_count: u8,
    pub core_enabled: u8,
    pub thread_count: u8,
    pub processor_characteristics: u16,
    pub processor_family_2: u16,
}

unsafe impl Plain for ProcessorInfo {}

impl TableKind for ProcessorInfo {
    const KIND: u8 = 4;
}

#[repr(packed)]
#[derive(Clone, Copy, Default, Debug)]
pub struct MemoryDevice {
    pub array_handle: u16,
    pub error_information_handle: u16,
    pub total_width: u16,
    pub data_width: u16,
    pub size: u16,
    pub form_factor: u8,
    pub device_set: u8,
    pub device_locator: u8,
    pub bank_locator: u8,
    pub memory_kind: u8,
    pub kind_detail: u16,
    pub speed: u16,
    pub manufacturer: u8,
    pub serial_number: u8,
    pub asset_tag: u8,
    pub part_number: u8,
    pub attributes: u8,
    pub extended_size: u32,
    pub configured_speed: u16,
    pub minimum_voltage: u16,
    pub maximum_voltage: u16,
    pub configured_voltage: u16,
}

unsafe impl Plain for MemoryDevice {}

impl TableKind for MemoryDevice {
    const KIND: u8 = 17;
}

pub fn tables(data: &[u8]) -> Vec<Table> {
    let mut tables = Vec::new();

    let mut i = 0;
    while i < data.len() {
        // Read header
        let mut header = Header::default();
        {
            let bytes = unsafe { plain::as_mut_bytes(&mut header) };

            let mut j = 0;
            while i < data.len() && j < bytes.len() {
                bytes[j] = data[i];
                i += 1;
                j += 1;
            }
        }

        if header.kind == 127 {
            //println!("End header");
            break;
        }

        //println!("{:?}", header);

        // Read data
        let mut table = vec![0; header.len as usize - unsafe { plain::as_bytes(&header) }.len()];

        {
            let mut j = 0;
            while i < data.len() && j < table.len() {
                table[j] = data[i];
                i += 1;
                j += 1;
            }
        }

        // Read strings
        let mut strings = Vec::new();
        while i < data.len() {
            let mut string = String::new();
            while i < data.len() {
                let b = data[i];
                i += 1;

                if b == 0 {
                    break;
                } else {
                    string.push(b as char);
                }
            }

            if string.is_empty() && ! strings.is_empty() {
                break;
            } else {
                //println!("{}: {}", strings.len(), string);
                strings.push(string);
            }
        }

        tables.push(Table {
            header: header,
            data: table,
            strings: strings
        });
    }

    tables
}
