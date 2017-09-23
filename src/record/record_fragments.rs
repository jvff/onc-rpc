use bytes::BytesMut;

use super::record_fragment::RecordFragment;

pub trait RecordFragments: RecordFragment {
    fn next(self) -> Self;
    fn is_full_record(&self) -> bool;
    fn full_record_length(&self) -> Option<usize>;
}

impl<'a> RecordFragments for &'a [u8] {
    fn next(self) -> &'a [u8] {
        let end = self.length() + 4;

        &self.as_ref()[end..]
    }

    fn is_full_record(&self) -> bool {
        let min_length = 4 + self.length();
        let reborrow = &*self;

        if self.as_ref().len() < min_length {
            false
        } else if self.is_last() {
            true
        } else {
            reborrow.next().is_full_record()
        }
    }

    fn full_record_length(&self) -> Option<usize> {
        let fragment_length = 4 + self.length();
        let reborrow = &*self;

        if self.as_ref().len() < fragment_length {
            None
        } else if self.is_last() {
            Some(fragment_length)
        } else if let Some(next_length) = reborrow.next().full_record_length() {
            Some(fragment_length + next_length)
        } else {
            None
        }
    }
}

impl RecordFragments for BytesMut {
    fn next(mut self) -> BytesMut {
        let length = self.length();

        self.split_off(4 + length)
    }

    fn is_full_record(&self) -> bool {
        let mut position = 0;

        while position < self.len() {
            let fragment = &self[position..];
            let fragment_length = fragment.length();

            position += fragment_length;

            if position >= self.len() {
                return false;
            } else if fragment.is_last() {
                return true;
            }
        }

        false
    }

    fn full_record_length(&self) -> Option<usize> {
        let mut position = 0;

        while position < self.len() {
            let fragment =&self[position..];
            let fragment_length = fragment.length();

            position += fragment_length;

            if position > self.len() {
                return None;
            } else if fragment.is_last() {
                return Some(position);
            }
        }

        None
    }
}
