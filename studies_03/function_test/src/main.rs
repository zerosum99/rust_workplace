fn main() {
    println!("Hello, function world!");

    let five = five();                   // 함수 호출

    println!(" 함수실행 : {} ", five);

    println!(" 바로 함수실행 결과 출력 : {}", plus_one(100));

    plus_two(200);

    let mut ss = String::from("Hello");
    plus_three(&mut ss);
    println!(" 가변참조 : {} ", ss);


}

fn five() -> i32 {
    5                          // 함수의 결과도 return 없이 표현식
}

fn plus_one(x:i32) -> i32 {    // 매개변수가 있는 함수
    x + 1
}

fn plus_two(x:i32) {
    if x < 100 {
        println!(" 100 미만");
    } else {
        println!(" 100 이상 ");
    }
}

fn plus_three(x:&mut String) {
    x.push_str(" world!!!");
}