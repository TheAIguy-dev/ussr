pub enum WriteError {}

pub trait Writable
where
    Self: Sized,
{
    fn write_to(&self, buf: &mut [u8]) -> Result<usize, WriteError>;
}
