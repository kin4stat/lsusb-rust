use crate::consts::{READ_STR_TIMEOUT};
use rusb;
use std::fmt::Write;

pub fn get_guid(buf: &[u8]) -> String {
    return format!("{{{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                   buf[3],
                   buf[2],
                   buf[1],
                   buf[0],
                   buf[5],
                   buf[4],
                   buf[7],
                   buf[6],
                   buf[8],
                   buf[9],
                   buf[10],
                   buf[11],
                   buf[12],
                   buf[13],
                   buf[14],
                   buf[15],
    );
}

pub fn read_be_u32(input: &[u8]) -> u32 {
    let (int_bytes, _) = input.split_at(std::mem::size_of::<u32>());
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn read_le_u32(input: &[u8]) -> u32 {
    let (int_bytes, _) = input.split_at(std::mem::size_of::<u32>());
    u32::from_le_bytes(int_bytes.try_into().unwrap())
}

pub fn read_be_u16(input: &[u8]) -> u16 {
    let (int_bytes, _) = input.split_at(std::mem::size_of::<u16>());
    u16::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn print_with_bitflags(us: u32, values: &[String], output_buf: &mut String) -> Result<(), std::fmt::Error> {
    print_with_bitflags_with_off(us, values, 0, output_buf)
}

pub fn print_with_bitflags_with_off(us: u32, values: &[String], start_offset: u32, output_buf: &mut String) -> Result<(), std::fmt::Error> {
    for i in 0..values.len() {
        if (us & (1 << (i + start_offset as usize))) != 0 {
            write!(output_buf, "{}", values[i])?;
        }
    }

    Ok(())
}

pub fn read_device_string<F>(
    device: Option<&rusb::DeviceHandle<rusb::Context>>,
    get_desc_index: F,
) -> String where F: FnOnce() -> u8 {
    match device {
        Some(dev) =>
            match dev.read_languages(READ_STR_TIMEOUT) {
                Ok(langs) => {
                    if let Some(lang) = langs.first() {
                        dev.read_string_descriptor(*lang, get_desc_index(), READ_STR_TIMEOUT).map_or(String::new(), |res| res)
                    } else {
                        String::new()
                    }
                }
                Err(_) => String::new()
            },
        None => String::new()
    }
}

pub fn read_device_manufacturer_string(
    device: Option<&rusb::DeviceHandle<rusb::Context>>,
    device_desc: &rusb::DeviceDescriptor,
) -> String {
    match device_desc.manufacturer_string_index() {
        Some(v) => {
            read_device_string(device, || v)
        },
        None => String::new()
    }
}

pub fn read_device_product_string(
    device: Option<&rusb::DeviceHandle<rusb::Context>>,
    device_desc: &rusb::DeviceDescriptor,
) -> String {
    match device_desc.product_string_index() {
        Some(v) => {
            read_device_string(device, || v)
        },
        None => String::new()
    }
}

pub fn read_device_serial_string(
    device: Option<&rusb::DeviceHandle<rusb::Context>>,
    device_desc: &rusb::DeviceDescriptor,
) -> String {
    match device_desc.serial_number_string_index() {
        Some(v) => {
            read_device_string(device, || v)
        },
        None => String::new()
    }
}

pub fn read_configuration_description_string(
    device: Option<&rusb::DeviceHandle<rusb::Context>>,
    config: &rusb::ConfigDescriptor
) -> String {
    match config.description_string_index() {
        Some(v) => {
            read_device_string(device, || v)
        },
        None => String::new()
    }
}

pub fn read_interface_description_string(
    device: Option<&rusb::DeviceHandle<rusb::Context>>,
    desc: &rusb::InterfaceDescriptor
) -> String {
    match desc.description_string_index() {
        Some(v) => {
            read_device_string(device, || v)
        },
        None => String::new()
    }
}