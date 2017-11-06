use std::io;
use std::io::Write;

use bytes::BytesMut;

pub struct BytesMutExtendingWriter<'b> {
    bytes: &'b mut BytesMut,
}

impl<'b> From<&'b mut BytesMut> for BytesMutExtendingWriter<'b> {
    fn from(bytes: &'b mut BytesMut) -> Self {
        BytesMutExtendingWriter { bytes }
    }
}

impl<'b> Write for BytesMutExtendingWriter<'b> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.bytes.extend_from_slice(buffer);

        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
