use std::collections::HashMap;

//REGISTER CODE - PERSONAL UTIL STRUCT, USED FOR KEEPING IDS ATTACHED TO VEC INDICES

pub trait Identify {
    fn get_id(&self) -> u32;
    fn set_id(&mut self, i: u32);
}

#[derive(Clone)]
pub struct Register<T:Identify> {
    pub data: Vec<T>,
    pub index: HashMap<u32, usize>,
    pub max_id: u32,
    pub free_ids: Vec<u32>
}

impl<T:Identify> Register<T> {
    pub fn new() -> Self {
        let data = Vec::new();
        let index = HashMap::new();
        let max_id = 0;
        let free_ids = Vec::new();
        Self {
            data,
            index,
            max_id,
            free_ids,
        }
    }

    fn get_id(&mut self) -> u32 {
        if self.free_ids.len() > 0 { return self.free_ids.pop().unwrap(); }
        else { self.max_id += 1; return self.max_id - 1; }
    }

    fn refresh(&mut self) {
        self.index = HashMap::new();
        let mut i = 0;
        for item in &self.data {
            self.index.insert(item.get_id(), i);
            i += 1;
        }
    }

    pub fn insert(&mut self, mut item: T) -> u32 {
        let index = self.data.len();
        let id = self.get_id();
        item.set_id(id);
        self.data.push(item);
        self.index.insert(id, index);
        return id;
    }

    pub fn remove(&mut self, id: &u32) -> Option<T> {
        if id.clone() >= self.data.len() as u32 { return None; }
        self.free_ids.push(id.clone());
        let v = self.index.remove_entry(id).unwrap().1;
        let i = self.data.swap_remove(v);
        self.refresh();
        return Some(i);
    }

    pub fn get(&self, id: &u32) -> Option<&T> {
        match self.index.get(id) {
            Some(index) => self.data.get(*index),
            None => None,
        }
    }

    pub fn get_mut(&mut self, id: &u32) -> Option<&mut T> {
        match self.index.get(id) {
            Some(index) => self.data.get_mut(*index),
            None => None,
        }
    }

    pub fn get_vec(&self, ids: &Vec<u32>) -> Vec<Option<&T>> {
        let mut v = Vec::new();
        for id in ids {
            match self.index.get(id) {
                Some(index) => v.push(self.data.get(*index)),
                None => v.push(None),
            }
        }
        v
    }
    
    pub fn get_indices(&self, ids: &Vec<u32>) -> Vec<Option<&usize>> {
        return ids.iter().map(|i| self.index.get(i)).collect::<Vec<_>>();
    }
}