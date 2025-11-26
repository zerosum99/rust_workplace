
pub fn fibo() {
    let mut a  = 1;
    let mut b = 1;

    println!("{}", a);
    println!("{}", b);

    for _ in 1..30 {
        println!("{}", a+b);
        let tem = a ;
        a=b;
        b= tem+b;
    }
}