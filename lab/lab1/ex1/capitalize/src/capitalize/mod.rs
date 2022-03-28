fn _capitalize(s: &str) -> String {
    let mut c = s.chars();
    
    match c.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().to_string() + c.as_str()
        },
    }
}

pub fn capitalize(s: &str) -> String {
    let mut ans  = String::new();
    let v: Vec<&str> = s.trim()
            .split(" ")
            .filter(|x|{*x != ""})
            .collect();
    
    for item in v.iter() {
        ans.push_str(_capitalize(item).as_str());
        ans.push_str(" ");
    }
    ans.trim().to_string()
}
