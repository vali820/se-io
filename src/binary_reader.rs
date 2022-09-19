use std::{
    fs::File,
    io::{BufReader, Cursor, Error, ErrorKind},
};

use super::serialization::*;

/// Binary data reader
#[derive(Debug)]
pub struct BinaryReader {
    is_file: bool,
    size: u64,
    position: i64,
    inner: super::DataInterface,
}

impl BinaryReader {
    /// Create a `BinaryReader` from a file path.
    pub fn from_path(path: &str) -> Result<Self, Error> {
        let file = File::open(path)?;
        let size = file.metadata()?.len();
        let inner = super::DataInterface::File(BufReader::new(file));

        Ok(Self {
            is_file: true,
            size,
            position: 0,
            inner,
        })
    }
    /// Create a `BinaryReader` from a `File`.
    pub fn from_file(file: File) -> Result<Self, Error> {
        let size = file.metadata()?.len();
        let inner = super::DataInterface::File(BufReader::new(file));

        Ok(Self {
            is_file: true,
            size,
            position: 0,
            inner,
        })
    }
    /// Create a `BinaryReader` from a `u8` slice.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let size = bytes.len() as u64;
        let inner = super::DataInterface::Bytes(Cursor::new(bytes.to_vec()));

        Self {
            is_file: true,
            size,
            position: 0,
            inner,
        }
    }
    /// Read a specified number of bytes.
    pub fn read(&mut self, len: u64) -> Result<Vec<u8>, Error> {
        match self.inner.read(len) {
            Ok(b) => {
                self.position += len as i64;
                Ok(b)
            }
            Err(e) => Err(e),
        }
    }
    /// Seek to a specified position in the data.
    pub fn seek(&mut self, pos: u64) -> Result<(), Error> {
        match self.inner.seek(pos) {
            Ok(()) => {
                self.position = pos as i64;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    /// Seek a specified amount of bytes in the data.
    pub fn seek_relative(&mut self, offset: i64) -> Result<(), Error> {
        match self.inner.seek_relative(offset) {
            Ok(()) => {
                self.position += offset;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Return the current position in the data.
    pub fn pos(&self) -> u64 {
        self.position as u64
    }
    /// Returns `true` if the reader is reading from a real file and `false` if it is reading from a buffer in memory.
    pub fn is_file(&self) -> bool {
        self.is_file
    }
    /// Returns size of the data.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Read a `u8` from the data.
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        Ok(self.read(1)?[0])
    }
    /// Read a `u16` from the data. (Little Endian)
    pub fn read_u16(&mut self) -> Result<u16, Error> {
        Ok(read_u16(&self.read(2)?))
    }
    /// Read a `u32` from the data. (Little Endian)
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        Ok(read_u32(&self.read(4)?))
    }
    /// Read a `u64` from the data. (Little Endian)
    pub fn read_u64(&mut self) -> Result<u64, Error> {
        Ok(read_u64(&self.read(8)?))
    }
    /// Read a `u128` from the data. (Little Endian)
    pub fn read_u128(&mut self) -> Result<u128, Error> {
        Ok(read_u128(&self.read(16)?))
    }

    /// Read a `i16` from the data. (Little Endian)
    pub fn read_i16(&mut self) -> Result<i16, Error> {
        Ok(read_i16(&self.read(2)?))
    }
    /// Read a `i32` from the data. (Little Endian)
    pub fn read_i32(&mut self) -> Result<i32, Error> {
        Ok(read_i32(&self.read(4)?))
    }
    /// Read a `i64` from the data. (Little Endian)
    pub fn read_i64(&mut self) -> Result<i64, Error> {
        Ok(read_i64(&self.read(8)?))
    }
    /// Read a `i128` from the data. (Little Endian)
    pub fn read_i128(&mut self) -> Result<i128, Error> {
        Ok(read_i128(&self.read(16)?))
    }

    /// Read a `f32` from the data. (Little Endian)
    pub fn read_f32(&mut self) -> Result<f32, Error> {
        Ok(read_f32(&self.read(4)?))
    }
    /// Read a `f64` from the data. (Little Endian)
    pub fn read_f64(&mut self) -> Result<f64, Error> {
        Ok(read_f64(&self.read(8)?))
    }

    /// Read a `u16` from the data. (Big Endian)
    pub fn read_u16_be(&mut self) -> Result<u16, Error> {
        Ok(read_u16_be(&self.read(2)?))
    }
    /// Read a `u32` from the data. (Big Endian)
    pub fn read_u32_be(&mut self) -> Result<u32, Error> {
        Ok(read_u32_be(&self.read(4)?))
    }
    /// Read a `u64` from the data. (Big Endian)
    pub fn read_u64_be(&mut self) -> Result<u64, Error> {
        Ok(read_u64_be(&self.read(8)?))
    }
    /// Read a `u128` from the data. (Big Endian)
    pub fn read_u128_be(&mut self) -> Result<u128, Error> {
        Ok(read_u128_be(&self.read(16)?))
    }

    /// Read a `i16` from the data. (Big Endian)
    pub fn read_i16_be(&mut self) -> Result<i16, Error> {
        Ok(read_i16_be(&self.read(2)?))
    }
    /// Read a `i32` from the data. (Big Endian)
    pub fn read_i32_be(&mut self) -> Result<i32, Error> {
        Ok(read_i32_be(&self.read(4)?))
    }
    /// Read a `i64` from the data. (Big Endian)
    pub fn read_i64_be(&mut self) -> Result<i64, Error> {
        Ok(read_i64_be(&self.read(8)?))
    }
    /// Read a `i128` from the data. (Big Endian)
    pub fn read_i128_be(&mut self) -> Result<i128, Error> {
        Ok(read_i128_be(&self.read(16)?))
    }

    /// Read a `f32` from the data. (Big Endian)
    pub fn read_f32_be(&mut self) -> Result<f32, Error> {
        Ok(read_f32_be(&self.read(4)?))
    }
    /// Read a `f64` from the data. (Big Endian)
    pub fn read_f64_be(&mut self) -> Result<f64, Error> {
        Ok(read_f64_be(&self.read(8)?))
    }

    /// Read a string of a specified length from the data. Fails if it encounters a non-ascii character.
    pub fn read_string_ascii_limited(&mut self, len: u64) -> Result<String, Error> {
        let buf = self.read(len)?;
        let mut out = String::new();
        for c in buf {
            if c.is_ascii() {
                out.push(c as char);
            } else {
                self.seek_relative(-(len as i64))?;
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Found non-ascii character while reading string",
                ));
            }
        }
        Ok(out)
    }

    /// Read a string from the data until it encounters a NUL(0x00)/non-ascii character or the end of the file. Doesn't fail.
    pub fn read_string_ascii(&mut self) -> String {
        let mut out = String::new();
        let mut read = true;
        while read {
            let c = self.read_u8();
            match c {
                Ok(c) => {
                    if c.is_ascii() && c != 0x00 {
                        out.push(c as char);
                    } else {
                        read = false;
                    }
                }
                Err(_) => read = false,
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_reader_read() {
        let mut reader = BinaryReader::from_bytes(&vec![0, 0, 1, 2, 3, 67, 89, 44]);
        assert_eq!(reader.read(2).unwrap(), vec![0, 0]);
        assert_eq!(reader.pos(), 2);
        reader.seek(4).unwrap();
        assert_eq!(reader.read(2).unwrap(), vec![3, 67]);
        assert_eq!(reader.pos(), 6);
        reader.seek_relative(-3).unwrap();
        assert_eq!(reader.read(3).unwrap(), vec![2, 3, 67]);
    }
    #[test]
    fn test_binary_reader_read_strings() {
        let mut reader = BinaryReader::from_bytes(&vec![
            0x61, 0x70, 0x70, 0x6c, 0x65, 0x73, 0x00, // "apples\0"
            0x62, 0x61, 0x6e, 0x61, 0x6e, 0x61, 0x73, 0x64, // "bananas"
            0x61, 0x74, 0x61, 0x00, 0x0a, // "data\0\n"
        ]);

        let string1 = "apples".to_string();
        let string2 = "bananas".to_string();
        let string3 = "data\0\n".to_string();

        assert_eq!(reader.read_string_ascii(), string1);
        assert_eq!(reader.read_string_ascii_limited(7).unwrap(), string2);
        assert_eq!(reader.read_string_ascii_limited(6).unwrap(), string3);
    }
}
