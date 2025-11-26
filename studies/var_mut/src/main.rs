fn main() {
    println!("Hello, world!");
    // 소유자는 하나이지만 소유물울 변경할 수 있다.
    let mut i: i32 = 1; // 소유물이란 값이 아닌 그 값을 보관한 메모리 영역임
                        // 기존 소유물의 값을 변경할 수 있다.
    let ref_i = &mut i; //  메모리 영역은 그대로이고 내부의 값을 변경할 수 있다,
                        //let another_ref_i = &mut i;  // 가변 참조 빌려주기가 두 번 일어남

    println!(" {}  ", ref_i); //, another_ref_i);
}
