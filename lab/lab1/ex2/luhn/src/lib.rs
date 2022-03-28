/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut _sum  = 0;
    let mut _len = 0;

    for (i, c) in code.chars().rev().filter(|&x| x != ' ').enumerate() {
        _len += 1;
        match (i % 2, c.to_digit(10)){
            (1, Some(x)) if x > 4 => _sum += x*2 -9,
            (1, Some(x)) => _sum += x*2,
            (0, Some(x)) => _sum += x,
            (_, _) => return false 
        }
    }

    (_len > 1) && (_sum % 10 == 0) 
}
