use std::collections::HashMap;

pub fn frequencies<T: Eq + std::hash::Hash>(array: &[T]) -> HashMap<&T, usize> {
    array.iter().fold(HashMap::new(), |mut map, element| {
        *map.entry(element).or_default() += 1;
        map
    })
}
