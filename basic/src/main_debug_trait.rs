/*
fn main() {
    println!("Hello, world!");

    //let answer = 80;
    //assert_eq!(answer, 81);

    for i in 0..20 {
        if i % 2 == 0 {
            print!("{}\t", i);
        }
    }

    println!("");

    let var = if 10 % 2 == 0 { "Even" } else { "Odd" };
    println!("Var : {}", var);
}

*/

/*

//Sum of numbers

fn main() {
    let mut sum = 0.0;

    for i in 0..11 {
        sum += i as f32;
    }

    println!("Sum : {}", sum);
}

*/

/*
// Usage of functions

fn square(x: f64) -> f64 {
    x * x
}

fn main() {
    println!("Sq: {}\n", square(2));
}

*/

/*
// Factorial of a number

fn facto(n: i64) -> i64 {
    if n == 1 {
        return n
    }

    n * facto(n - 1)
}

fn main() {
    println!("Factorial of {} is {}", 8, facto(8));
}
*/

/*
// References

fn refer(x: &mut i32)  {
    *x += 1;
}

fn main() {
    let mut x = 5;

    refer(&mut x);
    println!("{}", x);
}

*/

/*
// Tan of value

fn main() {
    let pi: f32 = 3.1416;
    let x = pi / 4.0;

    println!("Tan of {} is {}", x, x.tan());
}

*/

/*

use std::f64::consts::PI;

fn main() {
    let x = 2.0 * PI;
    let abs_diff = (x.cos() - 1.0).abs();
    assert!(abs_diff < 1e-10);
}

*/

/*
// Usage of arrays

fn main() {
    let mut arr = [13, 45, 23, 56, 34, 576];

    arr[2] = 345;

    for i in 0 .. arr.len() {
        println!("[{}] : {}", i, arr[i]);
    }
}
*/

/*

// Borrow an array

fn arr_val_sum(arr: & [i32]) -> i32 {
    let mut val = 0;

    for i in 0 .. arr.len() {
        //arr[i] = i as i32;
        val += arr[i];
    }

    val
}

fn main() {
    let arr = [34, 56, 34, 67, 34, 45];

    println!("Array sum: {}", arr_val_sum(&arr));

    for i in 0 .. arr.len() {
        println!("[{}] : {}", i, arr[i]);
    }
}
*/

/*

// Printing the elements of the array
fn main() {
    let arr_int = [1, 2, 3, 4, 5];
    let arr_float = [1.2, 2.3, 3.4];
    let arr_strings = ["Vijay", "Manohar", "Dogiparthi"];
    let arr_arr = [[1, 2], [3, 4]];

    println!("arr_int: {:?}", arr_int);
    println!("arr_float: {:?}", arr_float);
    println!("arr_strings: {:?}", arr_strings);
    println!("arr_arr: {:?}", arr_arr);
}
*/

/*
// Array slicing

fn main() {
    let arr_ints = [1, 2, 3, 4, 5];
    let slice1 = &arr_ints[1..4];
    let slice2 = &arr_ints[1 ..];

    println!("arr_int: {:?}", arr_ints);
    println!("slice1: {:?}", slice1);
    println!("slice1: {:?}", slice2);

    let maybe_last = slice1.get(4);
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap()
    } else {
        -1
    };

    println!("First element of slice: {:?}", last);
}

*/

/*
// Usize and Isize demonstration

fn main() {
    let count : usize = 1100;

    for i in 0 .. count {
        print!("{}\t", i);
    }
}
*/

/*
//Numbers with separaters

fn main() {
    let num = 256_000;

    println!("num: {}", num);

    std::process::exit(0);
}
*/

/*

//Usage of unicode characters

use std::mem::size_of_val;

fn main() {
 let emoji = '😁';
 let special_chars = '£';
 let char_num = 342;

 println!("Emoji : {}\tSize of emoji: {}", emoji, size_of_val(&emoji));
 println!("Special chars : {}\tSize of special_chars: {}", special_chars, size_of_val(&special_chars));
 println!("Char num : {}\t Sizeof char_num : {}", char_num, size_of_val(&char_num));
}
*/

/*

//Usage of CONSTANTS

fn main() {
    const PI: f64 = 3.1412;
    let radius = 3.5;

    println!("Area of the circle: {}", PI * pow(radius, 2));
}
*/

/*

// Variable shadowing
fn foo() ->Result<, ()> {
    Ok(foo)
}


fn main() {
    let _var = 10;
    let var = 10.5;

    let x = foo();

    println!("Var : {}", var);
    println!("x : {}", x);
}
*/

/*

// Shadowing of variables
fn foo1() -> Result<bool, ()> {
    Ok(true)
}

fn foo2() -> Result<bool, ()> {
    Ok(false)
}

fn main() {
    let x = 5.0;//foo1();  // warning: unused variable: `x`
    let x = foo2(); // no warning

    println!("{:?}", x);
} // x and _x are dropped here
*/

/*

// String literals
fn main() {
    let name = "V. M. Dogiparthi";

    println!("name: {}\tLength of the name: {}", name, name.len());
}

*/

/*

// Panic for unrecoverable errors

fn main() {
    panic!("unrecoverable error");
    println!("Hello world!");
}

*/

/*
// Panic for bussiness rules

fn main() {
    let num = 35;

    if num % 2 == 0 {
        println!("Number is even!!");
    } else {
        panic!("Not allowed number!!");
        //unimplemented!();
    }
}

*/

/*
// String object

fn main() {
    let mut leeg_str = String::new();
    println!("Size of the empty string: {}", leeg_str.len());

    //let string = String::from("V. M. Dogiparthi");
    let mut string = String::from("V. M.");

    string.push_str(" Dogiparthi");
    println!("string is : {}", string);

    leeg_str = "Hello
    World!!".to_string();
    //println!("Empty string now: {}\n", leeg_str);

    print_string(leeg_str);


    let mut split_str = "A sack full of balls".to_string();
    print_split_string(&mut split_str);
    println!("{} ", split_str);
}

fn print_string(string : String) {
    println!("String received: {:?}", string);
}

fn print_split_string(split_str : & mut String) {
    for string in split_str.split_whitespace() {
        print!("{} ", string);
    }

    print!("\n");

    *split_str = "VM Dogiparthi".to_string();
}

*/

/*
// Usage of vector

fn main() {
    let full_name = "Vijay:Manohar:Dogiparthi".to_string();

    let sub_strings:Vec<&str> = full_name.split(':').collect();

    for i in 0 .. sub_strings.len() {
        println!("token{} : {}", i, sub_strings[i]);
    }
}

*/

/*

// Iterator of chars

fn main() {
    let mut string = "Vijaya Manohar Dogiparthi".to_string();

    for i in string.chars() {
        print!("{}", i);
    }
}

*/

/*

// Match statement

fn main() {
    let mut check : bool = true;

    match check {
        true => { println!("{}", "Indeed true");
                  println!("{}", " Truly true");
             },
        false => { println!("{}", "No, its false")},
        //_ => println!("{}", "It does not matter")
    };
}

*/

/*
// Print the tables

fn main() {
    const TABLE_NUM : u32 = 10000;

    for i in 1 .. TABLE_NUM + 1 {
        let mut j = 1;

        println!("{}th table:", i);

        while j < 21 {
            println!("{} * {} = {}", i, j, i * j);
            j += 1;
        }
    }
}

*/

/*
// Pass by value

fn print_multiplication_table(mut table_num : u32) {

        let mut j = 1;

        println!("{}th table:", table_num);

        while j < 21 {
            println!("{} * {} = {}", table_num, j, table_num * j);

            j += 1;
        }

}

fn main() {
    const TABLE_NUM : u32 = 10000;

    for i in 1 .. TABLE_NUM + 1 {
        print_multiplication_table(i);
    }
}

*/

/*
// Pass by reference

fn print_multiplication_table(table_num : & u32) {

        let mut j = 1;

        println!("{}th table:", *table_num);

        while j < 21 {
            println!("{} * {} = {}", *table_num, j, *table_num * j);

            j += 1;
        }

}

fn main() {
    const TABLE_NUM : u32 = 10000;

    for i in 1 .. TABLE_NUM + 1 {
        print_multiplication_table(&i);

    }
}

*/

/*
// Ownwership movement for non-primitive types

fn main(){
   let name:String = String::from("TutorialsPoint");
   display(name);
   //cannot access name after display
   println!("param_name value is :{}", name);
}

fn display(param_name:String){
   //println!("param_name value is :{}",param_name);
}

*/

/*
struct Obj {
    a: i32,
    b: i32,
}

fn main() {
    let obj1 = Obj { a: 20, b: 70 };
    let obj2 = obj1;

    println!("Obj1.a : {}", obj1.a);
}

*/

/*

// Tuple usage

fn main() {
    let tuple_name = ("V.M. Dogiparthi", 39, "Software Engineer");

    print_tuple(tuple_name);
}

fn print_tuple(tuple_name:(&str, i32, &str)) {
    let x : (&str, i32, &str) = tuple_name;
    let (name, age, profession) = x;

    println!("Name: {:?}", tuple_name);
    println!("Name: {:?}", x);

    println!("Age: {}", name);
    println!("Age: {}", age);
    println!("Profession: {}", profession);

}

*/

/*
// Arrays

fn main() {
    const ARR_SIZE : usize = 5;
    let mut arr1: [i32; ARR_SIZE] = [1, 2, 3, 4, 5];
    let mut arr2: [i32; ARR_SIZE] = [0; 5];

    print_arrs_by_val(arr1, arr2);

    print_arrs_by_ref(&mut arr1, &mut arr2);

    arr1[0] = 0;
    arr2[0] = 10;
}

fn print_arrs_by_val(mut arr1: [i32; 5], mut arr2: [i32; 5]) {
    println!("\nArray 1 elements: ");

    for i in 0..arr1.len() {
        print!("{}\t", arr1[i]);
    }

    println!("\n\nArray 2 elements:");

    for i in arr2.iter() {
        print!("{}\t", i);
    }

    arr1[0] = 20;
    arr2[0] = 30;
    println!("");
}

fn print_arrs_by_ref(arr1: &mut [i32; 5], arr2: &mut [i32; 5]) {
    println!("\nArray 1 elements: ");

    for i in 0..arr1.len() {
        print!("{}\t", arr1[i]);
    }

    println!("\n\nArray 2 elements:");

    for i in arr2.iter() {
        print!("{}\t", i);
    }

    arr1[0] = 40;
    arr2[0] = 50;

    println!("");
}
*/

/*

// Transferring the ownership by assignment

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];

    let v2: Vec<i32>;

    // Ownership of data transferred after the assignment
    v2 = v1;

    //println!("V1 contents: {:?}", v1);
    println!("V2 contents: {:?}", v2);
}

*/

/*
// Transferring the ownership by passing to a function

fn main() {
    let mut v1: Vec<i32> = vec![45, 56, 45, 8, 45, 68, 45, 567];
    let mut v2 = print_vec(v1);

    //v1[1] = 1;
    v2[1] = 2;

    print_vec(v2);
}

fn print_vec(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter() {
        print!("{}\t", i);
    }

    println!("");
    v[1] = 3;

    v
}

*/

/*

fn main() {
    let mut vect: Vec<i32> = vec![1, 2, 3, 4, 5];

    //transfer_ownership(vect); // Pass by value - transfer the ownership

    // Cannot modify, because the ownership is transferred from the above function call to the
    // called function!!

    //vect[0] = 0;

    borrow_ownership(&mut vect);

    // Can modify, because the ownership was earlier lent and reclaimed after the call
    vect[0] = 0;
}

fn transfer_ownership(mut v: Vec<i32>) {
    v[0] = -1;

    for iter in v.iter() {
        print!("{}\t", iter);
    }

    println!("");
}

fn borrow_ownership(v: &mut Vec<i32>) {
    v[0] = -1;

    for iter in v.iter() {
        print!("{}\t", iter);
    }

    println!("");
}

*/

/*
// Slicing

fn main() {
    let mut arr: [i32; 10] = [0; 10];

    let arr_slice = &mut arr[2..5];

    print_arr_slice(arr_slice);

    for i in 0..arr.len() {
        print!("{}\t", arr[i]);
    }

    print!("\n");
}

fn print_arr_slice(arr_slice: &mut [i32]) {
    arr_slice[1] = 1;

    for i in arr_slice.iter() {
        println!("{}\t", i);
    }

    print!("\n");
}

*/

/*
// Usage of structs

struct Employee {
    name: String,
    profession: String,
}

fn main() {
    let emp = Employee {
        name: "V. M. Dogiparthi".to_string(),
        profession: "Software engineer".to_string(),
    };

    println!(
        "Employee details\nName: {}, Profession: {}",
        emp.name, emp.profession
    );
}

*/

/*

// Methods on Structs
struct Rectangle {
    width: u8,
    breadth: u8,
}

impl Rectangle {
    fn get_instance(width: u8, breadth: u8) -> Rectangle {
        Rectangle {
            width: (width),
            breadth: (breadth),
        }
    }

    fn compute_area(&self) -> u32 {
        (self.width * self.breadth).into()
    }

    fn compute_perimeter(&self) -> u32 {
        (2 * (self.width + self.breadth)).into()
    }
}

fn main() {
    /*
    let rect: Rectangle = Rectangle {
        width: 10,
        breadth: 8,
    };
    */

    let rect = Rectangle::get_instance(10, 8);

    println!("Area of the rect is {}", rect.compute_area());
    println!("Perimeter of the rect is {}", rect.compute_perimeter());
}

*/

/*
// Usage of modules
pub mod movies {
    pub fn play_movie(movie_name : &str) -> i8 {
        println!("Movie {} is being played", movie_name);
        0
    }
}

use movies::play_movie;

fn main() {
    play_movie("Bahubali 2 - The conclusion");
}

*/

/*
fn debug_log(log_str: &String) {
        println!("{}", log_str);
}

fn main() {
    let sdac_ctrl_reg_val = 344354;

    let dbg_str = format!("{}0x{:02x}", "QPC calibration register value: ", sdac_ctrl_reg_val);

    debug_log(&dbg_str);
 }

*/

/*
fn check_even(num: u32) -> Result<bool, String> {
    if num % 2 == 0 {
        Ok(true)
    } else {
        Err("Given number is not even".to_string())
    }
}

fn main() {
    let res = check_even(4);

    match res {
        Ok(r) => {
            if r {
                println!("Number is even");
            }
        }
        Err(msg) => {
            println!("{}", msg);
        }
    }
}
*/

/*
use std::io::{self, BufRead, BufReader, Write};
use std::time::Duration;

fn main() {
    let mut port = serialport::new("/dev/ttyS0", 115_200)
        .timeout(Duration::from_millis(6000))
        .open()
        // expect does exactly the same thing as your match did
        .expect("Error: ");

    // in your python you use readline, to archive this
    // in rust the easiest way is to use a BufReader
    let mut port = BufReader::new(port);
    let mut line_buffer = String::new();
    let config = vec![
        "AT+CFG=433500000,5,9,7,1,1,0,0,0,0,3000,8,4\r\n",
        "AT+RX\r\n",
        "AT+SAVE\r\n",
    ];

    for entry in config.iter() {
        // get_mut is required because BufReader does not implement
        // io::Write
        port.get_mut()
            // use write_all because write does not guarantee that
            // every byte gets written
            .write_all(entry.as_bytes())
            .expect("Write failed!");

        // Duration::from_millis only returns a Duration and does not sleep
        std::thread::sleep(Duration::from_millis(1500));

        // clear the line buffer to use it again
        line_buffer.clear();
        // previously you never stopped reading
        port.read_line(&mut line_buffer).expect("Read failed!");

        // maybe you have a reason to use io::stdout().write_all()
        // but most times you can just use println or print
        print!("{}", line_buffer);
    }
}
*/

/*
// Generate Oops
fn main() {
    panic!("Oops");
}

*/

/*
fn foo(x : u32) -> u32 {
    x
}

fn main() {
    let x;

    x = 5;

    let _ = println!("{:?}", foo(x));
}
*/

/*
fn main() {
    eprintln!("Rust Programming");
    eprintln!(" Course");
}
*/

/*
#[allow(unused_variables, unused_mut)]

fn main() {
    let mut programming_language = "Rust";

    println!("Language being learned: {}", programming_language);

    programming_language = "C++";

    println!("Language being learned: {}", programming_language);

    let (course, category) = ("Rust", "Programming Language");

    println!("Learning {1} {0}", category, course);

}
*/

/*
fn main() {
    let a: i32 = 8;
    println!("a : {}", a);
}
*/

/*
fn main() {
    let c: char = '8';
    println!("a : {}", c);
}
*/

/*
fn main() {
    let mut arr:[i32; 5] = [1, 2, 3, 4, 5];

    println!("Array elements: {:?}", arr);
    println!("Length of the array: {}", arr.len());

    arr[4] = 6;

    println!("Array elements: {:?}", arr);

    let arr_slice1:&[i32] = &arr;
    let arr_slice2:&[i32] = &arr[1..6];

    println!("array slice 1: {:?}", arr_slice1);
    println!("array slice 2: {:?}", arr_slice2);
}
*/

/*
fn main() {
    let mut emp: (&str, i32, &str) = ("Vijay", 39, "Ointwiklaar ICT 2");

    println!("Details of the employee: {:?}", emp);

    emp.1 = 38;

    let (name, age, position) = emp;

    println!("Name of the employee: {}", name);
    println!("Age of the employee: {}", age);
    println!("Position within the organisation: {}", position);
}
*/

/*
// Use of CONST

fn main() {
    const PI_VAL: f32 = 3.14;
    let radius = 2;

    {
        const PI_VAL: f32 = 3.143;
        let radius = 2;
        println!("Area of a circle: {}", PI_VAL * radius as f32 * radius as f32);
    }


    println!("Area of a circle: {}", PI_VAL * radius as f32 * radius as f32);
}

*/

/*
// Usage of as operator

fn main() {
    let a: i32 = 5;

    println!("a as floatig point: {}", a as &str);

}
*/

/*

// Mutable and Immutable borrowing

fn main() {
    let x = 10;
    let mut y = 20;

    println!("x : {}", x);
    println!("y : {}", y);

    y = 30;

    println!("y : {}", y);

    let a = &mut y;

    let  b = &x;

    println!("a : {}", a);

    *a = 100;

    println!("a : {}", a);
    println!("x : {}", y);

    println!("b : {}", b);
}
*/

/*
// Mutable and Immutable borrowing

fn main() {
    let x = &9;
    let y = & mut 10;

    *y = 11;

    println!("x : {}", x);
    println!("y : {}", y);
}
*/

/*

//(a + b) ^ 3 Computation

fn main() {
    let a = 2;
    let b = 2;
    let res: i64 = i64::pow(a, 3) + i64::pow(b, 3) + 3 * a * b * (a + b);

    println!("(a + b) ^ 3: {}", res);
}

*/

/*

// Usage of if statement

fn main() {
    let taal = "Dutch";

    if taal == "English" {
        println!("You are learning English language\n");
    }
    else if taal == "Dutch" {
        println!("You are learning Dutch language\n");
    }

    println!("You are learning {} language\n", if taal == "Dutch" {"Dutch"} else {"English"});
}
*/

/*
// Usage of if let
fn main() {
    let languages = ("Telugu", "English", "Hindi", "Kannada", "Dutch");

    if let ("Telugu", "English", "Hindi", d, c) = languages {

        //println!("Wrote first two values in pattern to be matched with the scrutinee expression : {}", c);

        println!("Some match: {} {}", c, d);
    }
    else  {
        println!("No expression matches");
    }

    if let _ = "Dutch" {
        println!("Learning Dutch\n");
    }
}
*/

/*
// Usage of match keyword

fn main() {
  let programming_language = "Rust";

  match programming_language {
    "C++" => {
        println!("C++ programming language");
    },

    "Python" => {
        println!("Python programming language");
    },

    "Rust" => {
        println!("Rust programming language");
    },

    _ => {
            println!("C++ programming language");
        }
  };
}
*/

/*
fn main() {
    let pl = "Rust";

    let lang = match pl {
        "Rust" => {"Rust is the language"},
        "Python" => {"Python is the language"},
        "C++" => {"C++ is the language"},
        _ => {"No known language"},
    };

    println!("{}", lang);
}
*/

/*
fn test(a: i32, operator: char, b: i32) {
    match operator {
        '+' => {
            print!("{}", a + b);
        }

        '-' => {
            print!("{}", a - b);
        }

        '/' => {
            if b != 0 {
                print!("{}", a / b);
            } else {
                print!("Division by 0 is undefined");
            }
        }

        '*' => {
            print!("{}", a * b);
        }

        '%' => {
            if b != 0 {
                print!("{}", a % b);
            } else {
                print!("Division by 0 is undefined");
            }
        }

        _ => {
            print!("invalid operator");
        }
    };
}

fn main() {
    test(5, '+', 3);
    println!("\n");
}
*/

/*
fn main() {
    for i in 0..5 {
        println!("{}\t", i);
    }
}
*/

/*
// Enumeration

fn main() {
    for (count, variable) in (7..15).enumerate() {
        println!("Count : {}\tVariable: {}\n", count, variable);
    }
}
*/

/*
// Loop construct
fn main() {
    let mut iterator = 0;

    loop {
        println!("{}\t", iterator);

        iterator += 1;

        if iterator > 100 {
            break;
        }
    }
}
*/

/*
// Print the multiplication tables

fn main() {
    'outer: for i in 1..10001 {
        println!("\n{}th table\n", i);

        'inner: for j in 1..11 {
            println!("{} * {} = {}\n", i, j, i * j);
        }
    }
}
*/

/*
fn swap(x : &mut i32, y: &mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}

fn main() {
  let mut x = 10;
  let mut y = 12;

  swap(&mut x, &mut y);

  println!("x : {}\ty : {}\n", x , y)
}
*/

/*
fn square(x : & i32) -> i32 {
    let sq = *x * *x;
    sq
}

fn main() {
    let x = 5;
    println!("Square of {} is {}\n", x, square(&x));
}
*/

/*
fn main() {
    let x = 5;
    let (area, perimeter) = calc(&x);

    println!("Area : {}\tPerimeter: {}\n", area, perimeter);
}

fn calc(x : &i32) -> (i32, i32) {
    let area = *x * *x;
    let perimeter = 4 * *x;

    return (area, perimeter);
}
*/

/*
fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    println!("1. {:?}", arr);

    change_arr_locally(arr);

    println!("2. {:?}", arr);

    let rc_arr = change_by_returning(arr);

    println!("3. {:?}", rc_arr);

    change_arr(&mut arr);

    println!("4. {:?}", arr);
}

fn change_arr_locally(mut arr:[i32; 5]) {
    arr[0] = 0;

    println!("5. {:?}", arr);
}

fn change_by_returning(mut arr: [i32; 5]) -> [i32; 5] {
    arr[0] = 10;

    return arr;
}

fn change_arr(arr:&mut [i32; 5]) {
    arr[0] = 12;
}
*/

/*
fn main() {
    let arr = [1, 2, 3, 4, 5];

    modify_my_array(arr);

    println!("Array in Driver Function : {:?}", arr);
    println!("Array after Function Call : {:?}", modify_my_array(arr));
}

fn modify_my_array(mut arr: [i32; 5]) -> [i32; 5] {
    arr[2] = 8;
    arr[3] = 9;
    arr
}
*/

/*
fn main() {
    println!("Factorial of {} is {}\n", 5, factorial(5));
}

fn factorial(n : i32) -> i32 {
    if n == 0 {
        1
    }
    else {
        n * factorial(n - 1)
    }
}
*/

/*
// Fibinacci series

fn main() {
    let mut fibi : [i32; 10] = [0; 10];

    let n = fibinacci_series(&mut fibi, 5);

    //println!("Fibinacci Series: \n\n{:?}", fibi);
    println!("Nth fibinacci number: {}", n);
}

fn fibinacci_series(fibi: &mut[i32; 10], n: i32) -> i32 {

    fibi[0] = 1;

    fibi[0] = 1;
    fibi[1] = 1;

    for i in 2..10 {
        fibi[i] = fibi[i - 1] + fibi[i - 2];
    }

    if n > 0 {

    }

    fibi[4]
}
*/

/*
fn fibi_non_recurssive(n:i32) -> i32 {
    if n == 1 || n == 2 {
        return 1;
    }

    let mut i = 3;
    let mut n1 = 1;
    let mut n2 = 1;
    let mut nth_term = 0;

    while i <= n {
        nth_term = n1 + n2;
        n1 = n2;
        n2 = nth_term;
        i += 1;
    }

    nth_term
}
*/

/*
//[1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
fn fibi_recurssive(term:i32) -> i32 {
/*
    if term == 1 || term == 2 {
        return 1;
    }

    fibi_recurssive(term - 1) + fibi_recurssive(term - 2)
*/

    match term {
        1 => 1,
        2 => 1,
        _ => fibi_recurssive(term - 1) + fibi_recurssive(term - 2)
    }
}

fn main() {
    println!("{}th fibinacci series number: {}\n", 48, fibi_recurssive(48));
}
*/

/*
fn main() {
    let str_literal = "VMD";
    let string_obj = str_literal.to_string();
    println!("String object: {}\n", string_obj);
    println!("String object length: {}\n", string_obj.len());

    let str_obj2 = String::new();
    let str_literal2 = str_obj2.to_string();
    println!("String object: {}\n", str_obj2);
    println!("String object length: {}\n", str_literal2.len());


    let str_obj3 = String::from("VMD");
    println!("String object: {}\n", str_obj3);


    println!("Capacity of the string: {}\n", string_obj.capacity());
}
*/

/*
use std::{thread, time};
use std::io;
use std::io::Write;

fn main() {
    let mut str = String::from("Rust Programming language");
    println!("{}\n", str);

    str = str.replace("language", "Language");
    println!("{}\n", str);

    if str.contains("Rust") {
        str = str.replace("Rust", "The Rust");
    }

    println!("{}\n", str);

    println!("Str Split whitespace");
    for found in str.split_whitespace() {
        println!("{}", found);
    }

    println!("Str Split");
    for found in str.split("P") {
        println!("{}", found);
    }

    println!("Iterating over chars");

    let ten_millis = time::Duration::from_millis(500);
    //let now = time::Instant::now();

    for found in str.chars() {
        print!("{}", found);
        match io::stdout().flush() {
            Ok(_) => print!(""),
            Err(error) => println!("{}", error),
        }
        thread::sleep(ten_millis);
    }
}
*/

/*
fn main() {
    let mut course = String::from("Rus");
    course.push_str("t Programming");

    let course_name = format!("Programming language: {}", course);

    println!("{}", course_name);
}
*/

/*
fn main() {
    let course = "Rust Programming".to_string();

    let slice = &course[0..5];
    println!("Programming language: {}", slice);
}
*/

/*
// using the string literal after the function call

fn main() {
    let course = "Rust programming language";
    change_tile(course);
    println!("Course: {}", course);
}

fn change_tile(course : &str) {
    println!("Course: {}", course);
}
*/

/*
fn main() {
    let course = "Rust Programming Language".to_string();
    foo(course);
    //println!("Course: {}", course);
}

fn foo(course: String) {
    println!("Course: {}", course);
}
*/

/*
fn main() {
    let str = "This is a comprehensive course in Rust programming language on the Educative. \
               Read it with full concentration to grasp the content of the course".to_string();
    //println!("Str: {}", str);

    //test(str);
    println!("{}", test(str));
}

fn test(str : String) -> String {
    let mut concat_string = String::new();

    for found in str.split_whitespace() {
        for character in found.chars() {
            if character == 'c' {
                concat_string.push_str(found);
                concat_string.push(' ');
            }
            break;
        }
    }

    concat_string.pop();
    concat_string
}

*/

// Ownership demonstration

/*
fn main() {
    let str1 = "Rust".to_string();
    //let str2 = str1.clone();

    // Ownership has been moved from `str1` to `str2`
    let str2 = str1;

    //println!("{}", str1); // Gives compilation error
    println!("{}", str2);

    let a = 5;
    let b = a;

    println!("a : {}", a);
    println!("b : {}", b);
}
*/

// Ignore unused variables
// fn main() {
//     let _x = 5;
// }
//

// // Variable bindings
// fn main() {
//     let x = 5;

//     println!("x : {}", x);

//     // using `x` after that line only refers to the second `x`,
//     // the first `x` no longer exists.
//     let x = x + 1;

//     println!("x : {}", x);
// }

// Tuples: Different types. Fixed sized list of values
// fn main() {
//     let some_values = (1, 2, 3, 4, 5, "Vijaya Manohar");
//     println!("{:?}", some_values);
//     println!("{:?}", some_values.0);
//     println!("{:?}", some_values.5);
// }

// Tuples : Broken into individual elements
// fn main() {
//     let (some_char, some_int) = ('a', 32);
//     println!("{:?}", some_char);
//     println!("{:?}", some_int);
// }

// Curly brackets to limit the scope
// fn main() {
//     let x = 5;

//     {
//         let x = 6;
//         println!("{}", x);
//     }

//     println!("x : {}", x);
// }

// Multiple statements inside curly brackets
// fn main() {
//     let x = {
//         let y = 5;
//         y + 6 // this is the *tail* - what the whole block will evaluate to
//     };

//     println!("{}", x);
// }

// If statements are also expressions
// fn main() {
//     let x = {
//         let y = 5;

//         if y > 5 {
//             7
//         } else {
//             8
//         }
//     };

//     println!("{}", x);
// }

// Match is also an expression
// fn lucky_func(feeling_lucky: bool) -> u32 {
//     match feeling_lucky {
//         true => 6,
//         false => 7,
//     }
// }

// fn main() {
//     let x = lucky_func(true);

//     println!("{}", x);
// }

// Returning a tuple
// fn return_tuple() -> (u32, u32) {
//     (1, 3)
// }

// fn main() {
//     let x = return_tuple();
//     println!("{}", x.0);
//     println!("{}", x.1);
// }

// Find the leat number amongst a list of numbers

// use std::cmp::{min, max};

// fn main() {
//     let least_number = min(34434, 5567);
//     let max_number = max(34434, 5567);

//     println!("{}", least_number);
//     println!("{}", max_number);
// }

// Rust vector usage

// fn main() {
//     //let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
//     let mut v = Vec::<u8>::new();
//     v.push(1);
//     v.push(2);
//     v.push(3);
//     v.push(4);

//     println!("{:?}", v);
// }

// Tuple demonstation

// fn main() {
//     let pair: (char, i32) = ('a', 64);
//     let (some_char, some_int) = ('v', 64);

//     println!("{:?}", pair);
//     println!("a.1: {}", pair.1);

//     println!("some_char: {}", some_char);
//     println!("some_int: {}", some_int);

//     println!("Some tuple: {:?}", func_tuple().3);
// }

// fn func_tuple() -> (char, i32, i64, i128, i32) {
//     return ('v', 64, 347385789, 87465876438756, 342);
// }

// Block demonstration
// fn main() {
//     let x = {
//         let x = 10;
//         println!("{}", x);

//         x + 5
//     };

//     println!("{}", x);
// }

//Primitive Data types

// fn main() {
//     println!("Length of string: {}", "Vijaya Manohar Dogiparthi".len());
//     println!("Length of string: {}", str::len("Vijaya Manohar Dogiparthi"));
// }

// struct practice
// fn main() {
//     struct vec2 {
//         x: f32,
//         y: f64,
//         z: f32,
//     };

//     let v1 = vec2 {
//         y: 1.30,
//         z: 1.40,
//         x: 1.20,
//     };

//     let v2 = vec2 {
//         x:1.5,
//         ..v1
//     };

//     println!("v1: {}, {}, {}", v1.x, v1.y, v1.z);
//     println!("v2: {}, {}, {}", v2.x, v2.y, v2.z);
// }

// Playing with structures

// struct Number {
//     odd: bool,
//     value1: i32,
//     value2: i32,
// }

// fn main() {
//     let one = Number {
//         odd: true,
//         value1: 1,
//         value2: 2,
//     };
//     let two = Number {
//         odd: false,
//         value1: 3,
//         value2: 4,
//     };

//     print_number(one);
//     print_number(two);
// }

// fn print_number(n: Number) {
//     if let Number { odd, value: 2 } = n {
//         println!("Odd number: {}", odd);
//     } else if let Number { odd: false, value } = n {
//         println!("Even number: {}", value);
//     } else {
//         println!("{}", n.odd);
//     }
// }

// fn print_number(n: Number) {
//     match n {
//         Number {
//             odd: true,
//             value1,
//             value2,
//         } => println!("Odd number: {}, {}", value1, value2),
//         Number {
//             odd: false,
//             value1,
//             value2,
//         } => println!("Even number: {}, {}", value1, value2),
//     }
// }

// Todo: efdsf
// FIXME:dfgg

// extern crate memmap;
// use memmap::MmapMut;
// use std::fs::OpenOptions;
// use tonic::codegen::ok;

// use std::error::Error;
// use std::fmt::{Display, Formatter, Result as FmtResult};

// struct SlowDacManager {
//     verbosity_level: bool,
// }

// #[derive(Debug)]
// enum DacErrorCode {
//     InvalidInput = 20,
//     OutOfMemory = 21,
//     FileNotFound = 22,
//     MemoryMapError = 23,
//     InvalidError = 40,
// }

// #[derive(Debug)]
// struct DacError {
//     message: String,
//     error_code: DacErrorCode,
// }

// impl DacError {
//     fn new(message: &str, error_code: DacErrorCode) -> DacError {
//         DacError {
//             message: message.to_string(),
//             error_code: error_code,
//         }
//     }
// }

// impl Error for DacError {}

// impl Display for DacError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         write!(f, "DAC Error: {}", self.message)
//     }
// }

// fn debug_log(dac_mgr: &SlowDacManager, log_str: &String) {
//     if dac_mgr.verbosity_level == true {
//         println!("{}", log_str);
//     }
// }

// fn map_physical_mem_rw(
//     phy_addr: usize,
//     mmap_len: usize,
// ) -> Result<MmapMut, Box<dyn std::error::Error>> {
//     let fd = OpenOptions::new()
//         .read(true)
//         .write(true)
//         //.custom_flags(O_DIRECT | O_SYNC)
//         .open("/dev/mem")?;

//     let m = unsafe {
//         memmap::MmapOptions::new()
//             .offset(phy_addr as u64)
//             .len(mmap_len)
//             .map_mut(&fd)?
//     };

//     Ok(m)
// }

// fn write_to_qpc(phy_addr: usize, val_to_write: u32) -> Result<u32, Box<dyn std::error::Error>> {
//     let m = match map_physical_mem_rw(phy_addr, std::mem::size_of::<u32>()) {
//         Ok(m) => m,
//         Err(_err) => {
//             return Err(Box::new(DacError::new(
//                 "Error in mapping the device memory",
//                 DacErrorCode::MemoryMapError,
//             )));
//         }
//     };

//     let p = m.as_ptr() as *mut u32;
//     unsafe {
//         std::ptr::write_volatile(p, val_to_write);
//     }

//     Ok(0)
// }

// fn main() -> Result<u32, Box<dyn std::error::Error>> {
//     // -> Result<u32, Box<dyn std::error::Error>> {
//     let rc = write_to_qpc(0x1, 5);

//     // TODO:Check if the rc has an error code
//     // Fixme: Check if the rc has an error code

//     // match rc {
//     //     Ok(_) => {}
//     //     Err(error) => {
//     //         println!("Error: {:?}", error);
//     //     }
//     // }

//     if let Err(error) = rc {
//         println!("Error: {}", error);
//     } else {
//         let value = rc?; //rc.unwrap();
//         println!("Value: {}", value);
//     }

//     Ok(0)
// }

// fn main() {
//     let x = "Out the block";
//     {
//         let x = "inside the block";
//         println!("x : {}", x);
//     }

//     println!("x : {}", x);
// }

// Inside a block, there can be multiple statements:
// fn main() {
//     let x = {
//         let a = 5;
//         let b = 10;
//         a + b
//     };

//     println!("x : {}", x);
// }

// Match expression usage
// fn foo(x: bool) -> u32 {
//     match x {
//         true => 6,
//         false => 7,
//     }
// }

// fn main() {
//     println!("foo(true) : {}", foo(true));
// }

// Declaring methods on the types

// struct Number {
//     odd: bool,
//     value: u32,
// }

// impl Number {
//     fn is_strictly_positive(&self) -> bool {
//         self.value > 0
//     }
// }

// fn main() {
//     //Interior values of the variable n are immutable by default
//     let n = Number {
//         odd: false,
//         value: 32,
//     };

//     if n.is_strictly_positive() == true {
//         println!("n: {}", n.value);
//     }
// }

// Code to debug

// static A: i32 = 5;

// struct MyStruct {
//     prop: usize,
// }

// struct Point(f32, f32);

// fn main() {
//     let a = 42;
//     let b = vec![0, 0, 0, 100];
//     let c = [1, 2, 3, 4, 5];
//     let d = 0x5ff;
//     let e = MyStruct { prop: 10 };
//     let p = Point(3.14, 3.14);

//     println!("Hello, world!");
// }

// Dots are typically used to access fields of a value.
// fn main() {
//     let a = (1, 2);
//     println!("a: {}", a.0)
// }

// Use scope to bring in names from other namespaces.

// use std::cmp::min;

// fn main() {
//     println!("min of two numbers: {}", min(43535, 54646));
// }

// Types are namespaces too

// fn main() {
//     let len = "adeu amigos".len();

//     println!("len: {}", len);

//     let len = str::len("adeu amigos");
//     println!("len: {}", len);
// }

// Creation of a new vector

// fn main() {
//     let v1: Vec<i32> = Vec::new(); //Vec is in the scope by default
//     println!("{:?}", v1);

//     let v2: Vec<u32> = std::vec::Vec::new();
//     println!("{:?}", v2);
// }

// Struct usage and struct variables
// struct FloatVector {
//     a: f64,
//     b: f64,
// }

// fn main() {
//     let a = FloatVector {
//         a: 41.343,
//         b: 42.343,
//     };

//     println!("a: {:?}\tb: {:?}", a.a, a.b);

//     let b = FloatVector { ..a };

//     print_if_same(a, b);
// }

// fn print_if_same(a: FloatVector, b: FloatVector) {
//     if let FloatVector
// }

// struct Employee {
//     name: String,
//     id: u32,
// }

// impl Employee {
//     pub fn print_emp_details(&self) {
//         println!("Employee name: {:?}", self.name);
//         println!("Employee ID : {:?}", self.id);
//     }
// }

// fn main() {
//     let emp1 = Employee {
//         name: "Vijay".to_string(),
//         id: 0,
//     };

//     let emp2 = Employee {
//         name: "VMD".to_string(),
//         id: 1,
//     };

//     emp1.print_emp_details();

//     emp1.print_emp_details();

//     emp2.print_emp_details();
// }

// Implementing certain method on a primitive type

// trait ModuloNum {
//     fn make_positive(self) -> i32;
// }

// impl ModuloNum for i32 {
//     fn make_positive(self) -> i32 {
//         self.abs()
//     }
// }

// impl ModuloNum for f64 {
//     fn make_positive(self) -> i32 {
//         self.abs() as i32
//     }
// }

// fn main() {
//     let i: i32 = -123;

//     println!(
//         "Making the fixed point number : {} positive. Now it is: {}",
//         i,
//         i.make_positive()
//     );

//     let i: f64 = -435345.3434;
//     println!(
//         "Making the floating point number : {} positive. Now it is: {}",
//         i,
//         i.make_positive()
//     );
// }

// Borrow demonstration

// #[derive(Debug)]
// struct Number {
//     odd: bool,
//     value: i32,
// }

// fn main() {
//     let n = 34;

//     printn(n);
//     printn(n - 1);

//     let mut n = Number {
//         odd: true,
//         value: 1,
//     };

//     print_number(&n);
//     print_number(&Number {
//         odd: false,
//         value: n.value - 1,
//     });

//     print_number(&Number {
//         odd: false,
//         value: n.value + 34535,
//     });
//     invert_number(&mut n);
//     print_number(&n);
// }

// fn printn(n: i32) {
//     println!("n: {}", n);
// }

// //`n` is borrowed for the time of the call
// fn print_number(n: &Number) {
//     println!("Value: {}", n.value);
//     println!("Value: {:?}", n);
// }

// fn invert_number(n: &mut Number) {
//     n.value = -n.value;
// }

// Compute the area of Rectangle
// #![allow(dead_code)]

// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// #[derive(Debug)]
// struct Rectangle {
//     top_left: Point,
//     bottom_right: Point,
// }

// fn main() {
//     let rect = Rectangle {
//         top_left: Point { x: 10, y: 10 },
//         bottom_right: Point { x: 20, y: 0 },
//     };

//     println!("Area of the rectangle: {}", compute_rectangle_area(&rect));
// }

// fn compute_rectangle_area(rect: &Rectangle) -> i32 {
//     let height = rect.top_left.y - rect.bottom_right.y;
//     let width = rect.bottom_right.x - rect.top_left.x;

//     height * width
// }

// Usage of Tuples

// #[derive(Debug)]
// struct Pair(i32, f32);

// fn main() {
//     let pair = Pair(1, 2.0);

//     let Pair(x, y) = pair;
//     println!("x : {}, y : {}", x, y);
// }

// fn main() {
//     let person_name1 = "Vijay";
//     let mut person_name2 = person_name1;

//     println!("Person name:{}", person_name1);
//     println!("Person name:{}", person_name2);

//     person_name2 = "Vijay Manohar";

//     println!("Person name:{}", person_name1);
//     println!("Person name:{}", person_name2);
// }

// Procedural Macro

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

fn main() {
    let person = Person {
        name: "Vijay Manohar".to_string(),
        age: 40,
    };

    println!("Person: {:?}", person);
    println!("Person: {:#?}", person);
}
