use std::io::Read;

use bytes::BytesMut;
use tokio_io::codec::{Decoder, Encoder};

use super::bytes_mut_extending_writer::BytesMutExtendingWriter;
use super::super::errors::{Error, Result};
use super::super::record::{Record, RecordFragments, RecordReader, RecordWriter};

pub struct RecordCodec;

impl Decoder for RecordCodec {
    type Item = Record<Vec<u8>>;
    type Error = Error;

    fn decode(&mut self, buffer: &mut BytesMut) -> Result<Option<Self::Item>> {
        if !(&buffer as &[u8]).is_full_record() {
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
        let extending_writer = BytesMutExtendingWriter::from(buffer);
        let mut writer = RecordWriter::new(extending_writer);

        Ok(writer.write_record(&record)?)
    }
}
