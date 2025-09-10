// 반환타입 : i32
fn two() -> i32 {
    2
}

// 매개변수 반환타입 : i32, 함수 반환타입 : i32
fn double(n : i32) -> i32 {
    n*2
}

fn multiply(n : i32, fact : i32) ->i32 {
    fact * n
}

fn main(){
    let n = two() ; 
    println!("{}, {}", n, double(n)) ;

    println!("{}", multiply(3, 4)) ;
}