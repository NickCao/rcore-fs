pub trait Store {
    fn put<K: AsRef<[u8]>, V: AsRef<[u8]>>(&mut self, key: K, value: V) -> std::io::Result<()>;
    fn get<K: AsRef<[u8]>>(&mut self, key: K) -> std::io::Result<Option<Vec<u8>>>;
    fn delete<K: AsRef<[u8]>>(&mut self, key: K) -> std::io::Result<()>;
}
