//! Responses from the ESP8266 device.

use atat::AtatResp;
use heapless::consts::U32;
use heapless::String;

/// An empty response, no body.
#[derive(Debug)]
pub struct EmptyResponse;

impl AtatResp for EmptyResponse {}

/// Firmware version.
#[derive(Debug)]
pub struct FirmwareVersion {
    pub at_version: String<U32>,
    pub sdk_version: String<U32>,
    pub compile_time: String<U32>,
}

impl AtatResp for FirmwareVersion {}
