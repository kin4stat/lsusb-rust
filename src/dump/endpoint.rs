use rusb::constants::{LIBUSB_CLASS_COMM, LIBUSB_CLASS_DATA, LIBUSB_CLASS_MASS_STORAGE};
use crate::consts::{USB_DT_CS_ENDPOINT,
                    USB_DT_CS_INTERFACE,
                    USB_DT_INTERFACE_ASSOCIATION,
                    USB_DT_CS_DEVICE,
                    USB_DT_OTG,
                    USB_DT_SS_ENDPOINT_COMP,
                    USB_CLASS_CCID};
use super::audiostreaming_endpoint;
use super::midistreaming_endpoint;
use super::pipe_desc;
use super::ccid_device;
use super::association;
use std::fmt::Write;

pub fn dump_endpoint(device: Option<&rusb::DeviceHandle<rusb::Context>>, interface: &rusb::InterfaceDescriptor, endpoint: &rusb::EndpointDescriptor, mut indent: usize) -> Result<String, std::fmt::Error> {
    let typeattrs = ["Control", "Isochronous", "Bulk", "Interrupt"];
    let syncattrs = ["None", "Asynchronous", "Adaptive", "Synchronous"];
    let usage = ["Data", "Feedback", "Implicit feedback Data", "(reserved)"];
    let hb = ["1x", "2x", "3x", "(??)"];

    let mut output_buf = String::new();

    let wmax: u16 = u16::from_be(endpoint.max_packet_size());

    let inout = if (endpoint.address() & 0x80) != 0 { "IN" } else { "OUT" };

    writeln!(&mut output_buf, "{:indent$}Endpoint Descriptor:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bEndpointAddress    0x{:02x}  EP {} {}", "", endpoint.address(), endpoint.address() & 0x0f, inout)?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}Transfer Type            {:}", "", typeattrs[endpoint.transfer_type() as usize])?;
    writeln!(&mut output_buf, "{:indent$}Synch Type               {:}", "", syncattrs[endpoint.sync_type() as usize])?;
    writeln!(&mut output_buf, "{:indent$}Usage Type               {:}", "", usage[endpoint.usage_type() as usize])?;
    indent -= 2;
    writeln!(&mut output_buf, "{:indent$}wMaxPacketSize      0x{:04x}  {} {} bytes", "", wmax, hb[(wmax as usize >> 11) & 3], wmax & 0x7ff)?;
    writeln!(&mut output_buf, "{:indent$}bInterval            {:5}", "", endpoint.interval())?;

    if let Some(buf) = endpoint.extra() {
        let mut size = buf.len();
        let mut idx: usize = 0;

        while size >= 2 {
            let current = &buf[idx..];

            if current[0] < 2 {
                break;
            }

            match buf[1] {
                USB_DT_CS_ENDPOINT => {
                    if interface.class_code() == 1 && interface.sub_class_code() == 2 {
                        write!(&mut output_buf, "{}", audiostreaming_endpoint::dump(current, interface.protocol_code(), indent)?)?;
                    } else if interface.class_code() == 1 && interface.sub_class_code() == 3 {
                        write!(&mut output_buf, "{}", midistreaming_endpoint::dump(current, indent)?)?;
                    }
                }
                USB_DT_CS_INTERFACE => {
                    match interface.class_code() {
                        LIBUSB_CLASS_COMM | LIBUSB_CLASS_DATA => {}
                        LIBUSB_CLASS_MASS_STORAGE => write!(output_buf, "{}", pipe_desc::dump(current, indent)?)?,
                        _ => {
                            write!(&mut output_buf, "{:indent$}INTERFACE CLASS: ", "")?;
                            for i in 0..current[0] {
                                write!(&mut output_buf, " {:02x}", current[i as usize])?;
                            }
                            writeln!(&mut output_buf)?;
                        }
                    }
                }
                USB_DT_CS_DEVICE => {
                    match interface.class_code() {
                        USB_CLASS_CCID => {
                            write!(&mut output_buf, "{}", ccid_device::dump(current, indent)?)?;
                        }
                        _ => {
                            write!(&mut output_buf, "{:indent$}DEVICE CLASS: ", "")?;
                            for i in 0..current[0] {
                                write!(&mut output_buf, " {:02x}", current[i as usize])?;
                            }
                            writeln!(&mut output_buf)?;
                        }
                    }
                }
                USB_DT_OTG => (),
                USB_DT_INTERFACE_ASSOCIATION => {
                    write!(&mut output_buf, "{}", association::dump(device, current, indent)?)?;
                }
                USB_DT_SS_ENDPOINT_COMP => {
                    /* */
                }
                _ => {
                    write!(&mut output_buf, "{:indent$}** UNRECOGNIZED: ", "")?;
                    for i in 0..current[0] {
                        write!(&mut output_buf, " {:02x}", current[i as usize])?;
                    }
                    writeln!(&mut output_buf)?;
                }
            };

            size -= buf[0] as usize;
            idx += buf[0] as usize;
        }
    }

    Ok(output_buf)
}
