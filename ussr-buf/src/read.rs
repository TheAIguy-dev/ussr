pub enum ReadError {}

pub trait Readable
where
    Self: Sized,
{
    fn read_from(&mut self, buf: &mut [u8]) -> Result<Self, ReadError>;
}
