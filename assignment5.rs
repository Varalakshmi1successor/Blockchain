use std::collections::HashMap;

// Define a trait called SortByKey that defines a method for sorting the elements in the map by their key values.
trait SortByKey<K: Ord, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)>;
}

// Implement the SortByKey trait for the HashMap struct.
impl<K: Ord, V> SortByKey<K, V> for HashMap<K, V> {
    fn sort_by_key(&self) -> Vec<(&K, &V)> {
        let mut entries: Vec<_> = self.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries
    }
}

fn main() {
    // Create a new instance of the HashMap struct.
    let mut map = HashMap::new();

    // Add several key-value pairs to the map.
    map.insert("c", 3);
    map.insert("a", 1);
    map.insert("b", 2);

    // Sort the elements in the map by their keys.
    let sorted_entries = map.sort_by_key();

    // Print the sorted entries.
    for (key, value) in sorted_entries {
        println!("{}: {}", key, value);
    }
}


