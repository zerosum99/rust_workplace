use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_number = rand::thread_rng().gen_range(1..101);


    println!("정답이라고 생각하는 숫자를 입력하세요 !");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("입력한 값을 읽지 못했습니다");

    println!("입력된 값 : {} ", guess);

    println!("랜덤 값 : {:?} ", secret_number);
}