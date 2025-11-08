use rustc_hash::FxHashMap;

pub fn frequencies<T: Eq + std::hash::Hash>(array: &[T]) -> FxHashMap<&T, usize> {
    array.iter().fold(FxHashMap::default(), |mut map, element| {
        *map.entry(element).or_default() += 1;
        map
    })
}
