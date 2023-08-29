use crate::consts::{ USB_DT_CS_ENDPOINT };
use super::junk;
use std::fmt::Write;

pub fn dump(buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    if buf[1] != USB_DT_CS_ENDPOINT {
        writeln!(&mut output_buf, "{:indent$}Warning: Invalid descriptor", "")?;
    }

    let subtype_str = if buf[2] == 2 { "GENERAL" } else { "Invalid" };
    writeln!(&mut output_buf, "{:indent$}MIDIStreaming Endpoint Descriptor:", "")?;
    indent += 2;

    writeln!(&mut output_buf, "{:indent$}bLength:                 {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType:         {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorSubtype:      {:5} ({})", "", buf[2], subtype_str)?;
    writeln!(&mut output_buf, "{:indent$}NumEmbMIDIJack:         {:5}", "", buf[3], )?;

    if buf[0] < 5 {
        indent -= 2;
        writeln!(&mut output_buf, "{:indent$}Descriptor too short", "")?;
    } else {
        for j in 0..buf[3] {
            writeln!(&mut output_buf, "{:indent$}baAssocJackID({:2})   {:5}", "", j, buf[4 + j as usize])?;
        }

        writeln!(&mut output_buf, "{}", junk::dump(&buf[..(4 + buf[3] as usize)], indent)?)?;
    }
    
    Ok(output_buf)
}