use std::fmt::Write;

pub fn dump(buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    if buf[0] < 10 {
        writeln!(&mut output_buf, "{:indent$}Bad SuperSpeed USB Device Capability descriptor.", "")?;
    }

    writeln!(&mut output_buf, "{:indent$}SuperSpeed USB Device Capability:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength:             {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType:     {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bDevCapabilityType:  {}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}bmAttributes:         0x{:02x}, ", "", buf[3])?;

    if (buf[3] & 0x02u8) != 0 {
        indent += 2;
        writeln!(&mut output_buf, "{:indent$}Latency Tolerance Messages (LTM) Supported", "")?;
        indent -= 2;
    }

    writeln!(&mut output_buf, "{:indent$}wSpeedsSupported: 0x{:02x}{:02x}", "", buf[5], buf[6])?;

    if (buf[4] & (1 << 0)) != 0 {
        writeln!(&mut output_buf, "{:indent$}Device can operate at Low Speed (1Mbps)", "")?;
    }
    if (buf[4] & (1 << 1)) != 0 {
        writeln!(&mut output_buf, "{:indent$}Device can operate at Full Speed (12Mbps)", "")?;
    }
    if (buf[4] & (1 << 2)) != 0 {
        writeln!(&mut output_buf, "{:indent$}Device can operate at High Speed (480Mbps)", "")?;
    }
    if (buf[4] & (1 << 3)) != 0 {
        writeln!(&mut output_buf, "{:indent$}Device can operate at SuperSpeed (5Gbps)", "")?;
    }

    writeln!(&mut output_buf, "{:indent$}bFunctionalitySupport: {:3}", "", buf[6])?;
    indent += 2;
    match buf[6] {
        0 => writeln!(&mut output_buf, "{:indent$}Lowest fully-functional device speed is Low Speed (1Mbps)", "")?,
        1 => writeln!(&mut output_buf, "{:indent$}Lowest fully-functional device speed is Full Speed (12Mbps)", "")?,
        2 => writeln!(&mut output_buf, "{:indent$}Lowest fully-functional device speed is High Speed (480Mbps)", "")?,
        3 => writeln!(&mut output_buf, "{:indent$}Lowest fully-functional device speed is SuperSpeed (5Gbps)", "")?,
        _ => writeln!(&mut output_buf, "{:indent$}Lowest fully-functional device speed is at an unknown speed!", "")?,
    }

    indent -= 2;

    writeln!(&mut output_buf, "{:indent$}bU1DevExitLat:        {:4} microseconds", "", buf[7])?;
    writeln!(&mut output_buf, "{:indent$}bU1DevExitLat:    {:8} microseconds", "", buf[8] as u32 + ((buf[9] as u32) << 8))?;

    Ok(output_buf)
}