use super::{RecordGen, HeaderGen, Error, Result};
use super::utils::{unknown_field, set_field_u32};

use crate::msg_iter::{ChunkMessagesIterator, ChunkRecordsIterator};
use crate::cursor::Cursor;

/// Compression options for `Chunk` data.
#[derive(Debug, Clone, Copy)]
pub enum Compression {
    Bzip2,
    None,
}

/// Bulk storage with optional compression for messages data and connection
/// records.
#[derive(Debug, Clone)]
pub struct Chunk<'a> {
    /// Compression type for the data
    pub compression: Compression,
    /// Decompressed messages data and connection records
    data: &'a [u8],
}

impl<'a> Chunk<'a> {
    /// Get iterator over only messages
    pub fn iter_msgs(&self) -> ChunkMessagesIterator<'a> {
        ChunkMessagesIterator::new(&self.data)
    }

    /// Get iterator over all internall records.
    pub fn iter(&self) -> ChunkRecordsIterator<'a> {
        ChunkRecordsIterator::new(&self.data)
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ChunkHeader {
    compression: Option<Compression>,
    size: Option<u32>,
}

impl<'a> RecordGen<'a> for Chunk<'a> {
    type Header = ChunkHeader;

    fn read_data(c: &mut Cursor<'a>, header: Self::Header) -> Result<Self> {
        let compression = header.compression.ok_or(Error::InvalidHeader)?;
        if !matches!(compression, Compression::None) {
            return Err(Error::UnsupportedCompression);
        }
        let size = header.size.ok_or(Error::InvalidHeader)?;
        let data = c.next_chunk()?;
        if data.len() != size as usize {
            return Err(Error::InvalidRecord);
        }
        Ok(Self { compression, data })
    }
}

impl<'a> HeaderGen<'a> for ChunkHeader {
    const OP: u8 = 0x05;

    fn process_field(&mut self, name: &str, val: &[u8]) -> Result<()> {
        match name {
            "compression" => {
                if self.compression.is_some() {
                    return Err(Error::InvalidHeader);
                }
                self.compression = Some(match val {
                    b"none" => Compression::None,
                    b"bzip2" => Compression::Bzip2,
                    _ => return Err(Error::InvalidHeader),
                });
            },
            "size" => set_field_u32(&mut self.size, val)?,
            _ => unknown_field(name, val),
        }
        Ok(())
    }
}
