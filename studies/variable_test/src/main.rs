fn main() {
    println!("Hello, variable world!");

    let immutable_var = 100;    //변경불가한 변수
    let mut mutable_var = 300;   // 변경가능한 변수
    const CONST_VAR : i32  = 9999;   // 상수



    println!(" 변경가능한 변수 :  {} ", mutable_var);
    println!("변경불가한 변수 : {} ", immutable_var);
    println!(" 상수 : {} ", CONST_VAR);


    mutable_var = 377700;   // 변경가능한 변수

    println!(" 변경가능한 변수 :  {} ", mutable_var);
    println!("변경불가한 변수 : {} ", immutable_var);

}