mod userinput;

fn main() {
    println!("Hello, Everyone!");
    userinput::show1();
    userinput::show2();
    userinput::register();
    userinput::datatypes();
    let arr=[11,23,24];
    println!("arr[0]={}",arr[0]);
    println!("All elements of arr={:?}",arr);
    for i in 0..arr.len()
    {
        println!("{}",arr[i]);
    }
}

