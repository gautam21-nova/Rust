use std::any::type_name_of_val;


//here 'static - tells that, the borrowed string slice lifetime is depend on the application running to shutdown
//it will not dropped automatically (while running)
fn to_print_string()->&'static str{
    let name = "hello";
    return name;
}
fn to_print_integer(){
     //i8 -> signed integer ranging -128 to 127
     let byte_num:i8 = 100;
     println!("integer of 1byte or 8-bit: {}",byte_num);
    //i16 -> signed integer ranging -2^16 to 2^15-1
    let short_num:i16 = 500;
    println!("integer of 32-bits: {}",short_num);
    //i32 -> default value in rust for any int type (primitive data type) ranging -2^32 to 2^31-1
    let num:i32 = 900;
    println!("integer of 32-bits: {}",num);
    //i64 -> signed integer ranging -2^64 to 2^63-1
    let long_num:i64 = 500;
    println!("integer of 32-bits: {}",long_num);
}
fn to_print_decimal(){
    //rust has two data types for describing decimal -> 1. float32 2. float64(by default for more precision)
    let floating_point1:f32= 2.55;
    println!("{}",floating_point1);
    let floating_point2:f64= 2.5225;
    println!("{}",floating_point2);
}

//here we try to find the type of value with if - else condition 
fn to_print_boolean()->bool{
    //boolean -> true or false
    let check_num = 100;
    if type_name_of_val(&check_num)=="i32"{
        true
    }else{
        false
    }
}
//Entry point
fn main(){
    //In rust, data types are categorised into two type 
    //1. Scalar
    //2. Compound
    //We are looking for scalar data types -> string literals(&str), int(signed + unsigned), float, boolean.
    println!("String Literals example: {}",to_print_string());
    to_print_integer();
    to_print_decimal();
    println!("Bool value example: {}",to_print_boolean());
}