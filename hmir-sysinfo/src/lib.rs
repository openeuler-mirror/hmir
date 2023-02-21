#[macro_use]
extern crate lazy_static;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::fs::File;

fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
    File::open(&path).map_err(|why| io::Error::new(
        io::ErrorKind::Other,
        format!("unable to open file at {:?}: {}", path.as_ref(), why)
    ))
}

fn simple_line_read(path : & str) -> io::Result<String>
{
    let mut file = BufReader::new(open(path)?);
    let mut data = String::new();
    file.read_line(&mut data)?;
    Ok(data.trim_end().into()) //忽略换行符
}

pub mod release;
pub mod bios;
pub mod chassis;
pub mod machine;
pub mod product;
pub mod board;
