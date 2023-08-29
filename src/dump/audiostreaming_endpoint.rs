use super::audio_subtype;
use crate::consts::{ USB_DT_CS_ENDPOINT };
use std::fmt::Write;

pub fn dump(buf: &[u8], protocol: u8, mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();
    if buf[1] != USB_DT_CS_ENDPOINT {
        writeln!(&mut output_buf, "{:indent$}Warning: Invalid descriptor", "")?;
    }

    writeln!(&mut output_buf, "{:indent$}AudioStreaming Endpoint Descriptor:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength:                 {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType:         {:5}", "",  buf[1])?;
    write!(&mut output_buf,   "{:indent$}bDescriptorSubtype:      {:5}", "", buf[2])?;
    let subtype = if buf[2] == 1 { "EP_GENERAL" } else { "invalid" };

    writeln!(&mut output_buf, "{}", audio_subtype::dump(buf, subtype, protocol, 5)?)?;

    Ok(output_buf)
}