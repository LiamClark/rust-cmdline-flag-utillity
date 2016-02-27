mod tests;

use std::env;

fn main() {
    let flags : Vec<_> = env::args().skip(1).collect();
    println!("{:?}", flags)
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
