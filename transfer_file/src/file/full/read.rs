use std::fs::File;
use std::io::prelude::*;

pub enum ResultCode {
    Success,
    Continue(u64),
    Error
}

pub struct CRead {
    file: File
}

impl CRead {
    pub fn read<F>(&self, start: u64, limit: u64, f: &mut F) -> Result<(), &str>
        where F: FnMut(&u64, &u64, &u64, Vec<u8>) -> ResultCode {
        let mut offset = start;
        let meta = match self.file.metadata() {
            Ok(m) => m,
            Err(err) => {
                println!("metadata error, err: {}", err);
                return Err("metadata error");
            }
        };
        let total = meta.len();
        loop {
            let file = self.file.try_clone();
            let mut file = match file {
                Ok(f) => f,
                Err(err) => {
                    println!("file try clone error, err: {}", err);
                    return Err("file try clone error");
                }
            };
            if offset >= total {
                break;
            }
            match file.seek(std::io::SeekFrom::Start(offset)) {
                Ok(size) => {
                    // if size == 0 {
                    //     break;
                    // }
                },
                Err(err) => {
                    println!("seek error, err: {}", err);
                    return Err("seek error");
                }
            }
            let mut once = total - offset;
            if once >= limit {
                once = limit;
            }
            let mut handler = file.take(once);
            let mut vec = Vec::new();
            vec.resize(once as usize, 0);
            match handler.read(vec.as_mut_slice()) {
                Ok(size) => {
                    if size == 0 {
                        println!("file end ...");
                        break;
                    }
                    match (*f)(&offset, &once, &total, vec) {
                        ResultCode::Error => {
                            return Err("function return error");
                        },
                        ResultCode::Continue(pos) => {
                            offset = pos;
                            continue;
                        },
                        ResultCode::Success => {}
                    }
                },
                Err(err) => {
                    println!("read error, err: {}", err);
                    return Err("read error");
                }
            }
            offset += once;
        }
        Ok(())
    }
}

impl CRead {
    pub fn new(path: &str) -> Option<CRead> {
        let file = File::open(path);
        let file = match file {
            Ok(f) => f,
            Err(err) => {
                println!("open file error, err: {}", err);
                return None;
            }
        };
        Some(CRead{
            file: file
        })
    }
}

#[test]
fn readTest() {
    let read = CRead::new("test.txt");
    let read = match read {
        Some(r) => r,
        None => {
            assert!(false);
            return;
        }
    };
    read.read(0, 256, &mut |start: &u64, once: &u64, data: Vec<u8>| {
        println!("start: {}, next: {}, data: {:?}", start, start + once, String::from_utf8(data));
        // if start == &500 {
        //     return ResultCode::Continue(0);
        // }
        return ResultCode::Success;
    });
}
