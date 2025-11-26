use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, File world!");

    let f = File::open("/Users/a06411/Documents/rust_projects/file_test/src/main.rs");

    let mut ff = match f {
        Ok(file) => file,
        Err(error) =>  {
            panic!(" 파일열기 실패 : {:?}", error);
        }
    };

    let mut s = String::new();

    ff.read_to_string(&mut s);    // 갱신을 처리하는 파일

    println!(" {}",s);

    println!(" end main");
}
