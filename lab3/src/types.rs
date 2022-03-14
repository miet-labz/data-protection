pub type BOOL = i32;
pub type BYTE = u8;
pub type WORD = u16;
pub type DWORD = u32;
pub type LONG = i32;
pub type INT64 = i64;
pub type UINT64 = u64;

pub struct tagBITMAPFILEHEADER {
  pub bfType: WORD,
  pub bfSize: DWORD,
  pub bfReserved1: WORD,
  pub bfReserved2: WORD,
  pub bfOffBits: DWORD,
} // 14 bytes
pub type TagBITMAPFILEHEADER = tagBITMAPFILEHEADER;

pub struct tagBITMAPINFOHEADER {
  pub biSize: DWORD,
  pub biWidth: LONG,
  pub biHeight: LONG,
  pub biPlanes: WORD,
  pub biBitCount: WORD,
  pub biCompression: DWORD,
  pub biSizeImage: DWORD,
  pub biXPelsPerMeter: LONG,
  pub biYPelsPerMeter: LONG,
  pub biClrUsed: DWORD,
  pub biClrImportant: DWORD,
} // 40 bytes
pub type TagBITMAPINFOHEADER = tagBITMAPINFOHEADER;

pub struct tagRGBQUAD {
  pub rgbBlue: BYTE,
  pub rgbGreen: BYTE,
  pub rgbRed: BYTE,
  pub rgbReserved: BYTE,
}
pub type TagRGBQUAD = tagRGBQUAD;
