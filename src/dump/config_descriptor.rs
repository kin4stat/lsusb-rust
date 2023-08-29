use crate::utils;
use std::fmt::Write;

pub fn dump(device: Option<&rusb::DeviceHandle<rusb::Context>>,
            config: &rusb::ConfigDescriptor,
            speed: rusb::Version,
            mut indent: usize) -> Result<String, std::fmt::Error> {
    let cfg = utils::read_configuration_description_string(device, config);

    let mut output_buf = String::new();

    writeln!(&mut output_buf, "{:indent$}Configuration Descriptor:", "")?;

    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bNumInterfaces:      {:5}", "", config.num_interfaces())?;
    writeln!(&mut output_buf, "{:indent$}bConfigurationValue: {:5}", "", config.number())?;
    writeln!(&mut output_buf, "{:indent$}iConfiguration:      {:5} {}", "", config.description_string_index().map_or(0, |v| v), cfg)?;

    indent += 2;
    if config.self_powered() {
        writeln!(&mut output_buf, "{:indent$}Self Powered", "")?;
    } else {
        writeln!(&mut output_buf, "{:indent$}(Bus Powered)", "")?;
    }

    if config.remote_wakeup() {
        writeln!(&mut output_buf, "{:indent$}Remote Wakeup", "")?;
    }

    indent -= 2;
    let power_multiplier = if speed >= rusb::Version::from_bcd(0x0300) { 8 } else { 2 };
    writeln!(&mut output_buf, "{:indent$}MaxPower            {:5}mA", "", config.max_power() * power_multiplier)?;

    Ok(output_buf)
}