use crate::error::Error;
use std::convert::{TryFrom, TryInto};

static HEADER_STRING: &[u8] = &[
    83, 81, 76, 105, 116, 101, 32, 102, 111, 114, 109, 97, 116, 32, 51, 0,
]; // "SQLite format 3\u{0}"

// The database page size in bytes.
#[derive(Debug)]
pub struct PageSize(u32);

pub fn validate_header_string(bytes: &[u8]) -> Result<(), Error> {
    let buf = &bytes[0..16];
    if buf != HEADER_STRING {
        return Err(Error::HeaderString(String::from_utf8_lossy(buf).to_string()))
    }
    Ok(())
}
pub fn parse_page_size(bytes: &[u8]) -> Result<PageSize, Error> {
    let page_size_bytes: [u8;2] = bytes[16..18].try_into().map_err(|_| {
        Error::InvalidPageSize(format!("expected a 2 byte slice, found: {:?}", bytes))
    })?;
    let raw_page_size = u16::from_be_bytes(page_size_bytes);
    raw_page_size.try_into()
}

impl TryFrom<u16> for PageSize {
    type Error = Error;
    fn try_from(v: u16) -> Result<PageSize, Self::Error> {
        match v {
            1 => Ok(PageSize(65_536u32)), // the value 1 representing a page size of 65536.
            0 | 2..=511 => Err(Error::InvalidPageSize(format!(
                "value must be >= 512, found: {}",
                v
            ))), // Must be a power of two between 512 and 32768 inclusive
            _ => {
                if v.is_power_of_two() {
                    Ok(PageSize(v as u32))
                }else {
                    Err(Error::InvalidPageSize(format!(
                        "value must be a power of 2 found: {}",
                        v
                    )))
                }
            }
        }
    }
}