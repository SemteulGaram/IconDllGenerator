#[cfg(windows)] extern crate winapi;
use std::io::Error;

use winapi::{
	um::{
		winuser,
		wingdi,
	},
	shared::windef::{
		HICON__,
		RECT
	},
	ctypes::c_void
};

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
		use std::ffi::OsStr;
		use std::iter::once;
		use std::os::windows::ffi::OsStrExt;
		use std::ptr::null_mut;
		let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
		let ret = unsafe {
			winuser::MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), winuser::MB_OK)
		};
		if ret == 0 { Err(Error::last_os_error()) }
		else { Ok(ret) }
}
#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
		println!("{}", msg);
		Ok(())
}

fn icon_default_question() -> *mut HICON__ {
	return unsafe { winuser::LoadIconW(std::ptr::null_mut(), winuser::IDI_QUESTION) };
}

fn icon_manual_test() -> *mut HICON__ { 
	// Yang icon AND bitmask 
	let ANDmaskIcon = [0xFF, 0xFF, 0xFF, 0xFF,   // line 1 
																0xFF, 0xFF, 0xC3, 0xFF,   // line 2 
																0xFF, 0xFF, 0x00, 0xFF,   // line 3 
																0xFF, 0xFE, 0x00, 0x7F,   // line 4 

																0xFF, 0xFC, 0x00, 0x1F,   // line 5 
																0xFF, 0xF8, 0x00, 0x0F,   // line 6 
																0xFF, 0xF8, 0x00, 0x0F,   // line 7 
																0xFF, 0xF0, 0x00, 0x07,   // line 8 

																0xFF, 0xF0, 0x00, 0x03,   // line 9 
																0xFF, 0xE0, 0x00, 0x03,   // line 10 
																0xFF, 0xE0, 0x00, 0x01,   // line 11 
																0xFF, 0xE0, 0x00, 0x01,   // line 12 

																0xFF, 0xF0, 0x00, 0x01,   // line 13 
																0xFF, 0xF0, 0x00, 0x00,   // line 14 
																0xFF, 0xF8, 0x00, 0x00,   // line 15 
																0xFF, 0xFC, 0x00, 0x00,   // line 16 

																0xFF, 0xFF, 0x00, 0x00,   // line 17 
																0xFF, 0xFF, 0x80, 0x00,   // line 18 
																0xFF, 0xFF, 0xE0, 0x00,   // line 19 
																0xFF, 0xFF, 0xE0, 0x01,   // line 20 

																0xFF, 0xFF, 0xF0, 0x01,   // line 21 
																0xFF, 0xFF, 0xF0, 0x01,   // line 22 
																0xFF, 0xFF, 0xF0, 0x03,   // line 23 
																0xFF, 0xFF, 0xE0, 0x03,   // line 24 

																0xFF, 0xFF, 0xE0, 0x07,   // line 25 
																0xFF, 0xFF, 0xC0, 0x0F,   // line 26 
																0xFF, 0xFF, 0xC0, 0x0F,   // line 27 
																0xFF, 0xFF, 0x80, 0x1F,   // line 28 

																0xFF, 0xFF, 0x00, 0x7F,   // line 29 
																0xFF, 0xFC, 0x00, 0xFF,   // line 30 
																0xFF, 0xF8, 0x03, 0xFF,   // line 31 
																0xFF, 0xFC, 0x3F, 0xFF];  // line 32
// Yang icon XOR bitmask 
let XORmaskIcon = [0x00, 0x00, 0x00, 0x00,   // line 1 
																0x00, 0x00, 0x00, 0x00,   // line 2 
																0x00, 0x00, 0x00, 0x00,   // line 3 
																0x00, 0x00, 0x00, 0x00,   // line 4 

																0x00, 0x00, 0x00, 0x00,   // line 5 
																0x00, 0x00, 0x00, 0x00,   // line 6 
																0x00, 0x00, 0x00, 0x00,   // line 7 
																0x00, 0x00, 0x38, 0x00,   // line 8 

																0x00, 0x00, 0x7C, 0x00,   // line 9 
																0x00, 0x00, 0x7C, 0x00,   // line 10 
																0x00, 0x00, 0x7C, 0x00,   // line 11 
																0x00, 0x00, 0x38, 0x00,   // line 12 

																0x00, 0x00, 0x00, 0x00,   // line 13 
																0x00, 0x00, 0x00, 0x00,   // line 14 
																0x00, 0x00, 0x00, 0x00,   // line 15 
																0x00, 0x00, 0x00, 0x00,   // line 16 

																0x00, 0x00, 0x00, 0x00,   // line 17 
																0x00, 0x00, 0x00, 0x00,   // line 18 
																0x00, 0x00, 0x00, 0x00,   // line 19 
																0x00, 0x00, 0x00, 0x00,   // line 20 

																0x00, 0x00, 0x00, 0x00,   // line 21 
																0x00, 0x00, 0x00, 0x00,   // line 22 
																0x00, 0x00, 0x00, 0x00,   // line 23 
																0x00, 0x00, 0x00, 0x00,   // line 24 

																0x00, 0x00, 0x00, 0x00,   // line 25 
																0x00, 0x00, 0x00, 0x00,   // line 26 
																0x00, 0x00, 0x00, 0x00,   // line 27 
																0x00, 0x00, 0x00, 0x00,   // line 28 

																0x00, 0x00, 0x00, 0x00,   // line 29 
																0x00, 0x00, 0x00, 0x00,   // line 30 
																0x00, 0x00, 0x00, 0x00,   // line 31 
																0x00, 0x00, 0x00, 0x00];  // line 32 

	return unsafe {
		winuser::CreateIcon(std::ptr::null_mut(),    // application instance  
							32,                          // icon width 
							32,                         // icon height
							1,                          // number of XOR planes 
							1,                       // number of bits per pixel 
							ANDmaskIcon.as_ptr(),    // AND bitmask  
							XORmaskIcon.as_ptr())    // XOR bitmask 
	}
}

fn get_bmp_from_icon(hicon: *mut HICON__) -> Result<(wingdi::BITMAPINFO, Vec<u8>), Error> {
	unsafe {
		// Create an ICONINFO struct and initialize it
		let mut icon_info: winuser::ICONINFO = std::mem::zeroed();
		// Get the bitmap information associated with the icon
		if winuser::GetIconInfo(hicon, &mut icon_info) == 0 {
			return Err(Error::last_os_error());
		}

		// Get the color bitmap handle
		let hbm = icon_info.hbmColor;
		if hbm.is_null() {
			return Err(Error::last_os_error());
		}

		// Convert HBITMAP to file
		let mut bmp: wingdi::BITMAP = std::mem::zeroed();
		if wingdi::GetObjectW(
			hbm as *mut c_void, 
			std::mem::size_of_val(&bmp) as i32, 
			&mut bmp as *mut _ as *mut c_void
		) == 0 {
			return Err(Error::last_os_error());
		}
		let mut bmp_info: wingdi::BITMAPINFO = std::mem::zeroed();
		// bmp_info.bmiHeader.biSize = std::mem::size_of_val(&bmp_info.bmiHeader) as u32;
		bmp_info.bmiHeader.biSize = std::mem::size_of::<wingdi::BITMAPINFOHEADER>() as u32;
		bmp_info.bmiHeader.biWidth = bmp.bmWidth;
		bmp_info.bmiHeader.biHeight = bmp.bmHeight;
		bmp_info.bmiHeader.biPlanes = 1;
		bmp_info.bmiHeader.biBitCount = bmp.bmBitsPixel;
		bmp_info.bmiHeader.biCompression = wingdi::BI_RGB;

		let width = bmp.bmWidth;
		let height = bmp.bmHeight;
		let mut bitmap_data: Vec<u8> = vec![0; (width * height * 4) as usize];

		let hdc = winuser::GetDC(std::ptr::null_mut());
		if hdc.is_null() {
			return Err(Error::last_os_error());
		}
		wingdi::GetDIBits(
			hdc,
			hbm,
			0,
			height as u32,
			bitmap_data.as_mut_ptr() as *mut c_void, // OUT
			&mut bmp_info, // IN,OUT
			wingdi::DIB_RGB_COLORS
		);

		wingdi::DeleteObject(hbm as *mut c_void);
		wingdi::DeleteObject(icon_info.hbmMask as *mut c_void);
		return Ok((bmp_info, bitmap_data));
	}
}

fn save_bitmap_to_bmp_file(
	file_path: &str,
	bmp_info: &wingdi::BITMAPINFO,
	bitmap_data: &[u8],
) -> std::io::Result<()> {
	use std::fs::File;
	use std::io::Write;

	// Open the file for writing
	let mut file = File::create(file_path)?;

	// Write the BITMAPFILEHEADER
	let file_header = wingdi::BITMAPFILEHEADER {
			bfType: 0x4D42, // 'BM' in little-endian
			bfSize: std::mem::size_of::<wingdi::BITMAPFILEHEADER>() as u32 + bmp_info.bmiHeader.biSizeImage,
			bfReserved1: 0,
			bfReserved2: 0,
			bfOffBits: std::mem::size_of::<wingdi::BITMAPFILEHEADER>() as u32 + bmp_info.bmiHeader.biSize,
	};

	file.write_all(&file_header.bfType.to_le_bytes())?;
	file.write_all(&file_header.bfSize.to_le_bytes())?;
	file.write_all(&file_header.bfReserved1.to_le_bytes())?;
	file.write_all(&file_header.bfReserved2.to_le_bytes())?;
	file.write_all(&file_header.bfOffBits.to_le_bytes())?;

	// Write the BITMAPINFOHEADER
	file.write_all(&bmp_info.bmiHeader.biSize.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biWidth.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biHeight.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biPlanes.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biBitCount.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biCompression.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biSizeImage.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biXPelsPerMeter.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biYPelsPerMeter.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biClrUsed.to_le_bytes())?;
	file.write_all(&bmp_info.bmiHeader.biClrImportant.to_le_bytes())?;

	// Write the bitmap data
	file.write_all(bitmap_data)?;

	Ok(())
}


fn main() {
	let hicon1 = icon_default_question();
	let (bmp_info, bmp_vec) = get_bmp_from_icon(hicon1).unwrap();
	save_bitmap_to_bmp_file("icon1.bmp", &bmp_info, bmp_vec.as_slice()).unwrap();
}
