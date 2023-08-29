use rusb::constants::{LIBUSB_CLASS_HUB};

use super::device_descriptor;
use super::config_descriptor;
use super::interface_descriptor;
use super::bos_descriptor;
use super::dualspeed;
use super::hub;
use super::otg;
use super::device_status;
use super::debug;

use std::fmt::Write;

pub fn dump(device: &rusb::Device<rusb::Context>) -> Result<String, rusb::Error> {
    let udev = device.open();

    let mut output_buf = String::new();

    if let Err(e) = udev {
        let _ = writeln!(&mut output_buf, "Couldn't open device, some information will be missing (Err: {})", e);
    }

    let descriptor = device.device_descriptor()?;

    let indent = 0;

    let _ = write!(&mut output_buf, "{}", device_descriptor::dump(udev.as_ref().ok(), &descriptor, indent).unwrap());

    let mut has_otg = false;

    let num_configs = descriptor.num_configurations();

    if num_configs > 0 {
        let ret = device.config_descriptor(0);

        match ret {
            Ok(config) => {
                let result = otg::dump(&config, indent);
                has_otg = has_otg || result.0;

                let _ = write!(&mut output_buf, "{}", result.1.unwrap());
            }
            Err(_) => eprintln!("Couldn't get configuration descriptor 0, some information will be missing")
        }

        for i in 0..num_configs {
            let ret = device.config_descriptor(i);

            match ret {
                Ok(config) => {
                    let config_dump = config_descriptor::dump(
                        udev.as_ref().ok(), &config, descriptor.usb_version(), indent + 2,
                    ).map_or(String::from("Couldn't dump config descriptor"), |v| v);
                    let _ = write!(&mut output_buf, "{}", config_dump);
                    for k in config.interfaces() {
                        for j in k.descriptors() {
                            let interface_dump = interface_descriptor::dump(
                                udev.as_ref().ok(), &j, indent + 4,
                            ).map_or(String::from("Couldn't dump interface descriptor"), |v| v);

                            let _ = write!(&mut output_buf, "{}", interface_dump);
                        }
                    }
                }
                Err(_) => writeln!(&mut output_buf, "Couldn't get configuration descriptor {}, some information will be missing", i).unwrap()
            }
        }
    }

    if udev.is_err() {
        return Ok(output_buf);
    }

    let mut udev = udev.unwrap();

    let mut has_ssp: bool = false;
    if descriptor.usb_version() >= rusb::Version::from_bcd(0x0201) {
        let res = bos_descriptor::dump(&udev, indent).unwrap();
        has_ssp = res.0;
        let _ = write!(&mut output_buf, "{}", res.1);
    }
    if descriptor.class_code() == LIBUSB_CLASS_HUB {
        let hub_dump = hub::dump(
            &mut udev, descriptor.protocol_code(), descriptor.usb_version(), has_ssp, indent + 1,
        ).map_or(String::from("Couldn't dump hub descriptor"), |v| v);
        let _ = write!(&mut output_buf, "{}", hub_dump);
    }
    if descriptor.usb_version() == rusb::Version::from_bcd(0x0200) {
        let dualspeed_dump = dualspeed::dump(&udev, indent)
            .map_or(String::from("Couldn't dump dualspeed descriptor"), |v| v);
        let _ = write!(&mut output_buf, "{}", dualspeed_dump);
    }

    let usb_ss = descriptor.usb_version() >= rusb::Version::from_bcd(0x0300);

    let _ = write!(&mut output_buf, "{}", device_status::dump(&udev, has_otg, usb_ss, indent).unwrap());
    let _ = write!(&mut output_buf, "{}", debug::dump(&udev, indent).unwrap());

    Ok(output_buf)
}