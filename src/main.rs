mod tests;

use std::env;
use std::collections::HashMap;

fn main() {
    let flags : Vec<_> = env::args().skip(1).collect();
    println!("{:?}", flags)
}

struct FlagFormat {
    bools   : HashMap<String,bool>,
    // ints    : HashMap<String,i32>,
    // strings : HashMap<String,String>,
}

impl FlagFormat {
    // fn new(tokens: Vec<String>) -> FlagFormat {
    //     FlagFormat{}
    // }

    pub fn parse<S: Into<String>>(flag: S) -> BoolArg {
        let flag_string = flag.into();

        flag_string.trim().split(',').map(|s| {
            if let Some((name, arg)) = parse_multi_letter(s.into()) {
                match arg.trim() {
                    "#" => BoolArg,
                    "*" => BoolArg,
                    s   => BoolArg,
                }
            } else {
                BoolArg
            }
        });

        BoolArg
    }

    pub fn parsePostFix(postfix: String) {
            match postfix.trim() {
                
            }
    }

    pub fn getBool(&self, s :&String) -> Option<&bool> {
        self.bools.get(s)
    }
}


pub fn parse_multi_letter(flag_string: String) -> Option<(String, String)> {
    if let Some(last) = flag_string.chars().last() {
        let split : (String,String) = flag_string.chars().partition( |c| {
            *c != last
        });
        return Some(split)
    }
    None
}
