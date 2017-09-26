use std::io::{Read, Write};

use bytes::{BufMut, BytesMut};
use tokio_io::codec::{Decoder, Encoder};

use super::super::errors::{Error, Result};
use super::super::record::{Record, RecordFragments, RecordReader};

pub struct RecordCodec;

impl Decoder for RecordCodec {
    type Item = Record<Vec<u8>>;
    type Error = Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>> {
        if buffer.len() < 4 {
            return Ok(None);
        }

        let full_length = {
            let bytes: &[u8] = buffer;

            bytes.full_record_length()
        };

        if let Some(full_length) = full_length {
            let bytes = buffer.split_to(full_length);
            let mut record_bytes = Vec::new();
            let mut reader = RecordReader::new(bytes);

            reader.read_to_end(&mut record_bytes)?;

            Ok(Some(record_bytes.into()))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for RecordCodec {
    type Item = Record<Vec<u8>>;
    type Error = Error;

    fn encode(
        &mut self,
        record: Self::Item,
        buffer: &mut BytesMut,
    ) -> Result<()> {
        let mut writer = buffer.writer();

        Ok(writer.write_all(&record)?)
    }
}
