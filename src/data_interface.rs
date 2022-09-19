use std::{
    fs::File,
    io::{BufReader, Cursor, Error, Read, Seek, SeekFrom},
};

#[derive(Debug)]
pub(crate) enum DataInterface {
    File(BufReader<File>),
    Bytes(Cursor<Vec<u8>>),
    _String, // TODO:
}

impl DataInterface {
    pub(crate) fn read(&mut self, len: u64) -> Result<Vec<u8>, Error> {
        let mut buf = vec![0; len as usize];
        match self {
            DataInterface::File(reader) => {
                reader.read_exact(&mut buf)?;
            }
            DataInterface::Bytes(cursor) => {
                cursor.read_exact(&mut buf)?;
            }
            DataInterface::_String => {}
        }

        Ok(buf)
    }
    pub(crate) fn seek(&mut self, pos: u64) -> Result<(), Error> {
        match self {
            DataInterface::File(reader) => {
                reader.seek(SeekFrom::Start(pos))?;
            }
            DataInterface::Bytes(cursor) => {
                cursor.seek(SeekFrom::Start(pos))?;
            }
            DataInterface::_String => {}
        }

        Ok(())
    }
    pub(crate) fn seek_relative(&mut self, offset: i64) -> Result<(), Error> {
        match self {
            DataInterface::File(reader) => {
                reader.seek_relative(offset)?;
            }
            DataInterface::Bytes(cursor) => {
                cursor.seek(SeekFrom::Current(offset))?;
            }
            DataInterface::_String => {}
        }

        Ok(())
    }
}
