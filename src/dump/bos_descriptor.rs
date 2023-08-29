use rusb::constants::{LIBUSB_REQUEST_GET_DESCRIPTOR, LIBUSB_ENDPOINT_IN, LIBUSB_RECIPIENT_DEVICE};
use crate::consts::{USB_DT_DEVICE_QUALIFIER, CTRL_TIMEOUT};
use super::usb2_device_capability_desc;
use super::ss_device_capability_desc;
use super::ssp_device_capability_desc;
use super::container_id_device_capability_desc;
use super::platform_device_capability_desc;
use super::billboard_device_capability_desc;
use super::billboard_alt_mode_capability_desc;
//use super::desc;
use std::fmt::Write;

use crate::consts::{USB_DC_WIRELESS_USB,
                    USB_DC_20_EXTENSION,
                    USB_DC_SUPERSPEED,
                    USB_DC_SUPERSPEEDPLUS,
                    USB_DC_CONTAINER_ID,
                    USB_DC_PLATFORM,
                    USB_DC_BILLBOARD,
                    USB_DC_BILLBOARD_ALT_MODE,
                    USB_DC_CONFIGURATION_SUMMARY};

pub fn dump(device: &rusb::DeviceHandle<rusb::Context>, mut indent: usize) -> Result<(bool, String), std::fmt::Error> {
    let mut result = false;
    let mut bos_desc: [u8; 5] = [0; 5];

    let mut output_buf = String::new();

    let ret = device.read_control(LIBUSB_ENDPOINT_IN | LIBUSB_RECIPIENT_DEVICE,
                                  LIBUSB_REQUEST_GET_DESCRIPTOR,
                                  USB_DT_DEVICE_QUALIFIER << 8,
                                  0,
                                  &mut bos_desc,
                                  CTRL_TIMEOUT);

    return if ret.is_err() {
        Ok((result, output_buf))
    } else if bos_desc[0] != 5 || bos_desc[1] != 0x0f {
        Ok((result, output_buf))
    } else {
        let bos_desc_size = bos_desc[2] as usize + ((bos_desc[3] as usize) << 8);

        writeln!(&mut output_buf, "{:indent$}Binary object Store Descriptor:", "")?;
        indent += 2;
        writeln!(&mut output_buf, "{:indent$}bLength: {}", "", bos_desc[0])?;
        writeln!(&mut output_buf, "{:indent$}bDescriptorType: {}", "", bos_desc[1])?;
        writeln!(&mut output_buf, "{:indent$}wTotalLength: {:04x}", "", bos_desc_size)?;
        writeln!(&mut output_buf, "{:indent$}bNumDeviceCaps: {}", "", bos_desc[4])?;

        if bos_desc_size <= 5 {
            if bos_desc[4] > 0 {
                writeln!(&mut output_buf, "Couldn't get device capability descriptors")?;
            }
            return Ok((result, output_buf));
        }

        let mut bos_desc = vec![0u8; bos_desc_size];

        let ret = device.read_control(LIBUSB_ENDPOINT_IN | LIBUSB_RECIPIENT_DEVICE,
                                      LIBUSB_REQUEST_GET_DESCRIPTOR,
                                      USB_DT_DEVICE_QUALIFIER << 8,
                                      0,
                                      bos_desc.as_mut_slice(),
                                      CTRL_TIMEOUT);

        if ret.is_err() {
            writeln!(&mut output_buf, "Couldn't get device capability descriptors")?;
            return Ok((result, output_buf));
        }

        let mut bos_desc_size = bos_desc_size - 5;

        let buf = &bos_desc.as_slice()[5..];
        let idx: usize = 0;

        while bos_desc_size >= 3 {
            let current = &buf[idx..];
            if current[0] < 3 {
                writeln!(&mut output_buf, "buf[0] = {}", buf[0])?;
                return Ok((result, output_buf));
            }
            match current[2] {
                USB_DC_WIRELESS_USB => (),
                USB_DC_20_EXTENSION => write!(&mut output_buf, "{}", usb2_device_capability_desc::dump(current, indent + 2)?)?,
                USB_DC_SUPERSPEED => write!(&mut output_buf, "{}", ss_device_capability_desc::dump(current, indent + 2)?)?,
                USB_DC_SUPERSPEEDPLUS => {
                    write!(&mut output_buf, "{}", ssp_device_capability_desc::dump(current, indent + 2)?)?;
                    result = true;
                }
                USB_DC_CONTAINER_ID => write!(&mut output_buf, "{}", container_id_device_capability_desc::dump(current, indent + 2)?)?,
                USB_DC_PLATFORM => write!(&mut output_buf, "{}", platform_device_capability_desc::dump(current, indent + 2)?)?,
                USB_DC_BILLBOARD => write!(&mut output_buf, "{}", billboard_device_capability_desc::dump(device, current, indent + 2)?)?,
                USB_DC_BILLBOARD_ALT_MODE => write!(&mut output_buf, "{}", billboard_alt_mode_capability_desc::dump(current, indent + 2)?)?,
                USB_DC_CONFIGURATION_SUMMARY => {
                    writeln!(&mut output_buf, "{:indent$}Configuration Summary Device Capability:", "")?;
                    //desc::dump(current, None, 2);
                }
                _ => {
                    write!(&mut output_buf, "{:indent$}** UNRECOGNIZED: ", "")?;
                    for i in 0..current[0] {
                        write!(&mut output_buf, " {:02x}", current[i as usize])?;
                    }
                    writeln!(&mut output_buf)?;
                }
            }
            bos_desc_size -= current[0] as usize;
        }
        Ok((true, output_buf))
    };
}
