use std::collections::HashMap;
use std::hash::Hash;

pub trait GetVec<Key, Value> {
    fn get_vec(&self, vs:Vec<Key>) -> Vec<(Key, &Value)>;
}

impl<Key:Eq+Clone,Value> GetVec<Key, Value> for Vec<(Key,Value)> {
    fn get_vec(&self, vs:Vec<Key>) -> Vec<(Key, &Value)> {
        self.iter()
            .filter(|(k,_)|vs.contains(k))
            .map(|(k,v)|(k.clone(),v))
            .collect()
    }
}

impl<Key:Eq+Hash+Clone, Value> GetVec<Key, Value> for HashMap<Key, Value> {
    fn get_vec(&self, vs:Vec<Key>) -> Vec<(Key, &Value)> {
        vs.into_iter()
          .map(|key|(key.clone(), self.get(&key).unwrap()))
          .collect()
    }
}