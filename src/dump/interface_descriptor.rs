use crate::utils;
use super::endpoint::dump_endpoint;
use std::fmt::Write;

pub fn dump(device: Option<&rusb::DeviceHandle<rusb::Context>>, desc: &rusb::InterfaceDescriptor, mut indent: usize) -> Result<String, std::fmt::Error> {
    let istr = utils::read_interface_description_string(device, desc);

    let mut output_buf = String::new();

    writeln!(&mut output_buf, "{:indent$}Interface Descriptor:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bInterfaceNumber:    {:5}", "", desc.interface_number())?;
    writeln!(&mut output_buf, "{:indent$}bAlternateSetting:   {:5}", "", desc.setting_number())?;
    writeln!(&mut output_buf, "{:indent$}bNumEndpoints:       {:5}", "", desc.num_endpoints())?;
    writeln!(&mut output_buf, "{:indent$}bInterfaceClass:     {:5}", "", desc.class_code())?;
    writeln!(&mut output_buf, "{:indent$}bInterfaceSubClass:  {:5}", "", desc.sub_class_code())?;
    writeln!(&mut output_buf, "{:indent$}bInterfaceProtocol:  {:5}", "", desc.protocol_code())?;
    writeln!(&mut output_buf, "{:indent$}iInterface:          {:5} {}", "", desc.description_string_index().map_or(0, |v| v), istr)?;

    /*let buf = desc.extra();
    if buf.len() {
        let mut idx: usize = 0;
        let mut size = buf.len();

        while size >= 2 {
            let current = &buf[idx..];
            if current[0] < 2 {
                break;
            }

            match current[1] {
                USB_DT_CS_DEVICE | USB_DT_CS_INTERFACE => {
                    match desc.class_code() {
                        constants::LIBUSB_CLASS_AUDIO => {
                            match desc.sub_class_code() {
                                1 => {
                                    dump_audiocontrol_interface();
                                }
                                2 => {
                                    dump_audiostreaming_interface();
                                }
                                3 => {
                                    dump_midistreaming_interface();
                                }
                                _ => {}
                            }
                        }
                        constants::LIBUSB_CLASS_COMM => {}
                        USB_CLASS_VIDEO => {}
                        USB_CLASS_APPLICATION => {}
                        constants::LIBUSB_CLASS_HID => dump_hid_device(device, desc, current),
                        constants::LIBUSB_CLASS_PRINTER => {}
                        USB_CLASS_CCID => dump_ccid_device(current),
                        _ => {
                            match desc.class_code() {
                                constants::LIBUSB_CLASS_HID => dump_hid_device(device, desc, current),
                                USB_CLASS_CCID => dump_ccid_device(current),
                                0xe0 => {}
                                constants::LIBUSB_CLASS_AUDIO => match desc.sub_class_code() {
                                    2 => dump_audiostreaming_endpoint(current, desc.protocol_code());
                                    _ => ()
                                }
                                _ => match current[1] {
                                    USB_DT_OTG => {
                                        //
                                    }
                                    USB_DT_INTERFACE_ASSOCIATION => dump_association(current),
                                    _ => {
                                        writeln!(output_buf, "        ** UNRECOGNIZED: ")?;
                                        for i in 0..current[0] {
                                            print!(" {:02x}", current[i as usize]);
                                        }
                                        println!();
                                    }
                                }
                            }
                        }
                    };
                }
                _ => {}
            };

            size -= current[0];
            idx += current[0];
        }
    }*/

    for endpoint in desc.endpoint_descriptors() {
        writeln!(&mut output_buf, "{}", dump_endpoint(device,desc, &endpoint, indent)?)?;
    }

    Ok(output_buf)
}