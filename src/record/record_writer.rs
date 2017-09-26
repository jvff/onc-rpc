use std::io;
use std::io::Write;
use std::ops::{Deref, DerefMut};

use byteorder::{BigEndian, WriteBytesExt};

use super::record::Record;

pub struct RecordWriter<W>
where
    W: Write,
{
    writer: W,
}

impl<W> RecordWriter<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        RecordWriter { writer }
    }

    pub fn write_record<T>(&mut self, record: &Record<T>) -> io::Result<()>
    where
        T: AsRef<[u8]>,
    {
        self.writer.write_u32::<BigEndian>(record.len() as u32 | 0x8000_0000)?;
        self.writer.write_all(record)
    }
}

impl<W> Deref for RecordWriter<W>
where
    W: Write,
{
    type Target = W;

    fn deref(&self) -> &W {
        &self.writer
    }
}

impl<W> DerefMut for RecordWriter<W>
where
    W: Write,
{
    fn deref_mut(&mut self) -> &mut W {
        &mut self.writer
    }
}
