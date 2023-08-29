use rusb;
use rusb::constants::{ LIBUSB_ENDPOINT_IN, LIBUSB_REQUEST_TYPE_CLASS, LIBUSB_RECIPIENT_DEVICE, LIBUSB_REQUEST_GET_DESCRIPTOR };
use crate::consts::{ CTRL_TIMEOUT, USB_DT_DEBUG };
use std::fmt::Write;

pub fn dump(device: &rusb::DeviceHandle<rusb::Context>, mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut buf = [0u8; 4];
    let ret = device.read_control(LIBUSB_ENDPOINT_IN |
                                      LIBUSB_REQUEST_TYPE_CLASS |
                                      LIBUSB_RECIPIENT_DEVICE,
                                  LIBUSB_REQUEST_GET_DESCRIPTOR,
                                  (USB_DT_DEBUG as u16) << 8,
                                  0,
                                  &mut buf,
                                  CTRL_TIMEOUT);

    let mut output_buf = String::new();

    if let Err(_) = ret {
        writeln!(&mut output_buf, "{:indent$}can't get debug descriptor", "")?;
        return Ok(output_buf);
    }

    if ret.unwrap() != buf.len() ||
        buf[0] != (ret.unwrap() as u8) ||
        buf[1] != USB_DT_DEBUG {
        return Ok(output_buf);
    }

    writeln!(&mut output_buf, "{:indent$}Debug descriptor:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength              {:4}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType      {:4}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bDebugInEndpoint     0x{:02x}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}bDebugOutEndpoint    0x{:02x}", "", buf[3])?;

    Ok(output_buf)
}