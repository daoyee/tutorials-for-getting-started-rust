
fn main() {
   
   // 循环
   // loop <===> while(1) <==> for(;;)
   // continue,break;
   
   let mut count = 0u32;
   loop {
       count += 1;

       if count == 3 {
        println!("three");
        continue;
       }

       println!("{}", count);

       if count == 5 {
           println!("Ok");
           break;
       }


   }


}
