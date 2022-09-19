use byteorder::ByteOrder;
use byteorder::{BigEndian, LittleEndian};

/// Read a `u16` from a buffer. (Little Endian)
pub fn read_u16(buf: &[u8]) -> u16 {
    LittleEndian::read_u16(buf)
}
/// Read a `u32` from a buffer. (Little Endian)
pub fn read_u32(buf: &[u8]) -> u32 {
    LittleEndian::read_u32(buf)
}
/// Read a `u64` from a buffer. (Little Endian)
pub fn read_u64(buf: &[u8]) -> u64 {
    LittleEndian::read_u64(buf)
}
/// Read a `u128` from a buffer. (Little Endian)
pub fn read_u128(buf: &[u8]) -> u128 {
    LittleEndian::read_u128(buf)
}

/// Read a `i16` from a buffer. (Little Endian)
pub fn read_i16(buf: &[u8]) -> i16 {
    LittleEndian::read_i16(buf)
}
/// Read a `i32` from a buffer. (Little Endian)
pub fn read_i32(buf: &[u8]) -> i32 {
    LittleEndian::read_i32(buf)
}
/// Read a `i64` from a buffer. (Little Endian)
pub fn read_i64(buf: &[u8]) -> i64 {
    LittleEndian::read_i64(buf)
}
/// Read a `i128` from a buffer. (Little Endian)
pub fn read_i128(buf: &[u8]) -> i128 {
    LittleEndian::read_i128(buf)
}

/// Read a `f32` from a buffer. (Little Endian)
pub fn read_f32(buf: &[u8]) -> f32 {
    LittleEndian::read_f32(buf)
}
/// Read a `f64` from a buffer. (Little Endian)
pub fn read_f64(buf: &[u8]) -> f64 {
    LittleEndian::read_f64(buf)
}

/// Read a `u16` from a buffer. (Big Endian)
pub fn read_u16_be(buf: &[u8]) -> u16 {
    BigEndian::read_u16(buf)
}
/// Read a `u32` from a buffer. (Big Endian)
pub fn read_u32_be(buf: &[u8]) -> u32 {
    BigEndian::read_u32(buf)
}
/// Read a `u64` from a buffer. (Big Endian)
pub fn read_u64_be(buf: &[u8]) -> u64 {
    BigEndian::read_u64(buf)
}
/// Read a `u128` from a buffer. (Big Endian)
pub fn read_u128_be(buf: &[u8]) -> u128 {
    BigEndian::read_u128(buf)
}

/// Read a `i16` from a buffer. (Big Endian)
pub fn read_i16_be(buf: &[u8]) -> i16 {
    BigEndian::read_i16(buf)
}
/// Read a `i32` from a buffer. (Big Endian)
pub fn read_i32_be(buf: &[u8]) -> i32 {
    BigEndian::read_i32(buf)
}
/// Read a `i64` from a buffer. (Big Endian)
pub fn read_i64_be(buf: &[u8]) -> i64 {
    BigEndian::read_i64(buf)
}
/// Read a `i128` from a buffer. (Big Endian)
pub fn read_i128_be(buf: &[u8]) -> i128 {
    BigEndian::read_i128(buf)
}

/// Read a `f32` from a buffer. (Big Endian)
pub fn read_f32_be(buf: &[u8]) -> f32 {
    BigEndian::read_f32(buf)
}
/// Read a `f64` from a buffer. (Big Endian)
pub fn read_f64_be(buf: &[u8]) -> f64 {
    BigEndian::read_f64(buf)
}

/// Write a `u16` to a buffer. (Little Endian)
pub fn write_u16(buf: &mut [u8], n: u16) {
    LittleEndian::write_u16(buf, n);
}
/// Write a `u32` to a buffer. (Little Endian)
pub fn write_u32(buf: &mut [u8], n: u32) {
    LittleEndian::write_u32(buf, n);
}
/// Write a `u64` to a buffer. (Little Endian)
pub fn write_u64(buf: &mut [u8], n: u64) {
    LittleEndian::write_u64(buf, n);
}
/// Write a `u64` to a buffer. (Little Endian)
pub fn write_u128(buf: &mut [u8], n: u128) {
    LittleEndian::write_u128(buf, n);
}

/// Write a `i16` to a buffer. (Little Endian)
pub fn write_i16(buf: &mut [u8], n: i16) {
    LittleEndian::write_i16(buf, n);
}
/// Write a `i32` to a buffer. (Little Endian)
pub fn write_i32(buf: &mut [u8], n: i32) {
    LittleEndian::write_i32(buf, n);
}
/// Write a `i64` to a buffer. (Little Endian)
pub fn write_i64(buf: &mut [u8], n: i64) {
    LittleEndian::write_i64(buf, n);
}
/// Write a `i64` to a buffer. (Little Endian)
pub fn write_i128(buf: &mut [u8], n: i128) {
    LittleEndian::write_i128(buf, n);
}

/// Write a `f32` to a buffer. (Little Endian)
pub fn write_f32(buf: &mut [u8], n: f32) {
    LittleEndian::write_f32(buf, n);
}
/// Write a `f64` to a buffer. (Little Endian)
pub fn write_f64(buf: &mut [u8], n: f64) {
    LittleEndian::write_f64(buf, n);
}

/// Write a `u16` to a buffer. (Big Endian)
pub fn write_u16_be(buf: &mut [u8], n: u16) {
    BigEndian::write_u16(buf, n);
}
/// Write a `u32` to a buffer. (Big Endian)
pub fn write_u32_be(buf: &mut [u8], n: u32) {
    BigEndian::write_u32(buf, n);
}
/// Write a `u64` to a buffer. (Big Endian)
pub fn write_u64_be(buf: &mut [u8], n: u64) {
    BigEndian::write_u64(buf, n);
}
/// Write a `u64` to a buffer. (Big Endian)
pub fn write_u128_be(buf: &mut [u8], n: u128) {
    BigEndian::write_u128(buf, n);
}

/// Write a `i16` to a buffer. (Big Endian)
pub fn write_i16_be(buf: &mut [u8], n: i16) {
    BigEndian::write_i16(buf, n);
}
/// Write a `i32` to a buffer. (Big Endian)
pub fn write_i32_be(buf: &mut [u8], n: i32) {
    BigEndian::write_i32(buf, n);
}
/// Write a `i64` to a buffer. (Big Endian)
pub fn write_i64_be(buf: &mut [u8], n: i64) {
    BigEndian::write_i64(buf, n);
}
/// Write a `i64` to a buffer. (Big Endian)
pub fn write_i128_be(buf: &mut [u8], n: i128) {
    BigEndian::write_i128(buf, n);
}

/// Write a `f32` to a buffer. (Big Endian)
pub fn write_f32_be(buf: &mut [u8], n: f32) {
    BigEndian::write_f32(buf, n);
}
/// Write a `f64` to a buffer. (Big Endian)
pub fn write_f64_be(buf: &mut [u8], n: f64) {
    BigEndian::write_f64(buf, n);
}