pub fn print()
{
//  println!("Lmao");

 let my_first_rust_variable ="test ";

 println!("{}", my_first_rust_variable)


}

pub fn datatypes()
{
    let x:bool = true;

    let y:i32 = -1;

    let z:char = 'a';
}

pub fn strings()
{
    let mut constant_mutable_string = "Hi";

    // //error
    // constant_immutable_string= "bye";

    let mut constant_immutable_string = String::from("hi");

    constant_immutable_string.push('!')

    
}