pub struct JsonCodec
{
    pub pretty:bool,
}

impl <T> super::Codec<T> for JsonCodec 
where 
    T: serde::Serialize + serde::de::DeserializeOwned
{
    type Error = serde_json::Error;

    fn encode(&self, value:&T) -> Result<Vec<u8>, Self::Error> 
    {
        if self.pretty 
        {
            return serde_json::to_vec_pretty(value)
        }
        else
        {
            return serde_json::to_vec(value)
        }
    }

    fn decode(&self, data:&[u8]) -> Result<T, Self::Error> 
    {
        serde_json::from_slice(data)
    }
}