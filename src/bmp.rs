use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, Write};

pub fn write_bmp(path: &str, fb: &Framebuffer) -> io::Result<()> {
    let row_padding = (4 - (fb.width * 3) % 4) % 4;
    let row_size = fb.width * 3 + row_padding;
    let pixel_data_size = row_size * fb.height;
    let file_size = 54 + pixel_data_size;

    let mut file = File::create(path)?;

    file.write_all(b"BM")?;
    file.write_all(&(file_size as u32).to_le_bytes())?;
    file.write_all(&[0; 4])?;
    file.write_all(&(54u32).to_le_bytes())?;

    file.write_all(&(40u32).to_le_bytes())?;
    file.write_all(&(fb.width as i32).to_le_bytes())?;
    file.write_all(&(fb.height as i32).to_le_bytes())?;
    file.write_all(&(1u16).to_le_bytes())?;
    file.write_all(&(24u16).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;
    file.write_all(&(pixel_data_size as u32).to_le_bytes())?;
    file.write_all(&(2835i32).to_le_bytes())?;
    file.write_all(&(2835i32).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;
    file.write_all(&(0u32).to_le_bytes())?;

    let padding = vec![0; row_padding];

    for y in (0..fb.height).rev() {
        for x in 0..fb.width {
            let [r, g, b] = fb.pixels()[y * fb.width + x];
            file.write_all(&[b, g, r])?;
        }
        file.write_all(&padding)?;
    }

    Ok(())
}
