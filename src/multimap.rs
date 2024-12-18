use std::borrow::Borrow;
use std::collections::{hash_map, HashMap, HashSet};
use std::collections::hash_set;
use std::hash::Hash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultiMap<K: PartialEq + Eq + Hash, V: PartialEq + Eq + Hash> {
    map: HashMap<K, HashSet<V>>,
}

impl<K: PartialEq + Eq + Hash, V: PartialEq + Eq + Hash> MultiMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(values) = self.map.get_mut(&key) {
            values.insert(value);
        } else {
            self.map.insert(key, HashSet::from([value]));
        }
    }
    
    pub fn remove(&mut self, key: &K, value: &V) {
        if let Some(values) = self.map.get_mut(key) {
            values.remove(value);
            if values.is_empty() {
                self.map.remove(key);
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<hash_set::Iter<'_, V>> {
        self.map.get(key.borrow()).map(|set| set.iter())
    }

    pub fn contains(&self, key: &K, value: &V) -> bool {
        self.map.get(key).is_some_and(|set| set.contains(value))
    }

    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            current_key: None,
            map_iter: self.map.iter(),
            set_iter: None,
        }
    }
    
    pub fn values(&self) -> Values<K, V> {
        Values {
            iter: self.iter()
        }
    }
}

impl<K: Eq + Hash, V: Eq + Hash, const N: usize> From<[(K, V); N]> for MultiMap<K, V> {
    fn from(keys_values: [(K, V); N]) -> Self {
        let mut map = Self::new();

        for (key, value) in keys_values {
            map.insert(key, value);
        }

        map
    }
}

impl<K: Eq + Hash, V: Eq + Hash> FromIterator<(K, V)> for MultiMap<K, V> {
    fn from_iter<T: IntoIterator<Item=(K, V)>>(iter: T) -> Self {
        let mut map = MultiMap::new();
        for (key, value) in iter {
            map.insert(key, value);
        }
        map
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    current_key: Option<&'a K>,
    map_iter: hash_map::Iter<'a, K, HashSet<V>>,
    set_iter: Option<hash_set::Iter<'a, V>>,
}

impl<'a, K: 'a, V: 'a> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.set_iter.is_none() {
            let (key, set) = self.map_iter.next()?;
            self.current_key = Some(key);
            self.set_iter = Some(set.iter());
        }
        
        if let Some(set_iter) = &mut self.set_iter {
            if let Some(next_value) = set_iter.next() {
                Some((self.current_key?, next_value))
            } else {
                self.set_iter = None;
                self.next()
            }
        } else {
            None
        }
    }
}

pub struct Values<'a, K: 'a, V: 'a> {
    iter: Iter<'a, K, V>,
}

impl<'a, K: 'a, V: 'a> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, value)| value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_get() {
        let mut map = MultiMap::new();
        let expected_results = [22, 33];

        map.insert(11, 22);
        map.insert(11, 33);

        let mut values = map.get(&11).unwrap();

        assert!(expected_results.contains(values.next().unwrap()));
        assert!(expected_results.contains(values.next().unwrap()));
        assert_eq!(None, values.next());
    }

    #[test]
    fn test_from_get() {
        let map = MultiMap::from([
            (11, 22),
            (11, 33),
        ]);
        let expected_results = [22, 33];

        let mut values = map.get(&11).unwrap();

        assert!(expected_results.contains(values.next().unwrap()));
        assert!(expected_results.contains(values.next().unwrap()));
        assert_eq!(None, values.next());
    }

    #[test]
    fn test_contains_value() {
        let map = MultiMap::from([
            (11, 22),
            (11, 33),
        ]);

        assert!(map.contains(&11, &22));
        assert!(map.contains(&11, &33));
        assert!(!map.contains(&11, &44));
        assert!(!map.contains(&22, &44));
    }

    #[test]
    fn test_iter() {
        let key_values = [
            (11, 22),
            (11, 33),
            (22, 44),
            (33, 55),
        ];
        
        let map = MultiMap::from(key_values);

        let mut map_iter = map.iter();

        assert!(key_values.contains(&map_iter.next().map(|(k, v)| (*k, *v)).unwrap()));
        assert!(key_values.contains(&map_iter.next().map(|(k, v)| (*k, *v)).unwrap()));
        assert!(key_values.contains(&map_iter.next().map(|(k, v)| (*k, *v)).unwrap()));
        assert!(key_values.contains(&map_iter.next().map(|(k, v)| (*k, *v)).unwrap()));
        assert_eq!(None, map_iter.next());
    }

    #[test]
    fn test_values() {
        let map = MultiMap::from([
            (11, 22),
            (11, 33),
            (11, 44),
            (22, 44),
            (33, 55),
        ]);
        let expected_values = [22, 33, 44, 44, 55];

        let mut values = map.values();

        assert!(expected_values.contains(values.next().unwrap()));
        assert!(expected_values.contains(values.next().unwrap()));
        assert!(expected_values.contains(values.next().unwrap()));
        assert!(expected_values.contains(values.next().unwrap()));
        assert!(expected_values.contains(values.next().unwrap()));
        assert_eq!(None, values.next());
    }
    
    #[test]
    fn test_remove() {
        let mut map = MultiMap::from([
            (11, 22),
            (11, 33),
            (22, 44),
            (22, 66),
            (33, 55),
        ]);

        let expected_map = MultiMap::from([
            (11, 22),
            (33, 55),
        ]);
        
        map.remove(&11, &33);
        map.remove(&22, &44);
        map.remove(&22, &66);
        
        assert_eq!(expected_map, map);
    }
}