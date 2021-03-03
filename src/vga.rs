pub const VGA_BUF: *mut u8 = 0xb8000 as *mut u8;
pub static mut VGA_OFFSET: u8 = 0;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum VgaColor {
	Black,
	Blue,
	Green,
	Cyan,
	Red,
	Magenta,
	Brown,
	Gray,
	DarkGray,
	LightBlue,
	LightGreen,
	LightCyan,
	LightRed,
	LightMagenta,
	Yellow,
	White,
}

pub fn write_char(char: u8, color: VgaColor) {
	unsafe {
		let offset = VGA_OFFSET as isize * 2;
		// character byte
		*VGA_BUF.offset(offset) = char;
		// color byte
		*VGA_BUF.offset(offset + 1) = color as u8;
		VGA_OFFSET += 1;
	}
}

pub fn write(string: &str, color: VgaColor) {
	for char in string.as_bytes() {
		write_char(*char, color);
	}
}
