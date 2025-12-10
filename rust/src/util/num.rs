pub fn concat<T>(array: &[T]) -> usize
where
    T: Copy + num::Integer + num::Unsigned + num::ToPrimitive,
{
    array.iter().fold(0, |mut result, element| {
        result = (result * 10_usize.pow(len(*element) as u32)) + element.to_usize().unwrap();
        result
    })
}

pub fn digits_rev<T>(number: T) -> impl Iterator<Item = u8>
where
    T: num::Integer + num::Unsigned + num::ToPrimitive,
{
    let mut number = number.to_usize().unwrap_or(0);

    std::iter::from_fn(move || {
        if number == 0 {
            return None;
        }

        let digit = number % 10;
        number /= 10;

        Some(digit as u8)
    })
}

pub fn len<T>(number: T) -> usize
where
    T: num::Integer + num::Unsigned + num::ToPrimitive,
{
    number.to_usize().unwrap_or(0).checked_ilog10().unwrap_or(0) as usize + 1
}

pub fn split<T>(number: T) -> (usize, usize)
where
    T: num::Integer + num::Unsigned + num::ToPrimitive,
{
    let number = number.to_usize().unwrap_or(0);
    let divisor = 10_usize.pow(len(number) as u32 / 2);
    (number / divisor, number % divisor)
}
