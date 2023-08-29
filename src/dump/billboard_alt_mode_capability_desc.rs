use std::fmt::Write;

pub fn dump(buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    if buf[0] != 8 {
        writeln!(&mut output_buf, "{:indent$}Bad Billboard Alternate Mode Capability descriptor.", "")?;
        return Ok(output_buf);
    }

    writeln!(&mut output_buf, "{:indent$}Billboard Alternate Mode Capability:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength:                 {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType:         {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bDevCapabilityType:      {:5}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}bIndex:                  {:5}", "", buf[3])?;
    writeln!(&mut output_buf, "{:indent$}dwAlternateModeVdo:         0x{:02x}{:02x}{:02x}{:02x}", "", buf[4], buf[5], buf[6], buf[7])?;

    Ok(output_buf)
}