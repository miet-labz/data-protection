use crate::types::{TagRGBQUAD, BYTE};

pub fn hide_byte_into_pixel(pixel: *mut TagRGBQUAD, hyde_byte: u8) {
  unsafe {
    (*pixel).rgb_blue &= 0xFC;
    (*pixel).rgb_blue |= (hyde_byte >> 6) & 0x3;
    (*pixel).rgb_green &= 0xFC;
    (*pixel).rgb_green |= (hyde_byte >> 4) & 0x3;
    (*pixel).rgb_red &= 0xFC;
    (*pixel).rgb_red |= (hyde_byte >> 2) & 0x3;
    (*pixel).rgb_reserved &= 0xFC;
    (*pixel).rgb_reserved |= hyde_byte & 0x3;
  }
}
pub fn get_hidden_byte(pixel: TagRGBQUAD) ->  BYTE {
  let mut result = pixel.rgb_blue & 0x3;
  result = result << 2 | pixel.rgb_green & 0x3;
  result = result << 2 | pixel.rgb_red & 0x3;
  result = result << 2 | pixel.rgb_reserved & 0x3;
  result
}
