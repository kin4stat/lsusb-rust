use crate::utils::read_be_u32;
use std::fmt::Write;

pub fn dump(buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();
    if buf[0] < 12 {
        writeln!(&mut output_buf, "{:indent$}Bad SuperSpeedPlus USB Device Capability descriptor.", "")?;
        return Ok(output_buf);
    }

    let bm_attr = read_be_u32(&buf[4..]);

    writeln!(&mut output_buf, "{:indent$}SuperSpeedPlus USB Device Capability:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength:             {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType:     {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bDevCapabilityType:  {:5}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}bmAttributes: 0x{:08x}", "", bm_attr)?;

    indent += 2;

    writeln!(&mut output_buf, "{:indent$}Sublink Speed Attribute count: {}", "", (buf[4] & 0x1f) + 1)?;
    writeln!(&mut output_buf, "{:indent$}Sublink Speed ID count: {}", "", ((bm_attr >> 5) & 0xf) + 1)?;
    indent -= 2;
    writeln!(&mut output_buf, "{:indent$}wFunctionalitySupport   0x{:02x}{:02x}", "", buf[9], buf[8])?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}Min functional Speed Attribute ID:  {}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}Min functional RX lanes: {}", "", buf[9] & 0x0f)?;
    writeln!(&mut output_buf, "{:indent$}Min functional TX lanes: {}", "", (buf[9] >> 4) & 0x0f)?;

    for i in 0..(buf[4] & 0x1f) + 1 {
        let base_idx = 12 + i * 4;
        let ss_attr: u32 = read_be_u32(&buf[base_idx as usize..]);

        let bitrate_prefix = match (ss_attr >> 4) & 0x3 {
            1 => "K",
            2 => "M",
            3 => "G",
            _ => " "
        };

        indent -= 2;
        writeln!(&mut output_buf, "{:indent$}bmSublinkSpeedAttr[{}]   0x{:08x}", "", i, ss_attr)?;
        indent += 2;
        writeln!(&mut output_buf, "{:indent$}Speed Attribute ID:{} {}{}b/s {} {} SuperSpeed{}", "",
                 ss_attr & 0x0f,
                 ss_attr >> 16,
                 bitrate_prefix,
                 if (ss_attr & 0x40) != 0 { "Asymmetric" } else { "Symmetric" },
                 if (ss_attr & 0x80) != 0 { "TX" } else { "RC" },
                 if (ss_attr & 0x4000) != 0 { "Plus" } else { "" })?;
    }

    Ok(output_buf)
}