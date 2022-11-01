use std::collections::HashMap;

pub trait Store {
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V) -> std::io::Result<()>;
    fn get<K: AsRef<[u8]>>(&mut self, key: K) -> std::io::Result<Option<Vec<u8>>>;
    fn delete<K: AsRef<[u8]>>(&mut self, key: K) -> std::io::Result<()>;
}

impl Store for HashMap<Vec<u8>, Vec<u8>> {
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V) -> std::io::Result<()> {
        self.insert(key.as_ref().to_vec(), value.as_ref().to_vec());
        Ok(())
    }
    fn get<K: AsRef<[u8]>>(&mut self, key: K) -> std::io::Result<Option<Vec<u8>>> {
        Ok(HashMap::get(self, key.as_ref()).map(|value| value.clone()))
    }
    fn delete<K: AsRef<[u8]>>(&mut self, key: K) -> std::io::Result<()> {
        self.remove(key.as_ref());
        Ok(())
    }
}
