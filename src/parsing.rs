use regex::Regex;

pub fn new_re(s: &str) -> Regex {
    Regex::new(s).unwrap()
}

pub fn re_captures<'a>(re: &Regex, s: &'a str) -> impl Iterator<Item = &'a str> {
    re.captures(s)
        .unwrap()
        .iter()
        .skip(1)
        .map(|om| om.map_or("", |m| m.as_str()))
        .collect::<Vec<_>>() // Not sure how to avoid this (lifetimes)
        .into_iter()
}

pub fn parse_re<'a, T: Extract<'a>>(re: &Regex, s: &'a str) -> T {
    T::extract(&mut re_captures(re, s))
}

pub trait Extract<'a> {
    fn extract<I: Iterator<Item = &'a str>>(iter: &mut I) -> Self;
}

impl<'a> Extract<'a> for &'a str {
    fn extract<I: Iterator<Item = &'a str>>(iter: &mut I) -> Self {
        iter.next().unwrap()
    }
}

impl<'a> Extract<'a> for String {
    fn extract<I: Iterator<Item = &'a str>>(iter: &mut I) -> Self {
        iter.next().unwrap().into()
    }
}

macro_rules! extract_impl {
    ($($t:ty)+) => {
        $(
            impl<'a> Extract<'a> for $t {
                fn extract<I: Iterator<Item = &'a str>>(iter: &mut I) -> Self {
                    let s = iter.next().unwrap();
                    if let Ok(res) = s.parse() {
                        res
                    } else {
                        panic!("Cannot convert string '{}'", s)
                    }
                }
            }
         )+
    }
}
extract_impl!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize char bool);

macro_rules! extract_tuple_impl {
    ($($types:ident),+) => {
        impl<'a, $($types: Extract<'a>),+> Extract<'a> for ($($types),+){
            fn extract<I: Iterator<Item=&'a str>>(iter: &mut I) -> Self {
                ($($types::extract(iter)),+)
            }
        }
    }
}
extract_tuple_impl!(A, B);
extract_tuple_impl!(A, B, C);
extract_tuple_impl!(A, B, C, D);
extract_tuple_impl!(A, B, C, D, E);