use crate::util::parse;

pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .map(|machine| {
            if let Some(solution) = machine.solve() {
                solution.0 * 3 + solution.1
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .iter_mut()
        .map(|machine| {
            machine.p.0 += 10_000_000_000_000.;
            machine.p.1 += 10_000_000_000_000.;

            if let Some(solution) = machine.solve() {
                solution.0 * 3 + solution.1
            } else {
                0
            }
        })
        .sum()
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|group| {
            let numbers: Vec<f64> = group
                .lines()
                .flat_map(|line| {
                    line.split(',')
                        .map(|section| parse::extract_integer(section) as f64)
                })
                .collect();

            Machine {
                ba: (numbers[0], numbers[1]),
                bb: (numbers[2], numbers[3]),
                p: (numbers[4], numbers[5]),
            }
        })
        .collect()
}

struct Machine {
    /// Button A
    ba: (f64, f64),

    /// Button B
    bb: (f64, f64),

    /// Prize
    p: (f64, f64),
}

impl Machine {
    fn solve(&self) -> Option<(usize, usize)> {
        // Uses Cramer's rule to solve the system of linear equations.
        // https://en.wikipedia.org/wiki/Cramer%27s_rule#Explicit_formulas_for_small_systems

        let x = (self.p.0 * self.bb.1 - self.bb.0 * self.p.1)
            / (self.ba.0 * self.bb.1 - self.bb.0 * self.ba.1);

        let y = (self.ba.0 * self.p.1 - self.p.0 * self.ba.1)
            / (self.ba.0 * self.bb.1 - self.bb.0 * self.ba.1);

        if x.fract() == 0. && y.fract() == 0. {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
}
