use FlagFormat;
use ArgMap;


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

#[test]
fn test_one_arg() {
    let args = vec!["-p".into()];
    let argmap = ArgMap::new("p", args);
    assert!(argmap.get_bool_arg("p"));
}

#[test]
fn test_bool_arg_of() {
    let args = vec!["-l".into()];
    let argmap = ArgMap::new("p", args);
    assert!(!argmap.get_bool_arg("p"));
}

#[test]
fn test_int_arg() {
    let args = vec!["-p".into(), "34".into()];
    let argmap = ArgMap::new("p#", args);
    assert_eq!(argmap.get_int_arg("p"), Some(&34));
}

#[test]
fn bool_and_int_arg() {
    let args = vec!["-p".into(), "34".into(),"-l".into()];
    let argmap = ArgMap::new("p#,l", args);
    assert_eq!(argmap.get_int_arg("p"), Some(&34));
    assert!(argmap.get_bool_arg("l"));
}

#[test]
fn all_arg_types() {
    let args = vec!["-p".into(), "34".into(),"-l".into(), "hello".into(),"-r".into()];
    let argmap = ArgMap::new("p#,l*,r", args);
    assert_eq!(argmap.get_int_arg("p"), Some(&34));
    assert!(argmap.get_bool_arg("r"));
    assert_eq!(argmap.get_string_arg(&"l"),Some( &"hello".into()))
}
