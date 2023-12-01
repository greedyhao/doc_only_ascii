use std::{fs::{self, File}, io::Write};

use bpaf::*;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Options {
    /// 需要被转换的文件路径
    #[bpaf(positional("file path"))]
    file_path: String,
}

fn main() {
    let path = options().run().file_path;

    let old = format!("{}.old", path);
    fs::rename(&path, &old).unwrap();
    let file = &fs::read(&old).unwrap();

    let mut out = File::create(&path).unwrap();

    let total = file.len();
    let mut cnt = 0;
    let mut buf = Vec::new();
    for b in file {
        cnt += 1;
        if *b < 128 {
            buf.push(*b);
        }
        if buf.len() >= 1024 || cnt == total {
            out.write_all(&buf).unwrap();
            buf.clear();
        }
    }
}
