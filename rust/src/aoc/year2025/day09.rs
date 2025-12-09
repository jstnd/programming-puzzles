use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .tuple_combinations()
        .map(|(point1, point2)| area(point1, point2))
        .max()
        .unwrap()
}

pub fn part2(input: &str) -> usize {
    let points = parse(input);

    // Get the edges of the outside polygon by iterating through windows of input points.
    let edges: Vec<(Point, Point)> = points
        .windows(2)
        .map(|window| (window[0], window[1]))
        .collect();

    // Find the two points that form the largest valid rectangle.
    // => .sorted_unstable_by_key(...).rev() -> sorts the point combinations by descending area
    // => .find(...) -> finds the first point combination that is valid
    let (point1, point2) = points
        .iter()
        .tuple_combinations()
        .sorted_unstable_by_key(|(point1, point2)| area(point1, point2))
        .rev()
        .find(|(point1, point2)| {
            // Calculate the end points of the current point combination.
            let min_x = point1.x.min(point2.x);
            let max_x = point1.x.max(point2.x);
            let min_y = point1.y.min(point2.y);
            let max_y = point1.y.max(point2.y);

            // Check that all edges are not inside of the rectangle formed by the current two points.
            edges.iter().all(|(start, end)| {
                // The rectangle should be ON or RIGHT of this edge, OR
                min_x >= start.x.max(end.x)
                    // The rectangle should be ON or LEFT of this edge, OR
                    || max_x <= start.x.min(end.x)
                    // The rectangle should be ON or BELOW this edge, OR
                    || min_y >= start.y.max(end.y)
                    // The rectangle should be ON or ABOVE this edge.
                    || max_y <= start.y.min(end.y)
            })
        })
        .unwrap();

    area(point1, point2)
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

fn area(point1: &Point, point2: &Point) -> usize {
    (point1.x.abs_diff(point2.x) + 1) * (point1.y.abs_diff(point2.y) + 1)
}
