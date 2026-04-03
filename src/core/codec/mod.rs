pub mod json;

pub trait Codec<T>
{
    type Error;

    fn encode(&self, value: &T) -> Result<Vec<u8>, Self::Error>;
    fn decode(&self, data: &[u8]) -> Result<T, Self::Error>;
}