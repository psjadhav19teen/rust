
fn main() 
{
    struct Student
    {
        name:String,
        rollno:u16,
        grade:char,
    }
let s1=Student
{
    name:String::from("aaa"),
    rollno:123,
    grade:'A',
};
let s2=Student
{
    name:String::from("bbb"),
    rollno:456,
    grade:'O',
};
println!("name={}",s1.name);
println!("rollno={}",s1.rollno);
println!("grade={}",s1.grade);

fn show(s2:Student)
{
println!("name={}",s2.name);
println!("rollno={}",s2.rollno);
println!("grade={}",s2.grade);
}
show(s2);
}



