use crate::framebuffer::Framebuffer;
use std::fs::File;
use std::io::{self, Write};

pub fn write_png(path: &str, fb: &Framebuffer) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(&[137, 80, 78, 71, 13, 10, 26, 10])?;

    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&(fb.width as u32).to_be_bytes());
    ihdr.extend_from_slice(&(fb.height as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 2, 0, 0, 0]);
    write_chunk(&mut file, b"IHDR", &ihdr)?;

    let mut raw = Vec::with_capacity((fb.width * 3 + 1) * fb.height);
    for y in 0..fb.height {
        raw.push(0);
        for x in 0..fb.width {
            raw.extend_from_slice(&fb.pixels()[y * fb.width + x]);
        }
    }

    let compressed = zlib_store(&raw);
    write_chunk(&mut file, b"IDAT", &compressed)?;
    write_chunk(&mut file, b"IEND", &[])?;

    Ok(())
}

fn write_chunk(file: &mut File, kind: &[u8; 4], data: &[u8]) -> io::Result<()> {
    file.write_all(&(data.len() as u32).to_be_bytes())?;
    file.write_all(kind)?;
    file.write_all(data)?;

    let mut crc_data = Vec::with_capacity(kind.len() + data.len());
    crc_data.extend_from_slice(kind);
    crc_data.extend_from_slice(data);
    file.write_all(&crc32(&crc_data).to_be_bytes())?;

    Ok(())
}

fn zlib_store(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&[0x78, 0x01]);

    let mut start = 0;
    while start < data.len() {
        let remaining = data.len() - start;
        let block_len = remaining.min(65_535);
        let is_last = start + block_len == data.len();
        let len = block_len as u16;

        out.push(if is_last { 1 } else { 0 });
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(&data[start..start + block_len]);
        start += block_len;
    }

    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65_521;
    let mut a = 1;
    let mut b = 0;

    for byte in data {
        a = (a + *byte as u32) % MOD;
        b = (b + a) % MOD;
    }

    (b << 16) | a
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffff;

    for byte in data {
        crc ^= *byte as u32;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0xedb8_8320;
            } else {
                crc >>= 1;
            }
        }
    }

    !crc
}
