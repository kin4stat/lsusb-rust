use crate::utils;
use std::fmt::Write;

pub fn dump(device: Option<&rusb::DeviceHandle<rusb::Context>>, buf: &[u8], mut indent: usize) -> Result<String, std::fmt::Error> {
    let istr = utils::read_device_string(device, || buf[7]);

    let mut output_buf = String::new();

    writeln!(&mut output_buf, "{:indent$}Interface Association:", "")?;
    indent += 2;
    writeln!(&mut output_buf, "{:indent$}bLength             {:5}", "", buf[0])?;
    writeln!(&mut output_buf, "{:indent$}bDescriptorType     {:5}", "", buf[1])?;
    writeln!(&mut output_buf, "{:indent$}bFirstInterface     {:5}", "", buf[2])?;
    writeln!(&mut output_buf, "{:indent$}bInterfaceCount     {:5}", "", buf[3])?;
    writeln!(&mut output_buf, "{:indent$}bFunctionClass      {:5}", "", buf[4])?;
    writeln!(&mut output_buf, "{:indent$}bFunctionSubClass   {:5}", "", buf[5])?;
    writeln!(&mut output_buf, "{:indent$}bFunctionProtocol   {:5}", "", buf[6])?;
    writeln!(&mut output_buf, "{:indent$}iFunction           {:5} {}", "", buf[7], istr)?;

    Ok(output_buf)
}