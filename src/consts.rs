use rusb::constants;
use std::time::Duration;

pub const CTRL_TIMEOUT: Duration = Duration::new(5, 0);
pub const READ_STR_TIMEOUT: Duration = Duration::new(1, 0);
pub const BILLBOARD_MAX_NUM_ALT_MODE: u8 = 0x34;

pub const USB_CLASS_CCID: u8 = 0x0b;
pub const USB_AUDIO_CLASS_2: u8 = 0x20;
pub const USB_AUDIO_CLASS_3: u8 = 0x30;

pub const USB_DT_DEVICE_QUALIFIER: u16 = 0x06;
pub const USB_DT_OTG: u8 = 0x09;
pub const USB_DT_DEBUG: u8 = 0x0a;
pub const USB_DT_INTERFACE_ASSOCIATION: u8 = 0x0b;

pub const USB_DT_CS_INTERFACE: u8 = constants::LIBUSB_REQUEST_TYPE_CLASS | constants::LIBUSB_DT_INTERFACE;
pub const USB_DT_CS_ENDPOINT: u8 = constants::LIBUSB_REQUEST_TYPE_CLASS | constants::LIBUSB_DT_ENDPOINT;
pub const USB_DT_CS_DEVICE: u8 = constants::LIBUSB_REQUEST_TYPE_CLASS | constants::LIBUSB_DT_DEVICE;

pub const USB_DT_SS_ENDPOINT_COMP: u8 = 0x30;

pub const USB_DC_WIRELESS_USB: u8 = 0x01;
pub const USB_DC_20_EXTENSION: u8 = 0x02;
pub const USB_DC_SUPERSPEED: u8 = 0x03;
pub const USB_DC_CONTAINER_ID: u8 = 0x04;
pub const USB_DC_PLATFORM: u8 = 0x05;
pub const USB_DC_SUPERSPEEDPLUS: u8 = 0x0a;
pub const USB_DC_BILLBOARD: u8 = 0x0d;
pub const USB_DC_BILLBOARD_ALT_MODE: u8 = 0x0f;
pub const USB_DC_CONFIGURATION_SUMMARY: u8 = 0x10;

pub const WEBUSB_GUID: &str = "{3408b638-09a9-47a0-8bfd-a0768815b665}";