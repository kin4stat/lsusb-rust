use crate::utils;
use crate::consts::{CTRL_TIMEOUT};
use rusb::constants::{LIBUSB_REQUEST_GET_DESCRIPTOR,
                      LIBUSB_RECIPIENT_DEVICE,
                      LIBUSB_ENDPOINT_IN,
                      LIBUSB_REQUEST_TYPE_CLASS,
                      LIBUSB_RECIPIENT_OTHER,
                      LIBUSB_REQUEST_GET_STATUS};
use std::fmt::Write;

fn dump_hub_impl(prefix: &str, buf: &[u8], tt_type: u8, mut indent: usize) -> Result<String, std::fmt::Error> {
    let whub_char = utils::read_be_u16(&buf[3..]);

    let mut output_buf = String::new();

    writeln!(&mut output_buf, "{}{:indent$}OTG Descriptor:", "", prefix)?;
    indent += 2;

    writeln!(&mut output_buf, "{}{:indent$}bLength:             {:3}", "", prefix, buf[0])?;
    writeln!(&mut output_buf, "{}{:indent$}bDescriptorType:     {:3}", "", prefix, buf[1])?;
    writeln!(&mut output_buf, "{}{:indent$}nNbrPorts:           {:3}", "", prefix, buf[2])?;
    writeln!(&mut output_buf, "{}{:indent$}wHubCharacteristic: 0x{:04x}", "", prefix, whub_char)?;

    indent += 2;
    writeln!(&mut output_buf, "{}{:indent$}{}", "", prefix, match whub_char & 0x03 {
        0 => "{:indent$}Ganged power switching",
        1 => "{:indent$}Per-port power switching",
        _ => "{:indent$}No power switching (usb 1.0)",
    })?;

    writeln!(&mut output_buf, "{}{:indent$}{}", "", prefix, match (whub_char >> 3) & 0x03 {
        0 => "Ganged overcurrent protection",
        1 => "Per-port overcurrent protection",
        _ => "No overcurrent protection",
    })?;

    if tt_type >= 1 && tt_type < 3 {
        let t = (whub_char >> 5) & 0x03;
        writeln!(&mut output_buf, "{}{:indent$}TT think time {} FS bits", "", prefix, t)?;
    }

    if tt_type != 3 && (whub_char & (1 << 7)) != 0 {
        writeln!(&mut output_buf, "{}{:indent$}Port indicators", "", prefix)?;
    }
    writeln!(&mut output_buf, "{}{:indent$}bPwrOn2PwrGood      {:3} * 2 milli seconds", "", prefix, buf[5])?;

    let mult = if tt_type == 3 { 4 } else { 1 };
    indent -= 2;

    writeln!(&mut output_buf, "{}{:indent$}bHubContrCurrent   {:4} milli Ampere", "", prefix, buf[6] * mult)?;

    let offset = if tt_type == 3 {
        writeln!(&mut output_buf, "{}{:indent$}bHubDecLat          0.{:1} micro seconds", "", prefix, buf[7])?;
        writeln!(&mut output_buf, "{}{:indent$}wHubDelay          {:4} nano seconds", "", prefix, (buf[8] << 4) + buf[7])?;

        10
    } else {
        7
    };

    let mut end = (buf[2] >> 3) + 1;
    if end > 3 { end = 3 }

    writeln!(&mut output_buf, "{}{:indent$}DeviceRemovable   ", "", prefix)?;
    for i in 0..end as usize {
        print!(" 0x{:02x}", buf[offset + i]);
    }

    if tt_type != 3 {
        writeln!(&mut output_buf, "{}{:indent$}PortPwrCtrlMask   ", "", prefix)?;
        for j in 0..end as usize {
            print!(" 0x{:02x}", buf[offset + end as usize + j]);
        }
    }
    writeln!(&mut output_buf)?;

    Ok(output_buf)
}

pub fn dump(device: &rusb::DeviceHandle<rusb::Context>, tt_type: u8, speed: rusb::Version, has_ssp: bool, mut indent: usize) -> Result<String, std::fmt::Error> {
    let is_ext_status = tt_type == 3 && speed >= rusb::Version::from_bcd(0x0310) && has_ssp;
    let mut buf = [0; 7 + 2 * 3];

    let mut output_buf = String::new();

    let value;
    if speed >= rusb::Version::from_bcd(0x0300) {
        value = 0x2A;
    } else {
        value = 0x29;
    }

    let ret = device.read_control(LIBUSB_RECIPIENT_DEVICE | LIBUSB_ENDPOINT_IN,
                                  LIBUSB_REQUEST_GET_DESCRIPTOR,
                                  value << 8,
                                  0,
                                  &mut buf,
                                  CTRL_TIMEOUT);

    match ret {
        Ok(size) => if size < 9 {
            writeln!(&mut output_buf, "incomplete hub descriptor, {size} bytes")?;
            return Ok(output_buf);
        },
        Err(e) => {
            writeln!(&mut output_buf, "can't get hub descriptor (Err: {e})")?;
            return Ok(output_buf);
        }
    }

    write!(output_buf, "{}", dump_hub_impl("", &buf, tt_type, 0)?)?;

    writeln!(&mut output_buf, "{:indent$}Hub Port Status:", "")?;
    for i in 0..buf[2] {
        let mut status = [0u8; 8];

        let ret = device.read_control(LIBUSB_ENDPOINT_IN |
                                          LIBUSB_REQUEST_TYPE_CLASS |
                                          LIBUSB_RECIPIENT_OTHER,
                                      LIBUSB_REQUEST_GET_STATUS,
                                      if is_ext_status { 2 } else { 0 },
                                      i as u16 + 1,
                                      &mut status,
                                      CTRL_TIMEOUT);

        match ret {
            Err(e) => {
                writeln!(&mut output_buf, "cannot read port {i} (Err: {e})")?;
                break;
            }
            _ => ()
        };

        indent += 2;
        writeln!(&mut output_buf, "{:indent$}Port {}: {:02x}{:02x}.{:02x}{:02x}", "", i + 1, status[3], status[2], status[1], status[0])?;
        if speed < rusb::Version::from_bcd(0x0300) {
            let arr = [String::from(" C_CONNECT"),
                String::from(" C_ENABLE"),
                String::from(" C_SUSPEND"),
                String::from(" C_OC"),
                String::from(" C_RESET")];
            utils::print_with_bitflags(status[2] as u32, &arr, &mut output_buf)?;
            let arr = [String::from(" power"),
                String::from(" lowspeed"),
                String::from(" highspeed"),
                String::from(" test"),
                String::from(" indicator")];
            utils::print_with_bitflags(status[1] as u32, &arr, &mut output_buf)?;
            let arr = [String::from(" connect"),
                String::from(" enable"),
                String::from(" suspend"),
                String::from(" oc"),
                String::from(" RESET"),
                String::from(" L1")];
            utils::print_with_bitflags(status[0] as u32, &arr, &mut output_buf)?;
            writeln!(&mut output_buf)?;
        } else {
            let link_state = ((status[0] & 0xe0) >> 5) + ((status[1] & 0x01) << 3);

            let arr = [
                String::from(" C_CONNECT"),
                String::new(),
                String::new(),
                String::from(" C_OC"),
                String::from(" C_RESET"),
                String::from(" C_BH_RESET"),
                String::from(" C_LINK_STATE"),
                String::from("C_CONFIG_ERROR")];
            utils::print_with_bitflags(status[2] as u32, &arr, &mut output_buf)?;

            write!(&mut output_buf, "{}", if (status[1] & 0x1C) == 0 { " 5Gbps" } else { " Unknown Speed" })?;
            if (status[1] & 0x02) != 0 {
                write!(&mut output_buf, " power")?;
            }

            let link_state_descriptions = [
                String::from("U0"),
                String::from("U1"),
                String::from("U2"),
                String::from("suspend"),
                String::from("SS.disabled"),
                String::from("Rx.Detect"),
                String::from("SS.Inactive"),
                String::from("Polling"),
                String::from("Recovery"),
                String::from("Hot Reset"),
                String::from("Compliance"),
                String::from("Loopback")];
            if (link_state as usize) < link_state_descriptions.len() {
                write!(&mut output_buf, " {}", link_state_descriptions[link_state as usize])?;
            }
            let arr = [
                String::from(" connect"),
                String::from("enable"),
                String::new(),
                String::from(" oc"),
                String::from(" RESET")];
            utils::print_with_bitflags(status[0] as u32, &arr, &mut output_buf)?;
        }

        indent += 2;
        if is_ext_status && (status[0] & 0x01) != 0 {
            writeln!(&mut output_buf, "{:indent$}Ext Status: {:02x}{:02x}.{:02x}{:02x}", "", status[7], status[6], status[5], status[4])?;
            indent += 2;
            writeln!(&mut output_buf, "{:indent$}RX Speed Attribute ID: {} Lanes: {}", "", status[4] & 0x0f, (status[5] & 0x0f) + 1)?;
            writeln!(&mut output_buf, "{:indent$}TX Speed Attribute ID: {} Lanes: {}", "", (status[4] >> 4) & 0x0f, ((status[5] >> 4) & 0x0f) + 1)?;
            indent -= 2;
        }

        indent -= 4;
    }

    Ok(output_buf)
}