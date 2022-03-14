pub type BYTE = u8;


pub struct TagRGBQUAD {
  pub rgb_blue: BYTE,
  pub rgb_green: BYTE,
  pub rgb_red: BYTE,
  pub rgb_reserved: BYTE,
}

impl TagRGBQUAD{
  
  pub fn read(data:Vec<BYTE>) -> Vec<TagRGBQUAD>{
    let pixels_len = (data.len() / 4) as usize;
    let mut pixels_vec:Vec<TagRGBQUAD> = Vec::new();
    for i in 0..pixels_len{
      pixels_vec.push(
        TagRGBQUAD{
          rgb_blue: data[4 * i], 
          rgb_green: data[4 * i + 1], 
          rgb_red: data[4 * i + 2], 
          rgb_reserved: data[4 * i + 3]
        }
      );
    }
    pixels_vec
  }
  pub fn to_bytes(&self) -> Vec<BYTE>{
    let result = vec!(self.rgb_blue, self.rgb_green, self.rgb_red, self.rgb_reserved);
    result
  }
}
// pub struct tagBITMAPFILEHEADER {
//   pub bfType: WORD,
//   pub bfSize: DWORD,
//   pub bfReserved1: WORD,
//   pub bfReserved2: WORD,
//   pub bfOffBits: DWORD,
// } // 14 bytes
// pub type TagBITMAPFILEHEADER = tagBITMAPFILEHEADER;

// pub struct tagBITMAPINFOHEADER {
//   pub biSize: DWORD,
//   pub biWidth: LONG,
//   pub biHeight: LONG,
//   pub biPlanes: WORD,
//   pub biBitCount: WORD,
//   pub biCompression: DWORD,
//   pub biSizeImage: DWORD,
//   pub biXPelsPerMeter: LONG,
//   pub biYPelsPerMeter: LONG,
//   pub biClrUsed: DWORD,
//   pub biClrImportant: DWORD,
// } // 40 bytes
// pub type TagBITMAPINFOHEADER = tagBITMAPINFOHEADER;
