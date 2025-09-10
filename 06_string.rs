fn main(){
    let name ="서태지" ; // 스택 메모리 저장, 정적, 기본적으로 변수선언하면 정적할당됨
    let other_name = String::from("Adrian Fahreheit Tepes") ; //from : 힙 메모리 사용, 메모리동적할당(string, vec, mut도 동적할당됨)

    println!("{:?}, {:?}", name, other_name) ; // {:?} : 디버그출력, 문자열은 디버그출력 안해도 출력됨.

    let together = format!("1 : {}, 2 : {}", name, other_name) ; // format! : 문자열 만드는 매크로, 출력은 안함
    println!("{}", together) ;

    println!("{} {} {}", name, std::mem::size_of_val(&name), std::mem::size_of_val("서 태 지")) ; //size_of_val : 메모리 크기 계산, name 변수에 16바이트가 할당돼있어서 메모리가 더 크게나옴.
    println!("{} {}", other_name, std::mem::size_of_val(&other_name)) ; //import처럼 use사용가능
    println!("{}", other_name.len()) ;
    dbg!(name, other_name) ; //dbg! : 디버깅용 매크로, 파일명, 줄 번호, 변수명까지 자동으로 출력
}