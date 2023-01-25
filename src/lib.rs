#[cfg(target_os = "macos")]
mod macos;

#[macro_use]
extern crate napi_derive;
use napi::bindgen_prelude::*;
use napi::Result;

#[napi]
pub fn read_image() -> Result<Buffer> {
    #[cfg(target_os = "macos")]
    match macos::read_image() {
        Ok(data) => Ok(Buffer::from(data)),
        Err(err) => Err(err.into()),
    }
}

#[napi]
pub fn write_image(data: Buffer) -> Result<()> {
    #[cfg(target_os = "macos")]
    macos::write_image(data.to_vec()).map_err(|err| err.into())
}
