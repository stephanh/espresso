//! Requests that can be sent from the driver to the ESP8266 device.

use atat::AtatCmd;
use heapless::String;

use crate::commands::responses;

/// An AT test command.
///
/// You will get an [`EmptyResponse`][EmptyResponse] if communication works
/// correctly.
///
/// [EmptyResponse]: ../responses/struct.EmptyResponse.html
#[derive(Debug)]
pub struct At;

impl AtatCmd for At {
    type CommandLen = heapless::consts::U4;
    type Response = responses::EmptyResponse;

    fn as_string(&self) -> String<Self::CommandLen> {
        String::from("AT\r\n")
    }

    fn parse(&self, resp: &str) -> Result<Self::Response, atat::Error> {
        if !resp.trim().is_empty() {
            Err(atat::Error::InvalidResponse)
        } else {
            Ok(responses::EmptyResponse)
        }
    }
}

/// Return information about the firmware version.
#[derive(Debug)]
pub struct GetFirmwareVersion;

impl AtatCmd for GetFirmwareVersion {
    type CommandLen = heapless::consts::U8;
    type Response = responses::FirmwareVersion;

    fn as_string(&self) -> String<Self::CommandLen> {
        String::from("AT+GMR\r\n")
    }

    fn parse(&self, resp: &str) -> Result<Self::Response, atat::Error> {
        let mut lines = resp.lines();

        // AT version (Example: "AT version:1.1.0.0(May 11 2016 18:09:56)")
        let at_version_raw = lines.next().ok_or(atat::Error::ParseString)?;
        if !at_version_raw.starts_with("AT version:") {
            return Err(atat::Error::ParseString);
        }
        let at_version = &at_version_raw[11..];

        // SDK version (example: "SDK version:1.5.4(baaeaebb)")
        let sdk_version_raw = lines.next().ok_or(atat::Error::ParseString)?;
        if !sdk_version_raw.starts_with("SDK version:") {
            return Err(atat::Error::ParseString);
        }
        let sdk_version = &sdk_version_raw[12..];

        // Compile time (example: "compile time:May 20 2016 15:08:19")
        let compile_time_raw = lines.next().ok_or(atat::Error::ParseString)?;
        if !compile_time_raw.starts_with("compile time:") {
            return Err(atat::Error::ParseString);
        }
        let compile_time = &compile_time_raw[13..];

        Ok(responses::FirmwareVersion {
            at_version: String::from(at_version),
            sdk_version: String::from(sdk_version),
            compile_time: String::from(compile_time),
        })
    }
}