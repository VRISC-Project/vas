use std::mem::size_of;

#[derive(Debug)]
pub enum CSParseError {
    NotAInteger,
    Overflow,
}

pub trait Num {}

impl Num for u8 {}
impl Num for u16 {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for u128 {}
impl Num for usize {}
impl Num for i8 {}
impl Num for i16 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for i128 {}
impl Num for isize {}

pub trait Convert<T> {
    fn convert(self) -> T;
}

impl Convert<u8> for u64 {
    fn convert(self) -> u8 {
        self as u8
    }
}
impl Convert<u16> for u64 {
    fn convert(self) -> u16 {
        self as u16
    }
}
impl Convert<u32> for u64 {
    fn convert(self) -> u32 {
        self as u32
    }
}
impl Convert<u64> for u64 {
    fn convert(self) -> u64 {
        self as u64
    }
}
impl Convert<u128> for u64 {
    fn convert(self) -> u128 {
        self as u128
    }
}
impl Convert<usize> for u64 {
    fn convert(self) -> usize {
        self as usize
    }
}

impl Convert<i8> for u64 {
    fn convert(self) -> i8 {
        self as i64 as i8
    }
}
impl Convert<i16> for u64 {
    fn convert(self) -> i16 {
        self as i64 as i16
    }
}
impl Convert<i32> for u64 {
    fn convert(self) -> i32 {
        self as i64 as i32
    }
}
impl Convert<i64> for u64 {
    fn convert(self) -> i64 {
        self as i64 as i64
    }
}
impl Convert<i128> for u64 {
    fn convert(self) -> i128 {
        self as i64 as i128
    }
}
impl Convert<isize> for u64 {
    fn convert(self) -> isize {
        self as i64 as isize
    }
}

pub trait CSParse<T: ToString, N: Num> {
    fn csparse(&self) -> Result<N, CSParseError>;
}

fn trim_basestr(s: &String) -> String {
    if s.len() < 2 {
        return String::new();
    }
    String::from_utf8(s.as_bytes()[2..].to_vec()).unwrap()
}

pub fn is_number(num: &String) -> bool {
    let mut src = num.to_string();
    let cs = if src.starts_with("0x") && trim_basestr(&src).chars().all(|c| c.is_digit(16)) {
        16usize
    } else if src.starts_with("0o") && trim_basestr(&src).chars().all(|c| c.is_digit(8)) {
        8usize
    } else if src.starts_with("0b") && trim_basestr(&src).chars().all(|c| c.is_digit(2)) {
        2usize
    } else if trim_basestr(&src).chars().all(|c| c.is_digit(10)) {
        10usize
    } else {
        0
    };
    if cs != 10 {
        src = String::from_utf8(src.as_bytes()[2..].to_vec()).unwrap();
    }
    match cs {
        2 => {
            for c in src.as_str().chars() {
                if !c.is_digit(2) {
                    return false;
                } else {
                    continue;
                }
            }
            true
        }
        8 => {
            for c in src.as_str().chars() {
                if !c.is_digit(8) {
                    return false;
                } else {
                    continue;
                }
            }
            true
        }
        10 => {
            for c in src.as_str().chars() {
                if !c.is_digit(10) {
                    return false;
                } else {
                    continue;
                }
            }
            true
        }
        16 => {
            for c in src.as_str().chars() {
                if !c.is_digit(16) {
                    return false;
                } else {
                    continue;
                }
            }
            true
        }
        _ => false,
    }
}

impl<T: ToString, N: Num> CSParse<T, N> for T
where
    u64: Convert<N>,
{
    fn csparse(&self) -> Result<N, CSParseError> {
        let mut src = self.to_string();
        let _ = src.trim();
        let cs = if src.starts_with("0x") && trim_basestr(&src).chars().all(|c| c.is_digit(16)) {
            16usize
        } else if src.starts_with("0o") && trim_basestr(&src).chars().all(|c| c.is_digit(8)) {
            8usize
        } else if src.starts_with("0b") && trim_basestr(&src).chars().all(|c| c.is_digit(2)) {
            2usize
        } else if src.chars().all(|c| c.is_digit(10)) {
            10usize
        } else {
            return Err(CSParseError::NotAInteger);
        };
        if cs != 10 {
            src = String::from_utf8(src.as_bytes()[2..].to_vec()).unwrap();
        }
        match cs {
            2 => {
                let mut num = 0u64;
                if src.len() > size_of::<N>() * 8 {
                    Err(CSParseError::Overflow)
                } else {
                    for &i in src.as_bytes() {
                        num <<= 1;
                        num += (i - b'0') as u64;
                    }
                    Ok(num.convert())
                }
            }
            8 => {
                let mut num = 0u64;
                if src.len() > size_of::<N>() * 8 / 3 {
                    Err(CSParseError::Overflow)
                } else {
                    for &i in src.as_bytes() {
                        num <<= 3;
                        num += (i - b'0') as u64;
                    }
                    Ok(num.convert())
                }
            }
            10 => {
                let mut num = 0u64;
                if src.len() > size_of::<N>() * 8 / 3 {
                    Err(CSParseError::Overflow)
                } else {
                    for &i in src.as_bytes() {
                        num *= 10;
                        num += (i - b'0') as u64;
                    }
                    Ok(num.convert())
                }
            }
            16 => {
                let mut num = 0u64;
                if src.len() > size_of::<N>() * 2 {
                    Err(CSParseError::Overflow)
                } else {
                    for i in src.as_str().chars() {
                        num <<= 4;
                        num += i.to_digit(16).unwrap() as u64;
                    }
                    Ok(num.convert())
                }
            }
            _ => panic!("Unknown error."),
        }
    }
}
