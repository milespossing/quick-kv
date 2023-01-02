pub trait EncoderDecoder<T, O, EEncode, EDecode> {
    fn encode(data: &T) -> Result<O, EEncode>;
    fn decode(data: O) -> Result<T, EDecode>;
}
