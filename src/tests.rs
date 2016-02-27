use FlagFormat;


#[test]
fn parse_multi_test_one_letter(){
    let expected = ("p", None);
    assert_eq!(FlagFormat::split_argument_specifier("p"), expected);
}

#[test]
fn multi_letter_test() {
    let expected = ("p", Some("*"));
    assert_eq!(FlagFormat::split_argument_specifier("p*"),expected)
}

#[test]
fn integration_test_bool() {
    let args = FlagFormat::parse("p*, l#,r");
    let arg = args.is_bool_arg(&"r");
    assert!(arg)
}

#[test]
fn integration_test_int() {
    let args = FlagFormat::parse("p*,l#,r");
    assert!(args.is_int_arg(&"l"))
}

#[test]
fn integration_test_string() {
    let args = FlagFormat::parse("p*,l#,r");
    assert!(args.is_string_arg(&"p"))
}
