use std::ops::Deref;

pub struct Record<T>
where
    T: AsRef<[u8]>,
{
    data: T,
}

impl<T> Record<T>
where
    T: AsRef<[u8]>,
{
    pub fn transaction_id(&self) -> u32 {
        let bytes = self.data.as_ref();

        ((bytes[0] as u32) << 24)
            | ((bytes[1] as u32) << 16)
            | ((bytes[2] as u32) << 8)
            | (bytes[3] as u32)
    }
}

impl<T> Record<T>
where
    T: AsMut<[u8]> + AsRef<[u8]>,
{
    pub fn set_transaction_id(&mut self, transaction_id: u32) {
        let bytes = self.data.as_mut();

        bytes[0] = ((transaction_id >> 24) & 0xff) as u8;
        bytes[1] = ((transaction_id >> 16) & 0xff) as u8;
        bytes[2] = ((transaction_id >> 8) & 0xff) as u8;
        bytes[3] = (transaction_id & 0xff) as u8;
    }
}

impl<T> From<T> for Record<T>
where
    T: AsRef<[u8]>,
{
    fn from(data: T) -> Self {
        Record { data }
    }
}

impl<T> AsRef<[u8]> for Record<T>
where
    T: AsRef<[u8]>,
{
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl<T> Deref for Record<T>
where
    T: AsRef<[u8]>,
{
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.data.as_ref()
    }
}
