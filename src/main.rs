mod tests;

use std::env;
use std::collections::HashMap;

fn main() {
    let flags : Vec<_> = env::args().skip(1).collect();

    for arg in flags.iter() {
        for i in arg.chars() {
            println!("{}",i);
        }
    }
    println!("{:?}", flags)
}


struct ArgMap {
    bools   : HashMap<String, bool>,
    // ints    : HashMap<String, i32>,
    // strings : HashMap<String, String>,
}

#[derive(PartialEq)]
enum ParseState {
    NewToken,
    StringFlag{name : String},
    IntFlag{name: String},
    BoolArg{name: String},
    InvalidFlag,
    Value,
    End,
}

impl ArgMap {
    pub fn new(format: &str, args: Vec<String> ) -> ArgMap {
        let mut bools   =   HashMap::new();
        // let mut ints    =   HashMap::new();
        // let mut strings =   HashMap::new();

        let format = FlagFormat::parse(format);

        for arg in args.iter() {
             let mut chars  = arg.chars();

             match chars.next() {
                 Some('-') => {
                     let name : String = chars.collect();
                     if format.is_bool_arg(name.trim()) {
                         bools.insert(name,true);
                     }
                 }
                 Some(_) => {}
                 None => {}
             };
        }

        ArgMap{bools : bools}
    }

    fn parse(&self, format : &FlagFormat, args:Vec<String>) {
        let mut state = ParseState::NewToken;
        let mut argIter = args.iter();

        while state != ParseState::End {
            match state {
                ParseState::NewToken => {
                    if let Some(s) = argIter.next() {
                        state = ArgMap::is_flag(s, format);
                    }
                }
                _ => {}
            }
        }
    }

    fn is_flag(arg: &str, format: &FlagFormat) -> ParseState {
         let mut chars = arg.chars();

         match chars.next() {
            Some('-') => {
                let name : String = chars.collect();
                ArgMap::flag_type(name, format)
            }
            _ => ParseState::InvalidFlag
          }
    }

    fn flag_type(name: String, format: &FlagFormat) -> ParseState {
        //TODO this is ugly.
        let clone = name.clone();
        let trim = clone.trim();
        if format.is_bool_arg(trim) {
            ParseState::BoolArg{name: name}
        } else if format.is_int_arg(trim) {
            ParseState::IntFlag{name: name}
        } else if format.is_string_arg(trim) {
            ParseState::StringFlag{name: name}
        } else {
            ParseState::InvalidFlag
        }
    }

    pub fn get_bool_arg(&self, name: &str) -> bool {
        self.bools.get(name).is_some()
    }
}


struct FlagFormat {
    bools   : Vec<String>,
    ints    : Vec<String>,
    strings : Vec<String>,
}

impl FlagFormat {
    pub fn parse(flag: &str) -> FlagFormat {
        let mut bools   =   Vec::new();
        let mut ints    =   Vec::new();
        let mut strings =   Vec::new();

        for s in flag.split(',') {
            let (name, postfix) = FlagFormat::split_argument_specifier(s);
            let name = name.into();
            match postfix {
                None => { bools.push(name); }
                Some(x) => match x {
                     "#" => { ints.push(name); }
                     "*" => { strings.push(name); }
                     _ => {}
                }
            };
        }
        FlagFormat{bools: bools, ints: ints, strings: strings}
    }

    fn split_argument_specifier(flag_string: &str) -> (&str, Option<&str>) {
        let size = flag_string.len();
        if size == 1 {
            (flag_string, None)
        } else {
            let (start, end) =  flag_string.split_at(size -1);
            (start, Some(end))
        }
    }

    pub fn is_bool_arg(&self, s : &str) -> bool {
        FlagFormat::is_arg(&self.bools, s)
    }

    pub fn is_int_arg(&self, s : &str) -> bool {
        FlagFormat::is_arg(&self.ints, s)
    }

    pub fn is_string_arg(&self, s : &str) -> bool {
        FlagFormat::is_arg(&self.strings, s)
    }

    fn is_arg(args: &Vec<String>, s: &str) -> bool {
        args.iter().find(|name| name.trim() == s).is_some()
    }
}
