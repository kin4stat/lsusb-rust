use rusb;
use crate::utils;
use std::fmt::Write;

pub fn dump(device: Option<&rusb::DeviceHandle<rusb::Context>>, device_descriptor: &rusb::DeviceDescriptor, mut indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    let usb_ver = device_descriptor.usb_version();
    let device_ver = device_descriptor.device_version();

    let serial = utils::read_device_serial_string(device, device_descriptor);

    let manufacturer = utils::read_device_manufacturer_string(device, device_descriptor);

    let product = utils::read_device_product_string(device, device_descriptor);

    writeln!(&mut output_buf, "{:indent$}Device Descriptor:", "")?;
    indent += 2;

    writeln!(&mut output_buf, "{:indent$}bcdUSB              {}.{}.{}", "", usb_ver.major(), usb_ver.minor(), usb_ver.sub_minor())?;
    writeln!(&mut output_buf, "{:indent$}bDeviceClass        {:5}", "", device_descriptor.class_code())?;
    writeln!(&mut output_buf, "{:indent$}bDeviceSubClass     {:5}", "", device_descriptor.sub_class_code())?;
    writeln!(&mut output_buf, "{:indent$}bDeviceProtocol     {:5}", "", device_descriptor.protocol_code())?;
    writeln!(&mut output_buf, "{:indent$}bMaxPacketSize0     {:5}", "", device_descriptor.max_packet_size())?;
    writeln!(&mut output_buf, "{:indent$}idVendor           0x{:04x}", "", device_descriptor.vendor_id())?;
    writeln!(&mut output_buf, "{:indent$}idProduct          0x{:04x}", "", device_descriptor.product_id())?;
    writeln!(&mut output_buf, "{:indent$}bcdDevice         {:3}.{}.{}", "", device_ver.major(), device_ver.minor(), device_ver.sub_minor())?;
    writeln!(&mut output_buf, "{:indent$}iManufacturer       {:5} {}", "", device_descriptor.manufacturer_string_index().map_or(0u8, |v| v), manufacturer)?;
    writeln!(&mut output_buf, "{:indent$}iProduct            {:5} {}", "", device_descriptor.product_id(), product)?;
    writeln!(&mut output_buf, "{:indent$}iSerial             {:5} {}", "", device_descriptor.serial_number_string_index().map_or(0u8, |v| v), serial)?;
    writeln!(&mut output_buf, "{:indent$}bNumConfigurations  {:5}", "", device_descriptor.num_configurations())?;

    Ok(output_buf)
}