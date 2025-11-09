use std::cmp::Ordering;

use crate::util::num;

pub fn part1(input: &str) -> usize {
    parse(input).first().unwrap().quality
}

pub fn part2(input: &str) -> usize {
    let swords = parse(input);
    let qualities: Vec<usize> = swords.iter().map(|sword| sword.quality).collect();
    qualities.iter().max().unwrap() - qualities.iter().min().unwrap()
}

pub fn part3(input: &str) -> usize {
    let mut swords = parse(input);
    swords.sort_by(|a, b| {
        let quality_ordering = b.quality.cmp(&a.quality);
        if quality_ordering != Ordering::Equal {
            return quality_ordering;
        }

        for (i, segment) in a.segments.iter().enumerate() {
            let mut a_level = [segment.left, segment.spine].to_vec();
            if segment.right != 0 {
                a_level.push(segment.right);
            }

            let mut b_level = [b.segments[i].left, b.segments[i].spine].to_vec();
            if b.segments[i].right != 0 {
                b_level.push(b.segments[i].right);
            }

            let level_ordering = num::concat(&b_level).cmp(&num::concat(&a_level));
            if level_ordering != Ordering::Equal {
                return level_ordering;
            }
        }

        b.id.cmp(&a.id)
    });

    swords
        .iter()
        .enumerate()
        .map(|(i, sword)| sword.id * (i + 1))
        .sum()
}

fn parse(input: &str) -> Vec<Sword> {
    input
        .lines()
        .map(|line| {
            let (id, nums) = line.split_once(":").unwrap();
            let numbers: Vec<u32> = nums.split(",").map(|n| n.parse().unwrap()).collect();
            let segments: Vec<Segment> = get_segments(&numbers);
            let quality = get_quality(&segments);

            Sword {
                id: id.parse().unwrap(),
                segments,
                quality,
            }
        })
        .collect()
}

fn get_segments(numbers: &[u32]) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();

    numbers.iter().for_each(|number| {
        let mut found_segment = false;

        for segment in segments.iter_mut() {
            if segment.left == 0 && *number < segment.spine {
                segment.left = *number;
                found_segment = true;
                break;
            } else if segment.right == 0 && *number > segment.spine {
                segment.right = *number;
                found_segment = true;
                break;
            }
        }

        if !found_segment {
            let new_segment = Segment {
                left: 0,
                spine: *number,
                right: 0,
            };
            segments.push(new_segment);
        }
    });

    segments
}

fn get_quality(segments: &[Segment]) -> usize {
    let spines: Vec<u32> = segments.iter().map(|segment| segment.spine).collect();
    num::concat(&spines)
}

struct Sword {
    id: usize,
    segments: Vec<Segment>,
    quality: usize,
}

#[derive(Clone, Copy)]
struct Segment {
    left: u32,
    spine: u32,
    right: u32,
}
