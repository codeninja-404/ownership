fn main() {
    let s = String :: from ("hello" );
    take_ownership(&s);
    println!("{}",s);

}


fn take_ownership(text:&String){
    println!("{}",text);
}