/*
    자료형 선언.
    int = i32(32가 기본값(32비트 = 4바이트))
    float = f64
    a as f64 : float형으로 자료형 변경
    다른 자료형끼리 계산 불가
    MIN, MAX : 연관상수(미리 정의된 상수)
*/
fn main(){
    let a : i16 = 42 ;
    let b : f64 = 3.14 ;
    println!("{}, {}", a, b) ;

    let c = b - 1.01 ; // 자동으로 자료형 할당 float
    let d = a as f64 + b ; //a를 float로 변경 후 b와 합
    println!("{}, {}", c, d) ;

    let e = true ; // bool자료형
    let f : bool = false ; 
    let g : char = 'a' ;
    println!("{}, {}, {}", e, f, g) ; 

    let h = i8::MIN ;
    let i = i8::MAX ; 
    println!("{}, {}", h, i) ;
}