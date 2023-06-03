pub fn show1()
{
    println!("Welcome user ");
}
pub fn show2()
{
    println!("for RUST programming");
}
pub fn register()
{
    let mut name = String::new();
    println!("Enter your name :");
    std::io::stdin().read_line(&mut name).unwrap();
    print!("Name={}",name);
 }

 pub fn datatypes()
 {
  let em1="\u{1F600}";
  println!("Emoji={}",em1);
  println!("Emoji={}","\u{1F601}");
  println!("Emoji={}","\u{1F602}");
  println!("Emoji={}","\u{1F605}");
 }