pub fn part1(input: &str) -> usize {
    input.lines().map(|bank| get_largest_joltage(bank, 2)).sum()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|bank| get_largest_joltage(bank, 12))
        .sum()
}

fn get_largest_joltage(bank: &str, batteries: usize) -> usize {
    let chars: Vec<char> = bank.chars().collect();
    let mut digits = Vec::new();
    let mut index = 0;

    // Loop through the number of batteries we need to turn on.
    for remaining_batteries in (0..batteries).rev() {
        // The `index` variable here will keep track of the start of the window where we will look for the next battery to turn on.
        // The `remaining_batteries` variable is used to remove digits from the end of the window, since we will need at least
        // that many digits left in the future searched windows to be able to turn on the required amount of batteries.
        // Take a bank of batteries of 818181911112111, and the number of required batteries to turn on of 2, as an example:
        //
        // Here, `index` will start at 0 (the start of the bank), and `remaining_batteries` will be 1 in the first loop iteration
        // (since there will be 1 remaining battery to find after this loop iteration).
        //
        // Therefore, the first window we will search through is:
        //         818181911112111 => 9 is the largest digit in this window
        //         --------------
        //
        // After that first window, `index` will be 7 (the index after the found 9 above), and `remaining_batteries` will be 0.
        // The window for the second battery will be:
        //         818181911112111 => 2 is the largest digit in this window
        //                --------
        //
        // We have found the digits of the two required batteries, and the largest joltage for this example bank is 92.
        // This problem has the same logic for larger numbers of required batteries, the searched windows will just have
        // different lengths in accordance with that number of batteries that we need to find.

        // Find the index of the battery with the largest digit in the next slice of batteries.
        index += get_largest_index(&chars[index..chars.len() - remaining_batteries]);

        // Add the digit of the found battery to our list of digits.
        digits.push(chars[index]);

        // Move the index forward for the start of the next window of chars.
        index += 1;
    }

    // Combine the digits into a string then parse into an integer to be added to the sum.
    let str: String = digits.iter().collect();
    str.parse().unwrap()
}

fn get_largest_index(chars: &[char]) -> usize {
    // Finds the first index of the max value in the passed slice.
    // => .iter().enumerate() -> enumerates over the slice
    // => .rev() -> max_by_key(...) returns the last max element, so we need to iterate
    //              in reverse order as the last max element would then contain the smallest index
    // => .max_by_key(...) -> finds the last element with the max value in the slice
    // => .map(...).unwrap() -> extracts the index from the found max value
    chars
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, &value)| value)
        .map(|(index, _)| index)
        .unwrap()
}
