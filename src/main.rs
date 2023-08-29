mod consts;
mod utils;
mod dump;

use std::num::{IntErrorKind, ParseIntError};
use clap::{Parser};
use rusb::{UsbContext};
use usb_ids::{Device};
use dump::device;

#[derive(Clone)]
struct DeviceFilter {
    device: Option<u8>,
    bus: Option<u8>,
}

#[derive(Clone)]
struct VendorFilter {
    vendor: Option<u16>,
    product: Option<u16>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Increase verbosity (show descriptors)
    #[arg(short, long)]
    verbose: bool,

    /*
    /// Dump the physical USB device hierarchy as a tree
    #[arg(short, long)]
    tree: bool,
    */

    /// Show only devices with specified device and/or
    /// bus numbers (in decimal)
    #[arg(short = 's', verbatim_doc_comment, value_name = "[[bus]:][devnum]", value_parser = parse_device_filter)]
    device_filter: Option<DeviceFilter>,

    /// Show only devices with the specified vendor and
    /// product ID numbers (in hexadecimal)
    #[arg(short = 'd', verbatim_doc_comment, value_name = "vendor:[product]", value_parser = parse_vendor_filter)]
    vendor_filter: Option<VendorFilter>,

    /// Selects which device lsusb will examine
    #[arg(short = 'D')]
    device: Option<String>,
}

struct DeviceFilters {
    device_filter: DeviceFilter,
    vendor_filter: VendorFilter,
}

fn parse_device_filter(s: &str) -> Result<DeviceFilter, String> {
    let del = s.find(":");

    let print_parse_error = |value: Result<u8, ParseIntError>, slice: &str| -> Result<u8, String> {
        match value {
            Ok(v) => {
                Ok(v)
            }
            Err(kind) => {
                match kind.kind() {
                    IntErrorKind::Empty => {
                        Err(String::from("Empty value provided"))
                    }
                    IntErrorKind::InvalidDigit => {
                        Err(format!("`{}` isn't a valid number", slice))
                    }
                    IntErrorKind::PosOverflow => {
                        Err(format!("`{}` not in range {}-{}", slice, u8::MIN, u8::MAX))
                    },
                    _ => {
                        Err(String::from("Mysterious error"))
                    }
                }
            }
        }
    };

    let mut bus: Option<u8> = None;
    let device: Option<u8>;
    match del {
        Some(v) => {
            let bus_slice = &s[..v];
            let device_slice = &s[v + 1..];
            bus = Some(print_parse_error(bus_slice.parse::<u8>(), bus_slice)?);

            device = match device_slice.parse::<u8>() {
                Ok(v) => {
                    Some(v)
                },
                Err(e) => {
                    match e.kind() {
                        IntErrorKind::Empty => {
                            None
                        },
                        _ => {
                            print_parse_error(Err(e), device_slice)?;
                            None
                        }
                    }
                }
            };
        }
        None => {
            device = Some(print_parse_error(s.parse::<u8>(), s)?);
        }
    }

    Ok(DeviceFilter {
        bus,
        device,
    })
}

fn parse_vendor_filter(s: &str) -> Result<VendorFilter, String> {
    let print_parse_error = |value: ParseIntError, slice: &str| -> Result<Option<u16>, String> {
        match value.kind() {
            IntErrorKind::Empty => {
                Ok(None)
            }
            IntErrorKind::InvalidDigit => {
                Err(format!("`{}` isn't a valid hex number", slice))
            }
            IntErrorKind::PosOverflow => {
                Err(format!("`{}` not in range {}-{}", slice, u16::MIN, u16::MAX))
            },
            _ => {
                Err(String::from("Mysterious error"))
            }
        }
    };

    let del = s.find(":");

    let vendor: Option<u16>;
    let product: Option<u16>;
    match del {
        Some(v) => {
            let vendor_slice = &s[..v];
            let product_slice = &s[v + 1..];
            vendor = match u16::from_str_radix(vendor_slice, 16) {
                Ok(v) => {
                    Some(v)
                },
                Err(e) => {
                    print_parse_error(e, vendor_slice)?
                }
            };
            product = match u16::from_str_radix(product_slice, 16) {
                Ok(v) => {
                    Some(v)
                },
                Err(e) => {
                    print_parse_error(e, product_slice)?
                }
            };
        }
        None => {
            return Err(format!("Is not valid vendor/product filter"));
        }
    }

    if vendor.is_none() && product.is_none() {
        return Err(String::from("Vendor or product not provided"));
    }

    Ok(VendorFilter {
        vendor,
        product,
    })
}

fn list_devices(ctx: &rusb::Context, filter: DeviceFilters, verbose: bool) -> Result<(), rusb::Error> {
    let devices = ctx.devices()?;

    for device in devices.iter() {
        let bus_num = device.bus_number();
        let device_address = device.address();

        if filter.device_filter.bus.is_some_and(|x| x != bus_num) ||
            filter.device_filter.device.is_some_and(|x| x != device_address) {
            continue;
        }

        let descriptor = device.device_descriptor()?;

        let vendor_id = descriptor.vendor_id();
        let product_id = descriptor.product_id();

        if filter.vendor_filter.vendor.is_some_and(|x| x != vendor_id) ||
            filter.vendor_filter.product.is_some_and(|x| x != product_id) {
            continue;
        }

        if verbose {
            println!();
        }

        match Device::from_vid_pid(vendor_id, product_id) {
            Some(device) => {
                let vendor_name = device.vendor().name();
                let dev_name = device.name();
                println!("Bus {bus_num:03} Device {device_address:03}: ID {vendor_id:04x}:{product_id:04x} {vendor_name} {dev_name}");
            }
            None => {
                println!("Bus {bus_num:03} Device {device_address:03}: ID {vendor_id:04x}:{product_id:04x}");
            }
        }

        if verbose {
            match device::dump(&device) {
                Ok(s) => {
                    println!("{}", s);
                },
                Err(e) => return Err(e)
            }
        }
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let libusb_ctx = rusb::Context::new().expect("Couldn't initialize libusb; Err:");

    let result = match cli.device {
        Some(_) => {
            Ok(())
        }
        None => {
            let device_filter = match cli.device_filter {
                Some(v) => {
                    v
                },
                None => DeviceFilter{
                    device: None,
                    bus: None
                }
            };
            let vendor_filter = match cli.vendor_filter {
                Some(v) => {
                    v
                },
                None => VendorFilter{
                    vendor: None,
                    product: None
                }
            };
            list_devices(&libusb_ctx, DeviceFilters {
                device_filter,
                vendor_filter
            }, cli.verbose)
        }
    };

    match result {
        Err(e) => panic!("{}", e),
        Ok(_) => ()
    };
}