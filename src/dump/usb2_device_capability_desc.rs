use crate::utils::read_be_u32;
use std::fmt::Write;

pub fn dump(buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();
    let wide: u32 = read_be_u32(&buf[3..]);

    writeln!(&mut output_buf, "{:indent$}USB 2.0 Extension Device Capability:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength:             {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType:     {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bDevCapabilityType:  {:5}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}bmAttributes:         0x{:08x}", "", wide)?;

    indent += 2;
    if (wide & 0x02) == 0 {
        writeln!(&mut output_buf, "{:indent$}(Missing must-be-set LPM bit!)", "")?;
    } else if (wide & 0x04) == 0 {
        writeln!(&mut output_buf, "{:indent$}HIRD Link Power Management (LPM) Supported", "")?;
    } else {
        writeln!(&mut output_buf, "{:indent$}BESL Link Power Management (LPM)  Supported", "")?;
        indent -= 2;
        if (wide & 0x08) != 0 {
            writeln!(&mut output_buf, "{:indent$}BESL value:    {:5} us", "", wide & 0xf00)?;
        }
        if (wide & 0x10) != 0 {
            writeln!(&mut output_buf, "{:indent$}Deep BESL value:    {:5} us", "", wide & 0xf000)?;
        }
    }

    Ok(output_buf)
}