use rustc_hash::FxHashSet;

use crate::util::point::Point;

pub fn part1(input: &str) -> u32 {
    get_regions(input)
        .iter()
        .map(|region| region.area * region.perimeter)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    get_regions(input)
        .iter()
        .map(|region| region.area * region.sides())
        .sum()
}

fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn get_regions(input: &str) -> Vec<Region> {
    let grid = parse(input);
    let mut visited = FxHashSet::default();
    let mut regions = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, plant) in row.iter().enumerate() {
            let point = Point::new(x as i32, y as i32);

            if !visited.contains(&point) {
                let mut region = Region::default();
                find_region(&grid, &mut visited, &mut region, point, *plant);
                regions.push(region);
            }
        }
    }

    regions
}

fn find_region(
    grid: &Vec<&[u8]>,
    visited: &mut FxHashSet<Point>,
    region: &mut Region,
    point: Point,
    plant: u8,
) -> bool {
    if let Some(row) = grid.get(point.y as usize) {
        if let Some(char) = row.get(point.x as usize) {
            if *char == plant {
                if visited.contains(&point) {
                    return true;
                }

                visited.insert(point);
                region.area += 1;

                for neighbor in Point::von_neumann() {
                    if !find_region(grid, visited, region, point + neighbor, plant) {
                        region.perimeter += 1;
                        region.edges.insert(Edge(point, neighbor.into()));
                    }
                }

                return true;
            }
        }
    }

    false
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Edge(Point, u8);

#[derive(Default)]
struct Region {
    area: u32,
    perimeter: u32,
    edges: FxHashSet<Edge>,
}

impl Region {
    fn sides(&self) -> u32 {
        let mut to_remove = FxHashSet::default();
        let mut sorted = Vec::from_iter(self.edges.iter());
        sorted.sort();

        for edge in sorted {
            let sides = match edge.1 {
                b'^' | b'v' => [Point::left(), Point::right()],
                _ => [Point::up(), Point::down()],
            }
            .map(|point| Edge(edge.0 + point, edge.1));

            if sides
                .iter()
                .any(|fence| self.edges.contains(fence) && !to_remove.contains(fence))
            {
                to_remove.insert(*edge);
            }
        }

        self.edges.difference(&to_remove).count() as u32
    }
}
