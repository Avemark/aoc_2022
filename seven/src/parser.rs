use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{digit1, space0};
use nom::sequence::{preceded, Tuple};
use nom::{AsChar, IResult, Parser};

fn ls_directory<'a>(input: &'a str, parent: &'a DirNode) -> IResult<&'a str, Directory<'a>> {
    let (input, (_, name)) = (tag("dir "), take_while1(AsChar::is_alpha)).parse(input)?;

    Ok((
        input,
        Directory {
            name,
            contents: vec![],
            parent,
        },
    ))
}

fn filename(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic() || c == '.').parse(input)
}

fn ls_file(input: &str) -> IResult<&str, File> {
    let (input, (size, name)) = (digit1, preceded(space0, filename)).parse(input)?;

    let size = str::parse(size).expect("Can't parse integers for some reason");

    Ok((input, File { size, name }))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum FSNode<'a> {
    Directory(Directory<'a>),
    File(File<'a>),
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum DirNode<'a> {
    Directory(Directory<'a>),
    Root(Root<'a>),
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Root<'a> {
    contents: Vec<FSNode<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Directory<'a> {
    name: &'a str,
    parent: &'a DirNode<'a>,
    contents: Vec<FSNode<'a>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct File<'a> {
    size: usize,
    name: &'a str,
}

#[test]
fn filename_test() {
    let long_name = "long.txt";
    let short_name = "A";
    let bad_name = "!!!!";
    let long_string = "long.txt some other stuff";

    assert_eq!(filename(long_name), Ok(("", "long.txt")));
    assert_eq!(filename(short_name), Ok(("", "A")));
    assert!(filename(bad_name).is_err());
    assert_eq!(filename(long_string), Ok((" some other stuff", "long.txt")));
}

#[test]
fn ls_directory_test() {
    let root = DirNode::Root(Root { contents: vec![] });
    let good_dir = "dir foo";
    let file = "123 bar";

    let expected_dir = Directory {
        name: "foo",
        parent: &root,
        contents: vec![],
    };

    assert_eq!(ls_directory(good_dir, &root), Ok(("", expected_dir)));
    assert!(ls_directory(file, &root).is_err());
}

#[test]
fn ls_file_test() {
    let normal_file = "123 file.txt";
    let short_filename = "321 f";
    let directory = "dir hello";

    assert_eq!(
        ls_file(normal_file),
        Ok((
            "",
            (File {
                name: "file.txt",
                size: 123
            })
        ))
    );
    assert_eq!(
        ls_file(short_filename),
        Ok((
            "",
            (File {
                name: "f",
                size: 321
            })
        ))
    );
    assert!(ls_file(directory).is_err());
}
