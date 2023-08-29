use crate::utils;
use crate::consts::{ WEBUSB_GUID };
use std::fmt::Write;

pub fn dump(buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let desc_len = buf[0];
    let cap_data_len = desc_len - 20;

    let mut output_buf = String::new();

    if desc_len < 20 {
        writeln!(&mut output_buf, "{:indent$}Bad Platform Device Capability descriptor.", "")?;
        return Ok(output_buf);
    }

    writeln!(&mut output_buf, "{:indent$}Platform Device Capability:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength:             {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType:     {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bDevCapabilityType:  {:5}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}bReserved:           {:5}", "", buf[3])?;

    let guid = utils::get_guid(&buf[4..]);

    if guid == WEBUSB_GUID && desc_len == 24 {
        indent += 2;
        writeln!(&mut output_buf, "{:indent$}WebUSB:", "")?;
        indent += 2;
        writeln!(&mut output_buf, "{:indent$}bcdVersion   {:2x}.{:02x}", "", buf[21], buf[22])?;
        writeln!(&mut output_buf, "{:indent$}bVendorCode  {:5}", "", buf[22])?;
        writeln!(&mut output_buf, "{:indent$}iLandingPage {:5}", "", buf[23])?;

        indent -= 4;
    }

    writeln!(&mut output_buf, "{:indent$}PlatformCapabilityUUID: {}", "", guid)?;

    for i in 0..cap_data_len {
        writeln!(&mut output_buf, "{:indent$}CapabilityData[{}]    0x{:02x}", "", i, buf[20 + i as usize])?;
    }

    Ok(output_buf)
}