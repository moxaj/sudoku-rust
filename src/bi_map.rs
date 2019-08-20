use std::collections::{HashMap, HashSet};
use std::collections::btree_map::BTreeMap;
use std::collections::btree_set::BTreeSet;
use std::fmt;
use std::hash::Hash;

use crate::DEBUG;

#[derive(Debug)]
pub struct BiMap<'a, T, U>
    where T: Eq + Hash + Copy + Ord + fmt::Debug,
          U: Eq + Hash + Copy + Ord + fmt::Debug {
    keys: &'a Vec<T>,
    values: &'a Vec<U>,
    mapping: HashMap<T, BTreeSet<U>>,
}

impl<'a, T, U> BiMap<'a, T, U>
    where T: Eq + Hash + Copy + Ord + fmt::Debug,
          U: Eq + Hash + Copy + Ord + fmt::Debug {
    pub fn new(keys: &'a Vec<T>, values: &'a Vec<U>, mapping: HashMap<T, BTreeSet<U>>) -> BiMap<'a, T, U> {
        BiMap { keys, values, mapping }
    }

    fn grouped(&self) -> Option<HashMap<usize, Vec<(HashSet<T>, BTreeSet<U>)>>> {
        if DEBUG {
            println!("Mapping: {:?}", &self.mapping);
        }

        let mut temp = BTreeMap::new();
        for (key, value) in self.mapping.clone() {
            temp.entry(value).or_insert(HashSet::new()).insert(key);
        }

        let mut grouped = HashMap::new();
        for (key, value) in temp {
            let size = key.len();
            if size >= value.len() {
                grouped.entry(size).or_insert(Vec::new()).push((value, key));
            } else {
                return None;
            }
        }

        Some(grouped)
    }

    pub fn singles(&self) -> Option<Vec<(T, U)>> {
        self.grouped().map(|mut grouped| {
            if DEBUG {
                println!("Grouped: {:?}", &grouped);
            }

            if let Some(singles) = grouped.remove(&1) {
                singles
                    .into_iter()
                    .map(|(keys, values)|
                        (keys.into_iter().next().unwrap(),
                         values.into_iter().next().unwrap()))
                    .collect()
            } else {
                Vec::new()
            }
        })
    }

    pub fn doubles(&self) -> Option<Vec<([T; 2], [U; 2])>> {
        self.grouped().map(|mut grouped| {
            if let Some(doubles) = grouped.remove(&2) {
                doubles
                    .into_iter()
                    .map(|(keys, values)| {
                        let mut keys_iter = keys.into_iter();
                        let mut values_iter = values.into_iter();
                        ([keys_iter.next().unwrap(), keys_iter.next().unwrap()],
                         [values_iter.next().unwrap(), values_iter.next().unwrap()])
                    })
                    .collect()
            } else {
                Vec::new()
            }
        })
    }

    pub fn reversed(&self) -> BiMap<'a, U, T> {
        let mut reversed_mapping = HashMap::new();
        for (key, values) in &self.mapping {
            for value in values {
                reversed_mapping.entry(*value).or_insert(BTreeSet::new()).insert(*key);
            }
        }

        for value in self.values {
            reversed_mapping.entry(*value).or_insert(BTreeSet::new());
        }

        BiMap::new(self.values, self.keys, reversed_mapping)
    }
}