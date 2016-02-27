mod tests;

use std::env;
use std::collections::HashMap;

fn main() {
    let flags : Vec<_> = env::args().skip(1).collect();
    println!("{:?}", flags)
}

struct FlagFormat {
    bools   : HashMap<String,bool>,
    ints    : HashMap<String,i32>,
    // strings : HashMap<String,String>,
}

impl FlagFormat {
    pub fn parse<S: Into<String>>(flag: S) -> FlagFormat {
        let flag_string = flag.into();
        let mut bools   =   HashMap::new();
        let mut ints    =   HashMap::new();
        let mut strings =   HashMap::new();

        for s in flag_string.trim().split(',') {
            let (name, postfix) = FlagFormat::split_argument_specifier(s.into());
            match postfix {
                None => { bools.insert(name,false); }
                Some(x) => match x.trim() {
                     "#" => { ints.insert(name, 3); }
                     "*" => { strings.insert(name, String::from("hello")); }
                     _ => {}
                }
            };
        }
        FlagFormat{bools: bools, ints: ints}
    }

    fn split_argument_specifier(flag_string: String) -> (String, Option<String>) {
        let size = flag_string.capacity();
        if size == 1 {
            (flag_string, None)
        } else {
            let (start, end) =  flag_string.split_at(size -1);
            (start.into(), Some(end.into()))
        }
    }

    pub fn getBool(&self, s :&String) -> Option<&bool> {
        self.bools.get(s)
    }

    pub fn getInt(&self, s: &String) -> Option<&i32> {
        self.ints.get(s)
    }
}
