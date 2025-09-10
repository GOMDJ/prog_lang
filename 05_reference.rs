fn main(){
    let v : i32 = 2025 ;
    let ptr = &v ;
    println!("{}, {}", v, ptr) ;
    println!("{:p}", ptr) ; //{:p} 주소값 출력, 그냥 {}하면 ptr은 v의 값 출력.
    
    let x : u32 = 2030 ;
    let ptr_2 : *const u32 = &x ; //원시 포인터, u32(unsigned 32bit)
    println!("{:p}", &ptr_2) ; 
    
    let mut y : i32 = 2035 ;
    let ptr_3 = &mut y ; // mut 변수일 때 포인터도 mut로 해야됨.
    *ptr_3 = 2040 ;
    println!("{}, {:p}", ptr_3, &ptr_3) ;
}