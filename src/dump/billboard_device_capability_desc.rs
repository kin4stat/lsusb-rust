use crate::consts::{BILLBOARD_MAX_NUM_ALT_MODE};
use crate::utils;
use std::fmt::Write;

pub fn dump(device: &rusb::DeviceHandle<rusb::Context>, buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    if buf[0] < 48 {
        writeln!(&mut output_buf, "{:indent$}Bad Billboard Capability descriptor.", "")?;
        return Ok(output_buf);
    }
    if buf[4] > BILLBOARD_MAX_NUM_ALT_MODE {
        writeln!(&mut output_buf, "{:indent$}Invalid value for bNumberOfAlternateModes.", "")?;
        return Ok(output_buf);
    }

    if buf[0] < (44 + buf[4] * 4) {
        writeln!(&mut output_buf, "{:indent$}Invalid value for bNumberOfAlternateModes.", "")?;
        return Ok(output_buf);
    }

    let vconn_power = utils::read_be_u16(&buf[6..]);
    let vconn = if vconn_power & (1 << 15) != 0 {
        "VCONN power not required"
    } else if vconn_power < 7 {
        ["1W", "1.5W", "2W", "3W", "4W", "5W", "6W", "reserved"][vconn_power as usize & 7]
    } else {
        "reserved"
    };

    let url = utils::read_device_string(Some(device), || buf[3]);

    writeln!(&mut output_buf, "{:indent$}Billboard Capability:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength                 {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType         {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bDevCapabilityType      {:5}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}iAdditionalInfoURL      {:5} {}", "", buf[3], url)?;
    writeln!(&mut output_buf, "{:indent$}bNumberOfAlternateModes {:5}", "", buf[4])?;
    writeln!(&mut output_buf, "{:indent$}bPreferredAlternateMode {:5}", "", buf[5])?;
    writeln!(&mut output_buf, "{:indent$}VCONN Power             {:5} {}", "", vconn_power, vconn)?;


    let bm_configured = &buf[8..32];
    writeln!(&mut output_buf, "{:indent$}bmConfigured               ", "")?;
    for i in 8..8 + 32 {
        print!(" {:02x}", buf[i as usize]);
    }

    writeln!(&mut output_buf, "{:indent$}bcdVersion              {:2x}.{:02x}", "", if buf[41] == 0 { 1 } else { buf[41] }, buf[40])?;
    writeln!(&mut output_buf, "{:indent$}bAdditionalFailureInfo  {:5}", "", buf[42])?;
    writeln!(&mut output_buf, "{:indent$}bReserved               {:5}", "", buf[43])?;

    writeln!(&mut output_buf, "{:indent$}Alternate Modes supported by Device Container:", "")?;
    let mut i = 44;
    for alt_mode in 0..buf[4] {
        let svid = utils::read_be_u16(&buf[i..]);
        let state = ((bm_configured[(alt_mode as usize) >> 2]) >> ((alt_mode & 0x3) << 1)) & 0x3;
        let alt_mode_state = ["Unspecified Error",
            "Alternate Mode configuration not attempted",
            "Alternate Mode configuration attempted but unsuccessful",
            "Alternate Mode configuration successful"][state as usize];
        let alt_mode_str = utils::read_device_string(Some(device), || buf[i + 3]);

        writeln!(&mut output_buf, "{:indent$}Alternate Mode {} : {}", "", alt_mode, alt_mode_state)?;
        indent += 2;
        writeln!(&mut output_buf, "{:indent$}wSVID[{}]                    0x{:04X}", "", alt_mode, svid)?;
        writeln!(&mut output_buf, "{:indent$}bAlternateMode[{}]       {:5}", "", alt_mode, buf[i + 2])?;
        writeln!(&mut output_buf, "{:indent$}iAlternateModeString[{}] {:5} {}", "", alt_mode, buf[i + 3], alt_mode_str)?;
        indent -= 2;
        i += 4;
    }

    Ok(output_buf)
}
