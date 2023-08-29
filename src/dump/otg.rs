use crate::consts::{ USB_DT_OTG };
use std::fmt::Write;
fn dump_otg_impl(buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    writeln!(&mut output_buf, "{:indent$}OTG Descriptor:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength               {:3}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType       {:3}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bmAttributes         0x{:02x}", "", buf[2])?;

    indent += 2;
    let srp_str = if (buf[2] & 0x01) != 0 {
        format!("{:indent$}SRP (Session Request Protocol)", "")
    } else {
        String::new()
    };

    let hnp_str = if (buf[2] & 0x01) != 0 {
        format!("{:indent$}HNP (Host Negotiation Protocol)", "")
    } else {
        String::new()
    };
    writeln!(&mut output_buf, "{}", srp_str)?;
    writeln!(&mut output_buf, "{}", hnp_str)?;

    Ok(output_buf)
}

fn find_otg(buf: &[u8]) -> Option<(usize, &[u8])> {
    let mut idx: usize = 0;

    let buf: &[u8] = buf;
    while buf.len() - idx >= 3 {
        if buf[idx] == 3 && (buf[idx + 1]) == USB_DT_OTG {
            return Some((idx, buf));
        }
        if (buf[0] as usize) > (buf.len() - idx) {
            return None;
        }
        idx += buf[0] as usize;
    }
    return None;
}

pub fn dump(config: &rusb::ConfigDescriptor, indent: usize) -> (bool, Result<String, std::fmt::Error>) {
    let dumper = |result: (usize, &[u8])| dump_otg_impl(&result.1[result.0..], indent);
    if let Some(result) = find_otg(config.extra()) {
        return (true, dumper(result));
    }

    for i in config.interfaces() {
        for j in i.descriptors() {
            if let Some(result) = find_otg(j.extra()) {
                return (true, dumper(result));
            }

            for k in j.endpoint_descriptors() {
                if let Some(extra) = k.extra() {
                    if let Some(result) = find_otg(extra) {
                        return (true, dumper(result));
                    }
                }
            }
        }
    }

    return (false, Ok(String::new()));
}