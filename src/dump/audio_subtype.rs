//use super::desc;
use crate::consts::{USB_AUDIO_CLASS_2, USB_AUDIO_CLASS_3};
use std::fmt::Write;

pub fn dump(_buf: &[u8], subtype: &str, protocol: u8, _indent: usize) -> Result<String, std::fmt::Error> {
    let _ = match protocol {
        USB_AUDIO_CLASS_2 => 1,
        USB_AUDIO_CLASS_3 => 2,
        _ => 0
    };

    let mut output_buf = String::new();

    write!(output_buf, " ({})", subtype)?;

    //desc::dump(buf, Some(buf[0] as usize - 3), indent);

    Ok(output_buf)
}