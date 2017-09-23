pub trait RecordFragment {
    fn is_last(&self) -> bool;
    fn length(&self) -> usize;
    fn data(&self) -> &[u8];
}

impl<T> RecordFragment for T
where
    T: AsRef<[u8]>,
{
    fn is_last(&self) -> bool {
        (0x80 & self.as_ref()[0]) != 0
    }

    fn length(&self) -> usize {
        let bytes = self.as_ref();

        let length = ((bytes[0] as u32 & 0x7f) << 24)
            | ((bytes[1] as u32) << 16)
            | ((bytes[2] as u32) << 8)
            | bytes[3] as u32;

        length as usize
    }

    fn data(&self) -> &[u8] {
        let end = self.length() + 4;

        &self.as_ref()[4..end]
    }
}
