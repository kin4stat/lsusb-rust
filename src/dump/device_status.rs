use rusb;
use rusb::constants::{ LIBUSB_ENDPOINT_IN, LIBUSB_REQUEST_TYPE_CLASS, LIBUSB_RECIPIENT_DEVICE, LIBUSB_REQUEST_GET_STATUS };
use crate::consts::{ CTRL_TIMEOUT };
use std::fmt::Write;

pub fn dump(device: &rusb::DeviceHandle<rusb::Context>, has_otg: bool, super_speed: bool, mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut status = [0u8; 8];
    let ret = device.read_control(LIBUSB_ENDPOINT_IN |
                                      LIBUSB_REQUEST_TYPE_CLASS |
                                      LIBUSB_RECIPIENT_DEVICE,
                                  LIBUSB_REQUEST_GET_STATUS,
                                  0,
                                  0,
                                  &mut status,
                                  CTRL_TIMEOUT);

    let mut output_buf = String::new();

    if let Err(e) = ret {
        writeln!(&mut output_buf, "{:indent$}Cannot read device status (Err: {e})", "")?;

        return Ok(output_buf);
    }

    writeln!(&mut output_buf, "{:indent$}Device Status:     0x{:02x}{:02x}", "", status[1], status[0])?;

    indent += 2;

    if (status[0] & 0b000001) != 0 {
        writeln!(&mut output_buf, "{:indent$}Self Powered", "")?;
    } else {
        writeln!(&mut output_buf, "{:indent$}(Bus Powered)", "")?;
    }

    if (status[0] & 0b000010) != 0 {
        writeln!(&mut output_buf, "{:indent$}Remote Wakeup Enabled", "")?;
    }
    if super_speed {
        if (status[0] & 0b000100) != 0 {
            writeln!(&mut output_buf, "{:indent$}U1 Enabled", "")?;
        }
        if (status[0] & 0b001000) != 0 {
            writeln!(&mut output_buf, "{:indent$}U2 Enabled", "")?;
        }
        if (status[0] & 0b010000) != 0 {
            writeln!(&mut output_buf, "{:indent$}Latency Tolerance Messaging (LTM) Enabled", "")?;
        }
    }

    if has_otg {
        if (status[0] & 0b001000) != 0 {
            writeln!(&mut output_buf, "{:indent$}HNP Enabled", "")?;
        }
        if (status[0] & 0b010000) != 0 {
            writeln!(&mut output_buf, "{:indent$}HNP Capable", "")?;
        }
        if (status[0] & 0b100000) != 0 {
            writeln!(&mut output_buf, "{:indent$}ALT port is HNP Capable", "")?;
        }
    }

    if (status[0] & 0b1000000) != 0 {
        writeln!(&mut output_buf, "{:indent$}Debug Mode", "")?;
    }

    Ok(output_buf)
}