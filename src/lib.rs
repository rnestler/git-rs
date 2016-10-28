#[macro_use]
extern crate nom;

use nom::{IResult, be_u32};

pub struct IndexEntry {
    pub ctime_s: u32,
    pub ctime_ns: u32,
    pub mtime_s: u32,
    pub mtime_ns: u32,
    pub dev: u32,
    pub ino: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub file_size: u32,
    pub sha_1: [u8; 20],
    pub flags: u16,
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
    pub header: IndexHeader,
    pub entries: Vec<IndexEntry>,
    pub extensions: Vec<IndexExtension>,
    pub sha1_checksum: [u8; 20],
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_header() {
        let d = include_bytes!("../.git/modules/test_data/index");
        let header = parse_header(d);
        println!("{:?}", header);
    }
}
