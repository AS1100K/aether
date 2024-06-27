use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;

pub(crate) fn approx_time_to_join(
    position: u32,
    login_rate: f32
) -> Duration {
    let time = position as f32 / login_rate;
    Duration::from_secs_f32(time)
}

pub trait HashMapSub<K, V> {
    fn sub(&self, other: &HashMap<K, V>) -> HashMap<K, V>;
}

impl<K, V> HashMapSub<K, V> for HashMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// ## Examples
    /// ```rust
    /// use std::collections::HashMap;
    ///
    /// fn main() {
    ///        let mut hash_map_1 = HashMap::new();
    ///        hash_map_1.insert("a", 1);
    ///        hash_map_1.insert("b", 2);
    ///        hash_map_1.insert("c", 3);
    ///
    ///        let mut hash_map_2 = HashMap::new();
    ///        hash_map_2.insert("b", 2);
    ///        hash_map_2.insert("c", 3);
    ///        hash_map_2.insert("d", 4);
    ///
    ///        // Subtract hash_map_2 from hash_map_1
    ///        let result = hash_map_1.sub(&hash_map_2);
    ///
    ///        let mut expected_result = HashMap::new();
    ///        expected_result.insert("a", 1);
    ///
    ///        assert_eq!(result, expected_result);
    /// }
    /// ```
    fn sub(&self, other: &HashMap<K, V>) -> HashMap<K, V> {
        let mut result = HashMap::new();

        // Iterate through self HashMap
        for (key, value) in self.iter() {
            // If the key is not present in the other HashMap, insert it into the result
            if !other.contains_key(key) {
                result.insert(key.clone(), value.clone());
            }
        }

        result
    }
}

