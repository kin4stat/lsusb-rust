use crate::utils;
use std::fmt::Write;

pub fn dump(buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    if buf[0] < 54 {
        writeln!(&mut output_buf, "{:indent$}Warning: Descriptor too short", "")?;
        return Ok(output_buf);
    }
    writeln!(&mut output_buf, "{:indent$}ChipCard Interface Descriptor:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength:            {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType:    {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bcdCCID:            {:2}.{:02x}", "", buf[3], buf[2])?;


    writeln!(&mut output_buf, "{:indent$}nMaxSlotIndex:       {:5}", "", buf[4])?;
    println!("{:indent$}bVoltageSupport:     {:5}  {}{}{}", "", buf[4],
             if (buf[5] & 1) != 0 { "5.0V " } else { "" },
             if (buf[5] & 2) != 0 { "3.0V " } else { "" },
             if (buf[5] & 4) != 0 { "1.8V " } else { "" });

    let us = utils::read_le_u32(&buf[6..]);
    let arr = [String::from(" T=0"), String::from(" T=1"), String::from(" (Invalid values detected)")];
    writeln!(&mut output_buf, "{:indent$}dwProtocols         {:5} ", "", us)?;
    utils::print_with_bitflags(us, &arr, &mut output_buf)?;

    writeln!(&mut output_buf, "{:indent$}dwDefaultClock:      {:5}", "", utils::read_le_u32(&buf[10..]))?;
    writeln!(&mut output_buf, "{:indent$}dwMaxiumumClock:     {:5}", "", utils::read_le_u32(&buf[14..]))?;
    writeln!(&mut output_buf, "{:indent$}bNumClockSupported:  {:5}", "", buf[18])?;
    writeln!(&mut output_buf, "{:indent$}dwDataRate:        {:7} bps", "", utils::read_le_u32(&buf[19..]))?;
    writeln!(&mut output_buf, "{:indent$}dwMaxDataRate:     {:7} bps", "", utils::read_le_u32(&buf[23..]))?;
    writeln!(&mut output_buf, "{:indent$}bNumDataRatesSupp:   {:5}", "", buf[27])?;

    writeln!(&mut output_buf, "{:indent$}dwMaxIFSD:           {:5}", "", utils::read_le_u32(&buf[28..]))?;


    let us = utils::read_le_u32(&buf[32..]);
    let arr = [String::from(" 2-wire"), String::from(" 3-wire"), String::from(" I2C")];
    writeln!(&mut output_buf, "{:indent$}dwSyncProtocols:  {:08X}", "", us)?;
    utils::print_with_bitflags(us, &arr, &mut output_buf)?;

    let us = utils::read_le_u32(&buf[36..]);
    let arr = [String::from(" accept"), String::from(" eject"), String::from(" capture"), String::from("lock")];
    writeln!(&mut output_buf, "{:indent$}dwMechanical:     {:08X}", "", us)?;
    utils::print_with_bitflags(us, &arr, &mut output_buf)?;

    let us = utils::read_le_u32(&buf[40..]);

    indent += 2;
    let arr = [String::new(),
        format!("{:indent$}Auto configuration based on ATR\n", ""),
        format!("{:indent$}Auto activation on insert\n", ""),
        format!("{:indent$}Auto clock change\n", ""),
        format!("{:indent$}Auto baud rate change\n", ""),
        format!("{:indent$}Auto parameter negotiation made by CCID\n", ""),
        format!("{:indent$}Auto PPS made by CCID\n", ""),
        format!("{:indent$}CCID can set ICC in clock stop mode\n", ""),
        format!("{:indent$}NAD value other than 0x00 accepted\n", ""),
        format!("{:indent$}Auto IFSD exchange\n", ""), ];
    let arr2 = [format!("{:indent$}TPDU level exchange\n", ""),
        format!("{:indent$}Short APDU level exchange\n", ""),
        format!("{:indent$}Short and extended APDU level exchange\n", ""), ];

    indent -= 2;
    writeln!(&mut output_buf, "{:indent$}dwMechanical:     {:08X}", "", us)?;
    utils::print_with_bitflags(us, &arr, &mut output_buf)?;
    utils::print_with_bitflags_with_off(us, &arr2, 16, &mut output_buf)?;

    let us = utils::read_le_u32(&buf[44..]);
    write!(&mut output_buf, "{:indent$}dwMaxCCIDMsgLen:     {:5}", "", us)?;
    writeln!(&mut output_buf, "{:indent$}bClassGetResponse:    ", "")?;
    if buf[48] == u8::MAX {
        write!(&mut output_buf, "echo")?;
    } else {
        write!(&mut output_buf, "  {:02X}", buf[48])?;
    }
    writeln!(&mut output_buf, "{:indent$}bClassEnvelope:       ", "")?;
    if buf[49] == u8::MAX {
        write!(&mut output_buf, "echo")?;
    } else {
        write!(&mut output_buf, "  {:02X}", buf[49])?;
    }

    writeln!(&mut output_buf, "        wlcdLayout           ")?;
    if buf[50] == 0 && !buf[51] == 0 {
        write!(&mut output_buf, "none")?;
    } else {
        write!(&mut output_buf, "{} cols {} lines", buf[50], buf[51])?;
    }

    let arr = [String::from(" verification"), String::from(" modification")];
    writeln!(&mut output_buf, "{:indent$}bPINSupport:         {:5}", "", buf[52])?;
    utils::print_with_bitflags(buf[52] as u32, &arr, &mut output_buf)?;
    writeln!(&mut output_buf, "{:indent$}bMaxCCIDBusySlots:   {:5}", "", buf[53])?;

    if buf[0] > 54 {
        writeln!(&mut output_buf, "        junk             ")?;
        for i in 0..buf[0] {
            write!(&mut output_buf, " {:02x}", buf[i as usize])?;
        }
    }

    Ok(output_buf)
}
