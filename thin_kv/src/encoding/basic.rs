use std::collections::HashMap;
use super::encoder_trait::EncoderDecoder;

pub struct BasicEncoder {}

impl EncoderDecoder<HashMap<String, String>, String, String, String> for BasicEncoder {
    fn encode(data: &HashMap<String, String>) -> Result<String, String> {
        Ok(data.iter().map(|(key, value)| { format!("{}:{}", key, value) }).collect::<Vec<String>>().join("\n"))
    }

    fn decode(data: String) -> Result<HashMap<String, String>, String> {
        let mut output = std::collections::HashMap::<String, String>::new();
        let lines = data.split("\n").filter(|s|!s.is_empty()).collect::<Vec<&str>>();
        for line in lines {
            let s: Vec<&str> = line.split(":").collect();
            output.insert(s[0].to_string(), s[1].to_string());
        }
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_decode_a_string() {
        let data: String = String::from("Hello:World\nAnother:Key");
        let result = BasicEncoder::decode(data).expect("Failed to decode");
        assert!(matches!(result.get("Hello"), Some(s) if s == "World"));
        assert!(matches!(result.get("Another"), Some(s) if s == "Key"));
        assert!(matches!(result.get("None"), None));
    }

    #[test]
    fn can_encode_a_string() {
        let mut data = HashMap::<String, String>::new();
        let expected = "Hello:World\nAnother:Key";
        data.insert(String::from("Hello"), String::from("World"));
        data.insert(String::from("Another"), String::from("Key"));
        let result = BasicEncoder::encode(data).expect("Failed to encode");
        assert_eq!(result, expected);
    }
}
