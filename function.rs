fn main() {
    square();
    rectarea(3.4,4.3);
    println!("{}",even(59));
    println!("{}",add(5,9));
    checknumber(-7);
    printnumbers();
    forloopdemo();
    loopdemo();
}

pub fn square()
{
    let n=34;
    println!("Square={}",n*n)
}
pub fn rectarea(l:f32,b:f32)
{
    let ans;
    ans=l*b;
    println!("Area of rectangle={}",ans)
}
pub fn even(n:u8)->bool
{
let ans1:u8=n%2;
ans1==0
}

pub fn add(a:i16,b:i16)->i16
{
    let addition:i16=a+b;
    addition
}

pub fn checknumber(n:i8)
{
    if n>0
    {
    println!("n is +ve number");
    }
    else
    {
        println!("n is -ve number");
    }
}

pub fn printnumbers()
{
    let mut i=1;
    while i<=10
    {
        println!("{}",i);
        i=i+1;
    }
}

pub fn forloopdemo()
{
    for i in 1..6
    {
        println!("{}",i);      
    }
}

pub fn loopdemo()
{
    let mut j=11;
    loop
    {
        if j<=20
        {
            println!("{}",j);
        }
    j=j+1;
    }
}