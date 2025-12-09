use num::Integer;

pub fn modulo<T>(a: T, b: T) -> T
where
    T: Integer + Copy,
{
    ((a % b) + b) % b
}
