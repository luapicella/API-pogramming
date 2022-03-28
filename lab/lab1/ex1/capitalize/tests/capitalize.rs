use capitalize::capitalize::capitalize;

#[test]
fn test_valid_single_word() {
    assert!(capitalize("ciao") == "Ciao");
}

#[test]
fn test_valid_two_word() {
    //assert!(capitalize("ciao amico") == "Ciao Amico");
    assert_eq!(capitalize("ciao amico"), "Ciao Amico")
}

#[test]
fn test_valid_acc() {
    //assert!(capitalize("ciao amico") == "Ciao Amico");
    assert_eq!(capitalize("èila amico"), "Èila Amico")
}

#[test]
fn test_valid_blank() {
    //assert!(capitalize("ciao amico") == "Ciao Amico");
    assert_eq!(capitalize(""), "")
}

#[test]
fn test_valid_mult_space() {
    //assert!(capitalize("ciao amico") == "Ciao Amico");
    assert_eq!(capitalize("cila         amico"), "Cila Amico")
}

#[test]
fn test_valid_not_english() {
    //assert!(capitalize("ciao amico") == "Ciao Amico");
    assert_eq!(capitalize("я тебя люблю"), "Я Тебя Люблю")
}