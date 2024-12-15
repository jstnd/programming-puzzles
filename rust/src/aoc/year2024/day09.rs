use rustc_hash::FxHashSet;

enum Block {
    File(u64, u32),
    Free,
}

pub fn part1(input: &str) -> u64 {
    let disk = parse(input);
    let mut checksum = 0;
    let mut left = 0;
    let mut right = disk.len() - 1;

    while left <= right {
        checksum += match disk[left] {
            Block::File(id, _) => left as u64 * id,
            Block::Free => {
                while let Block::Free = disk[right] {
                    right -= 1;
                }

                if let Block::File(id, _) = disk[right] {
                    right -= 1;
                    left as u64 * id
                } else {
                    0
                }
            }
        };

        left += 1;
    }

    checksum
}

pub fn part2(input: &str) -> u64 {
    let mut disk = parse(input);
    let mut passed = FxHashSet::with_capacity_and_hasher(10_000, Default::default());

    for i in (0..disk.len()).rev() {
        if let Block::File(id, size) = disk[i] {
            if !passed.contains(&id) {
                passed.insert(id);
                let mut start = 0;
                let mut free = 0;

                while start < i {
                    match disk[start + free] {
                        Block::File(_, size) => {
                            start += free + size as usize;
                            free = 0;
                        }
                        Block::Free => {
                            free += 1;

                            if free as u32 == size {
                                let file_start = (i - free) + 1;

                                for j in 0..free {
                                    disk.swap(start + j, file_start + j);
                                }

                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    disk.iter()
        .enumerate()
        .map(|(position, block)| match block {
            Block::File(id, _) => position as u64 * id,
            Block::Free => 0,
        })
        .sum()
}

fn parse(input: &str) -> Vec<Block> {
    input
        .trim()
        .char_indices()
        .flat_map(|(i, c)| {
            let size = c.to_digit(10).unwrap();
            let id = i as u64 / 2;

            (0..size).map(move |_| {
                if i % 2 == 0 {
                    Block::File(id, size)
                } else {
                    Block::Free
                }
            })
        })
        .collect()
}
