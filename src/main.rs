// fn main() {
//     let s = String :: from ("hello" );
//     take_ownership(&s);
//     println!("{}",s);

// }


// fn take_ownership(text:&String){
//     println!("{}",text);
// }
fn main (){
    let mut x = 10;
    
    {
    let a = &mut x; 
    *a +=1;
    }

    {
     let b = &mut x; 
    *b +=1;
    }
    
   

    println!("{}",x);
}