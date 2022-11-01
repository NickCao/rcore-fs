use std::{collections::HashMap, sync::Mutex};

use crate::transport::Transport;

pub trait Store {
    fn put(&mut self, key: &[u8], value: &[u8]) -> std::io::Result<()>;
    fn get(&mut self, key: &[u8]) -> std::io::Result<Option<Vec<u8>>>;
    fn delete(&mut self, key: &[u8]) -> std::io::Result<()>;
}

impl Store for HashMap<Vec<u8>, Vec<u8>> {
    fn put(&mut self, key: &[u8], value: &[u8]) -> std::io::Result<()> {
        self.insert(key.to_vec(), value.to_vec());
        Ok(())
    }
    fn get(&mut self, key: &[u8]) -> std::io::Result<Option<Vec<u8>>> {
        Ok(HashMap::get(self, key).map(|value| value.clone()))
    }
    fn delete(&mut self, key: &[u8]) -> std::io::Result<()> {
        self.remove(key);
        Ok(())
    }
}

struct DistributedStore {
    tranport: Mutex<Box<dyn Transport>>,
    store: Mutex<Box<dyn Store>>,
}

impl Store for DistributedStore {
    fn put(&mut self, key: &[u8], value: &[u8]) -> std::io::Result<()> {
        unimplemented!()
    }
    fn get(&mut self, key: &[u8]) -> std::io::Result<Option<Vec<u8>>> {
        unimplemented!()
    }
    fn delete(&mut self, key: &[u8]) -> std::io::Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::Store;
    use std::collections::HashMap;
    #[test]
    fn simple() {
        let mut store: Box<dyn Store> = Box::new(HashMap::new());
        store.put(b"foo", b"bar").unwrap();
        assert_eq!(store.get(b"foo").unwrap().unwrap(), b"bar");
        store.delete(b"foo").unwrap();
        assert_eq!(store.get(b"foo").unwrap(), None);
    }
}
