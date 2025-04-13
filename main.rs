fn add(a:i32,b:i32)-> i32{
    a+b
}


fn main(){
   let a = 38;
   let b = 4;
   let mut sum = add(a,b);
   
   sum += b;

   println!("{} + {} = {}" ,  a, b , sum)

}