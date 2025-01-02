// let introduces a variable binding
let x;
x = 42;

// or

let x = 42;

//specify the type (i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, char, bool, str, arrays, tuples, slices, etc.)

let x: i32 = 42;

// _ is a special name for "lack of name"
let _ = 42: //does nothing because 42 is a constant

let _ = get_thing() // calls get_thing but throws away its result

//shadow bindings:
let x = 13;
let x = x + 1; //now, x only refers to the second x (this one), you can no longer reference the first

//tubples:
let pair = ('x', 42);

pair.0; //thi is 'x'
pair.1; //this is 42

//with type annotation:

let pair: (char, i32) = ('x', 42);

//destructuring tuples:


//statements can span multiple lines

let y = vec![1,2,3,4,5,6]
    .iter()
    .map({|x| x * x})
    .fold(0, {|acc, x| acc + x});


// functions

fn greet() {
    println!("wassssup");3
}

//blocks

pub fn in_out() {
    let x = "out";
    {
        let x = "in";
        println!("{}", x);
    }
    println!("{}", x);
}