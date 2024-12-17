pub fn len(number: u64) -> usize {
    number.checked_ilog10().unwrap_or(0) as usize + 1
}

pub fn split(number: u64) -> (u64, u64) {
    let divisor = 10u64.pow(len(number) as u32 / 2);

    (number / divisor, number % divisor)
}
