pub struct CCreateParam<'a> {
    pub sharedMode: &'a str,
    pub transmitServiceFindMode: &'a str,
    pub dial: &'a str,
    pub port: u32,
    pub nat4IsTryMake: bool,
    pub threadMax: i32
}

pub mod ruled_out;
