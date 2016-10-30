#[macro_use]
extern crate nom;

use nom::{IResult, be_u16, be_u32};

#[derive(Debug)]
pub struct IndexEntry<'a> {
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
    /// 20 bytes SHA1 sum
    pub sha_1: &'a [u8],
    pub flags: u16,
    pub path: &'a str,
}

pub fn parse_index_entry(input: &[u8]) -> IResult<&[u8], IndexEntry> {
    chain!(input,
           ctime_s: be_u32 ~
           ctime_ns: be_u32 ~
           mtime_s: be_u32 ~
           mtime_ns: be_u32 ~
           dev: be_u32 ~
           ino: be_u32 ~
           mode: be_u32 ~
           uid: be_u32 ~
           gid: be_u32 ~
           file_size: be_u32 ~
           sha_1: take!(20) ~
           flags: be_u16 ~
           // TODO if path_lengh is bigger than 0xFFF this will fail
           path: take_str!(flags & 0xFFF) ~
           // TODO calculate padding
           _padding: take!(2) ,
           || {
               IndexEntry {
                   ctime_s: ctime_s,
                   ctime_ns: ctime_ns,
                   mtime_s: mtime_s,
                   mtime_ns: mtime_ns,
                   dev: dev,
                   ino: ino,
                   mode: mode,
                   uid: uid,
                   gid: gid,
                   file_size: file_size,
                   sha_1: sha_1,
                   flags: flags,
                   path: path,
               }
           }
    )
}

#[derive(Debug)]
pub struct IndexExtension<'a> {
    /// Signature starts with 'A'-'Z'
    pub signature: &'a [u8],
    pub data: &'a [u8],
}

/// Parse as many extensions as possible, but leave 20 bytes for the SHA-1 checksum in the input
pub fn parse_index_extensions(input: &[u8]) -> IResult<&[u8], Vec<IndexExtension>> {
    let head = &input[0..input.len()-20];
    let tail = &input[input.len()-20..];

    println!("head {:?}", head);
    println!("tail {:?}", tail);

    let extensions: IResult<&[u8], Vec<IndexExtension>> = many0!(head, parse_index_extension);
    println!("{:?}", extensions);

    // TODO get rid of this unwrap
    let extensions = extensions.unwrap().1;

    IResult::Done(tail, extensions)
}

pub fn parse_index_extension(input: &[u8]) -> IResult<&[u8], IndexExtension> {
    chain!(input,
           signature: take!(4) ~
           length: be_u32 ~
           data: take!(length),
           || {
               IndexExtension{
                   signature: signature,
                   data: data,
               }
           }
          )
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

#[derive(Debug)]
pub struct Index<'a> {
    pub header: IndexHeader,
    pub entries: Vec<IndexEntry<'a>>,
    pub extensions: Vec<IndexExtension<'a>>,
    pub sha1_checksum: &'a[u8],
}

pub fn parse_index(input: &[u8]) -> IResult<&[u8], Index> {
    chain!(input,
           header: parse_header ~
           entries: count!(parse_index_entry, header.index_entries as usize) ~
           extensions: parse_index_extensions ~
           sha1_checksum: take!(20),
           || {
               Index {
                   header: header,
                   entries: entries,
                   extensions: extensions,
                   sha1_checksum: sha1_checksum,
               }
           }
          )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_header() {
        let d = include_bytes!("../.git/modules/test_data/index");
        let header = parse_header(d).unwrap().1;
        assert_eq!(header.version, 2);
        assert_eq!(header.index_entries, 1);
    }

    #[test]
    fn test_parse_index_extension() {
        let d: &[u8] = b"ABCD\x00\x00\x00\x01\x02";
        let extension = parse_index_extension(d).unwrap().1;

        assert_eq!(extension.signature, b"ABCD", "Wrong signature");
        assert_eq!(extension.data, &[2], "Wrong data");
    }

    #[test]
    fn test_parse_index_extensions() {
        let mut d: [u8; 29] = [0; 29];

        for (i, v) in b"ABCD\x00\x00\x00\x01\x02".iter().enumerate() {
            d[i] = *v;
        }

        println!("input: {:?}", d);

        let extensions = parse_index_extensions(&d).unwrap();

        println!("output: {:?}", extensions);

        assert_eq!(extensions.0.len(), 20);

        assert_eq!(extensions.1.len(), 1);
        assert_eq!(extensions.1[0].signature, b"ABCD");
        assert_eq!(extensions.1[0].data, &[2]);
    }

    #[test]
    fn test_parse_index_entry() {
        let d = include_bytes!("../.git/modules/test_data/index");
        let header = parse_header(d).unwrap();

        let index_entry = parse_index_entry(header.0).unwrap().1;

        assert_eq!(index_entry.file_size, 2);
        assert_eq!(index_entry.file_size, 2);
        println!("{:?}", index_entry);
    }

    #[test]
    fn test_parse_index() {
        let d = include_bytes!("../.git/modules/test_data/index");
        let index = parse_index(d);

        println!("{:?}", index);
    }
}
