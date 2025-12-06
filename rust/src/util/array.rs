use rustc_hash::FxHashMap;

pub fn frequencies<T: Eq + std::hash::Hash>(array: &[T]) -> FxHashMap<&T, usize> {
    array.iter().fold(FxHashMap::default(), |mut map, element| {
        *map.entry(element).or_default() += 1;
        map
    })
}

pub fn transpose<T: Copy + Default>(array: &[Vec<T>]) -> Vec<Vec<T>> {
    let rows = array.len();
    let columns = array[0].len();
    let mut transposed = vec![vec![T::default(); rows]; columns];

    for row in 0..rows {
        for column in 0..columns {
            transposed[column][row] = array[row][column];
        }
    }

    transposed
}
