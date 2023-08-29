use crate::consts::{ USB_DT_DEVICE_QUALIFIER, CTRL_TIMEOUT };
use rusb::constants::{ LIBUSB_RECIPIENT_DEVICE, LIBUSB_ENDPOINT_IN, LIBUSB_REQUEST_GET_DESCRIPTOR, LIBUSB_REQUEST_TYPE_STANDARD };
use std::fmt::Write;

pub fn dump(device: &rusb::DeviceHandle<rusb::Context>, mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut buf: [u8; 10] = [0; 10];

    let mut output_buf = String::new();

    let ret = device.read_control(LIBUSB_RECIPIENT_DEVICE|
                                      LIBUSB_REQUEST_TYPE_STANDARD |
                                      LIBUSB_ENDPOINT_IN,
                                  LIBUSB_REQUEST_GET_DESCRIPTOR,
                                  USB_DT_DEVICE_QUALIFIER << 8,
                                  0,
                                  &mut buf,
                                  CTRL_TIMEOUT);

    let ret = match ret {
        Ok(v) => v,
        Err(_) => return Ok(output_buf)
    };


    if ret != buf.len() || buf[0] as usize != ret || buf[1] as u16 != USB_DT_DEVICE_QUALIFIER {
        return Ok(output_buf);
    }

    writeln!(&mut output_buf, "{:indent$}Device Qualifier (for other device speed):", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength             {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType     {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bcdUSB              {:2x}.{:02x}", "", buf[3], buf[2])?;
    writeln!(&mut output_buf, "{:indent$}bDeviceClass        {:5}", "", buf[4])?;
    writeln!(&mut output_buf, "{:indent$}bDeviceSubClass     {:5}", "", buf[5])?;
    writeln!(&mut output_buf, "{:indent$}bDeviceProtocol     {:5}", "", buf[6])?;
    writeln!(&mut output_buf, "{:indent$}bMaxPacketSize0     {:5}", "", buf[7])?;

    Ok(output_buf)
}
