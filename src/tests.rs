use FlagFormat;


#[test]
fn parse_multi_test_one_letter(){
    let expected = ("p".to_string(),None);
    assert_eq!(FlagFormat::split_argument_specifier("p".to_string()), expected);
}

#[test]
fn multi_letter_test() {
    let expected = ("p".to_string(),Some("*".to_string()));
    assert_eq!(FlagFormat::split_argument_specifier("p*".to_string()),expected)
}

#[test]
fn integration_test_bool() {
    let args = FlagFormat::parse("p*, l#,r");
    let arg = args.getBool(&"r".to_string());
    assert!(!arg.unwrap())
}


#[test]
fn integration_test_int() {
    let args = FlagFormat::parse("p*,l#,r");
    assert_eq!(args.getInt(&String::from("l")).unwrap(), &3)
}
