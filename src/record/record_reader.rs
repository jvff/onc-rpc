use std::cmp::min;
use std::io;
use std::io::Read;

use super::record_fragments::RecordFragments;

pub struct RecordReader<T>
where
    T: RecordFragments,
{
    fragment: Option<T>,
    fragment_position: usize,
}

impl<T> RecordReader<T>
where
    T: RecordFragments,
{
    pub fn new(record_fragments: T) -> Self {
        RecordReader {
            fragment: Some(record_fragments),
            fragment_position: 0,
        }
    }

    fn remaining_bytes_in_fragment(&self) -> usize {
        if let Some(ref fragment) = self.fragment {
            fragment.length() - self.fragment_position
        } else {
            0
        }
    }

    fn reached_end_of_fragment(&self) -> bool {
        if let Some(ref fragment) = self.fragment {
            self.fragment_position == fragment.length()
        } else {
            true
        }
    }

    fn advance_fragment(&mut self) {
        if let Some(fragment) = self.fragment.take() {
            if fragment.is_last() {
                self.fragment = None;
            } else {
                self.fragment = Some(fragment.next());
            }

            self.fragment_position = 0;
        }
    }

    fn copy_bytes(&mut self, buffer_slice: &mut [u8]) {
        if let Some(ref fragment) = self.fragment {
            let fragment_data = fragment.data();

            let start = self.fragment_position;
            let end = start + buffer_slice.len();

            let fragment_slice = &fragment_data[start..end];

            buffer_slice.copy_from_slice(fragment_slice);

            self.fragment_position = end;
        } else {
            unreachable!("attempted to copy data after end of record");
        }
    }
}

impl<T> Read for RecordReader<T>
where
    T: RecordFragments,
{
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let mut position = 0;
        let mut remaining_bytes = buffer.len();

        while remaining_bytes > 0 && self.fragment.is_some() {
            let block_size = min(
                remaining_bytes,
                self.remaining_bytes_in_fragment(),
            );

            let end_offset = position + block_size;
            let buffer_slice = &mut buffer[position..end_offset];

            self.copy_bytes(buffer_slice);

            if self.reached_end_of_fragment() {
                self.advance_fragment();
            }

            position += block_size;
            remaining_bytes -= block_size;
        }

        Ok(position)
    }
}
