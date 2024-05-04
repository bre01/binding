use std::os::raw::c_int;    // 32 bits
use std::os::raw::c_double; // 64 bits

// Import three functions from the standard library libc.
// Here are the Rust declarations for the C functions:
extern "C" {
    fn abs(num: c_int) -> c_int;
    fn sqrt(num: c_double) -> c_double;
    fn pow(num: c_double, power: c_double) -> c_double;
}

#[link(name = "my_library")]
extern "C" {
    fn my_library_function(x: i32) -> i32;
}




fn main() {
    let x: i32 = -123;
    println!("this is my own function returned value {}\n",unsafe {
        my_library_function(x)
    });

    println!("\nAbsolute value of {x}: {}.",
	     unsafe { abs(x) });

    let n: f64 = 9.0;
    let p: f64 = 3.0;
    println!("\n{n} raised to {p}: {}.",
	     unsafe { pow(n, p) });

    let mut y: f64 = 64.0;
    println!("\nSquare root of {y}: {}.",
	     unsafe { sqrt(y) });
    y = -3.14;
    println!("\nSquare root of {y}: {}.",
	     unsafe { sqrt(y) }); //** NaN = NotaNumber
}

