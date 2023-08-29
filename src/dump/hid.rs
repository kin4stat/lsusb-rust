/*fn dump_hid_device(device: Option<&mut rusb::DeviceHandle<rusb::Context>>, interface: &rusb::InterfaceDescriptor, buf: &[u8]) {
    if buf[1] != constants::LIBUSB_DT_HID {
        println!("      Warning: Invalid descriptor");
    } else if buf[0] < 6 + 3 * buf[5] {
        println!("      Warning: Descriptor too short");
    }
    println!("        HID Device Descriptor:");
    println!("          bLength             {:5}", buf[0]);
    println!("          bDescriptorType     {:5}", buf[1]);
    println!("          bcdHID              {:2x}.{:02x}", buf[3], buf[2]);
    println!("          bCountryCode        {:5}", buf[4]);
    println!("          bNumDescriptors     {:5}", buf[5]);

    for i in 0..buf[5] {
        println!("          bDescriptorType     {:5}", buf[6 + 3 * i]);
        println!("          wDescriptorLength   {:5}", buf[7 + 3 * i] | (buf[8 + 3 * i] << 8));
    }

    let mut rdbuf = [0; 8192];

    for i in 0..buf[5] {
        if buf[6 + 3 * i as usize] != constants::LIBUSB_DT_REPORT {
            continue;
        }
        let len = (buf[7 + 3 * i as usize] as usize) | ((buf[8 + 3 * i as usize] as usize) << 8);

        if len > rdbuf.len() {
            println!("report descriptor too long");
            continue;
        }

        if let x = device.claim_interface(interface.interface_number()) {
            let mut retries = 8;
            let mut n = 0;

            while n < len && (retries - 1) > 0 {
                n = device.read_control(Recipient::Interface as u8 |
                                            RequestType::Standard as u8 |
                                            constants::LIBUSB_ENDPOINT_IN,
                                        constants::LIBUSB_REQUEST_GET_DESCRIPTOR,
                                        (constants::LIBUSB_DT_REPORT as u16) << 8,
                                        interface.interface_number() as u16,
                                        &mut rdbuf,
                                        CTRL_TIMEOUT).map_or(-1, |x| x);
                retries -= 1;
            }

            if n > 0 {
                if n < len {
                    println!("          Warning: incomplete report descriptor");
                }
                dump_report_desc(dbuf, n);
            }
        } else {
            println!("          Report Descriptors: ");
            println!("            ** UNAVAILABLE **");
        }
    }
}
*/