use std::{env, fs};

fn main() {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "/sys/firmware/dmi/tables/DMI".to_string());
    let data = fs::read(path).unwrap();
    for table in dmi::tables(&data) {
        if let Some(info) = table.get::<dmi::BiosInfo>() {
            println!("{:?}", info);
        } else if let Some(info) = table.get::<dmi::SystemInfo>() {
            println!("{:?}", info);
        } else if let Some(info) = table.get::<dmi::BaseBoardInfo>() {
            println!("{:?}", info);
        } else if let Some(info) = table.get::<dmi::ChassisInfo>() {
            println!("{:?}", info);
        } else if let Some(info) = table.get::<dmi::ProcessorInfo>() {
            println!("{:?}", info);
        } else if let Some(info) = table.get::<dmi::MemoryDevice>() {
            println!("{:?}", info);
        } else {
            println!("Unknown table: {}", table.header.kind);
            continue;
        }
        println!("    strings: {:?}", table.strings);
    }
}
