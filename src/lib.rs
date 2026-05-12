use hashbrown::HashMap;
use delegate::delegate;
use ambassador::{Delegate, delegatable_trait_remote};
use core::hash::Hash;
use core::slice::Iter;   
use core::mem;
use core::ops::Index; 


pub trait Insertable: Hash + Eq {}
impl<T:Hash+Eq> Insertable for T {}

#[delegatable_trait_remote]
trait Index<Idx> {
    type Output;
    fn index(&self, index: Idx) -> &Self::Output;
}

#[derive(Delegate)]
#[delegate(Index<usize>, target = "vector")]
pub struct FastMap <K:Insertable, V:Sized >{
    map: HashMap<K,usize>,
    vector: Vec<V>,
}

impl<K:Insertable, V:Sized> FastMap<K,V> {
    delegate! {
        to self.map {
            pub fn len(&self) -> usize;
        }

        to self.vector {
            pub fn iter(&self) -> Iter<'_, V>;
        }
    }
    
    //pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    pub fn remove(&mut self, value:&K) -> Option<V>{
        if let Some(index) = self.map.remove(value) {
            return Some(self.vector.remove(index))
        } None
    }

    pub fn get(&self, k:&K) -> Option<&V> {
        if let Some(index) = self.map.get(k) {
            return Some(&self.vector[*index])
        } None
    }

    //pub fn insert(&mut self, k: K, v: V) -> Option<V>
    pub fn insert(&mut self, k:K, v:V) -> Option<V> {
        return if let Some(old_index) = self.map.get(&k) {
            Some(mem::replace(&mut self.vector[*old_index],v ))
        } else {
            self.map.insert(k,self.len());
            self.vector.push(v);  
            None
        };
    }

    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            vector: Vec::new()
        }
    }

}