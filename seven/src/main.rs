fn main() {
    println!("Hello, world!");
}

struct Directory {
    files: Vec<File>
}

struct File {
    size: usize,
    name: String
}

