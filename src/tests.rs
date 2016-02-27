use FlagFormat;
use parse_multi_letter;


#[test]
fn parse_multi_test_one_letter(){
    let expected = ("p".to_string(),"*".to_string());
    assert_eq!(parse_multi_letter("p*".to_string()).unwrap(), expected);
}

#[test]
fn longer_name() {
    let expected = ("ttp".to_string(),"*".to_string());
    assert_eq!(parse_multi_letter("ttp*".to_string()).unwrap(), expected);
}


#[test]
#[ignore]
fn integration_test() {
    let args = FlagFormat::parse("p*, l#,r");
    let bool = args.getBool("r");
}
