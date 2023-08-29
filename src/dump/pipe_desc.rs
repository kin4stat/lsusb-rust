use std::fmt::Write;

pub fn dump(buf: &[u8], indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    if buf[0] == 4 && buf[1] == 0x24 {
        let pipe_name = match buf[2] {
            0 => "Reserved",
            1 => "Command pipe",
            2 => "Status pipe",
            3 => "Data-in pipe",
            4 => "Data-out pipe",
            5..=0xDF => "Reserved",
            0xE0..=0xEF => "Vendor Specific",
            0xF0..=0xFF => "Reserved",
        };

        writeln!(&mut output_buf, "{:indent$}{} (0x{:02x})", "", pipe_name, buf[2])?;
    } else {
        write!(&mut output_buf, "{:indent$}INTERFACE CLASS: ", "")?;
        for i in 0..buf[0] {
            write!(&mut output_buf, " {:02x}", buf[i as usize])?;
        }
        writeln!(&mut output_buf)?;
    }

    Ok(output_buf)
}