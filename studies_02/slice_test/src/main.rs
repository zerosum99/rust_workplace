// 모듈을 지정하면 main.rs에 mod가 추가
mod first;
mod forif;
mod second; // 디렉토리 이름으로 모듈명, 모듈명은 mod.rs

mod fibo;
mod gugudan;

fn main() {
    println!("Hello, world!");
    let ss = String::from("Hello World ");
    let ll = first::first_words(&ss); // 함수로 이동시키지 않고 참조만 전달해서 결과를 받음

    println!(" 쿤자열 길이 : {} ", ll);
    let sll = first::first_words_str(&ss); // 함수로 전달해서 문자열 참조 받음
    println!(" 쿤자열 참조 {} ", sll);

    let ll1 = second::first_words(&ss);

    println!(" 세컨 문자열 길이 : {} ", ll1); // 두번째 문자열 받음
    let ssll = second::first_words_str(&ss);
    println!(" 세컨 문자열 참조 {} ", ssll);

    forif::for_if();

    gugudan::gugudan_pr();
    gugudan::gugudan_map();

    fibo::fibo();
}
