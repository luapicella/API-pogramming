/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn_my(isbn: &str) -> bool {
    isbn.chars().fold((0, 0, true), |(count, sum , valid), c|{
        match c {
            '0'..='9' => (count+1, (sum+(10-count)*c.to_digit(10).unwrap())%11, valid),
            'X' => (count+1, (sum+10)%11, valid&&(count==9)),
            '-' => (count, sum, valid),
            _ => (0, 0, false)
        }
    }) == (10, 0, true)
}



pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut t = 0;
    let mut w = 10;
    for ch in isbn.chars(){
        match ch {
            '-' => continue,
            'X' if w==1 => t += 10,
            '0'..='9' => t += ch.to_digit(10).unwrap()*w,
            _ => return false
        }
        if w==0 {return false}
        w -= 1
    }
    t % 11 == 0 && w == 0
}