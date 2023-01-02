pub mod encoding;

use std::io::Write;
use encoding::encoder_trait::EncoderDecoder;

pub struct Vault {
    data: std::collections::HashMap<String, String>,
    file_wrapper: FileWrapper,
}

#[derive(Default)]
struct FileWrapper {
    path: String,
}

impl FileWrapper {
    pub fn new(path: String) -> Self { FileWrapper { path } }
    pub fn read(&self) -> Result<String, String> {
        match std::fs::read_to_string(&self.path) {
            Ok(contents) => Ok(contents),
            Err(msg) => Err(format!("Error while reading file: {}", msg)),
        }
    }

    pub fn write(&self, data: String) -> Result<(), String> {
        let mut file = std::fs::OpenOptions::new()
            .truncate(true)
            .write(true)
            .open(&self.path)
            .unwrap();
        write!(file, "{}", data).expect("Failed to write file");
        Ok(())
    }
}

impl Vault {
    pub fn new<E: EncoderDecoder<std::collections::HashMap<String, String>, String, String, String>>(path: String) -> Self {
        let file_wrapper = FileWrapper::new(path);
        match file_wrapper.read() {
            Ok(contents) => {
                let data = E::decode(contents).unwrap();
                Vault { data, file_wrapper }
            }
            Err(msg) => panic!("{}", msg),
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.data.get(&key) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn commit<E: EncoderDecoder<std::collections::HashMap<String, String>, String, String, String>>(&self) {
        let new_data = E::encode(&self.data).unwrap();
        self.file_wrapper.write(new_data).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_vault_can_index_its_set() {
        let mut set = std::collections::HashMap::new();
        set.insert(String::from("Hello"), String::from("World"));
        let vault = Vault { data: set, file_wrapper: FileWrapper::default() };
        assert!(matches!(vault.get(format!("Hello")), Some(s) if s == "World"));
        assert!(matches!(vault.get(format!("None")), None));
    }
}
