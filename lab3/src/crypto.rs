use crate::types::{TagBITMAPFILEHEADER, TagBITMAPINFOHEADER, TagRGBQUAD, BYTE};

fn hide_byte_into_pixel(pixel: *mut TagRGBQUAD, hyde_byte: u8) {
  unsafe {
    (*pixel).rgbBlue &= 0xFC;
    (*pixel).rgbBlue |= (hyde_byte >> 6) & 0x3;
    (*pixel).rgbGreen &= 0xFC;
    (*pixel).rgbGreen |= (hyde_byte >> 4) & 0x3;
    (*pixel).rgbRed &= 0xFC;
    (*pixel).rgbRed |= (hyde_byte >> 2) & 0x3;
    (*pixel).rgbReserved &= 0xFC;
    (*pixel).rgbReserved |= hyde_byte & 0x3;
  }
}
