use crate::utils;
use std::fmt::Write;

pub fn dump(buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    if buf[0] < 20 {
        writeln!(&mut output_buf, "{:indent$}Bad Container ID Device Capability descriptor.", "")?;
        return Ok(output_buf);
    }
    writeln!(&mut output_buf, "{:indent$}SuperSpeedPlus USB Device Capability:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength:             {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType:     {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bDevCapabilityType:  {:5}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}bReserved:           {:5}", "", buf[3])?;

    writeln!(&mut output_buf, "{:indent$}ContainerID:             {}", "", utils::get_guid(&buf[4..]))?;

    Ok(output_buf)
}