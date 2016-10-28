#[macro_use]
extern crate nom;

use nom::{IResult, be_u32};

pub struct IndexEntry {
    ctime_s: u32,
    ctime_ns: u32,
    mtime_s: u32,
    mtime_ns: u32,
    dev: u32,
    ino: u32,
    mode: u32,
    uid: u32,
    gid: u32,
    file_size: u32,
    sha_1: [u8; 20],
    flags: u16,
}

pub struct IndexExtension {
}

/// The index header consists of 12 bytes:
///  * magic constant "DIRC"
///  * version
///  * number of index entries
#[derive(Debug)]
pub struct IndexHeader {
    /// BE version number
    version: u32,
    /// BE Number of index entries
    index_entries: u32,
}

pub fn parse_header(input: &[u8]) -> IResult<&[u8], IndexHeader> {
    chain!(input,
           tag!("DIRC") ~
           version: be_u32 ~
           index_entries: be_u32 ,
           || {
               IndexHeader{
                   version: version,
                   index_entries: index_entries,
               }
           }
          )
}

pub struct Index {
    header: IndexHeader,
    entries: Vec<IndexEntry>,
    extensions: Vec<IndexExtension>,
    sha1_checksum: [u8; 20],
}

#[cfg(test)]
mod tests {
    use std::io::prelude;
    use std::fs::File;
    use super::*;
    #[test]
    fn test_parse_header() {
        let d = include_bytes!("../.git/modules/test_data/index");
        let header = parse_header(d);
        println!("{:?}", header);
    }
}
