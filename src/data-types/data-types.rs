fn main() {
    let example_string = "Hello, World!"; // string
    let integer_value = 4; // int
    let rating_float = 4.5; // float
    let is_boolean = true; // boolean
    let icon_char = '♥'; //unicode character
}

// if the stored value in a variable exceeds the memory, Rust restart the count overflow

//INTEGER
// i32 by default
fn integer_type_unsigned() {
    let u8_integer: u8 = 8; // 0 -255
    let u16_integer: u16 = 16; // 0 - 65535
    let u32_integer: u32 = 32; // 0 - 4294967295
    let u64_integer: u64 = 64; // 0 - 18446744073709551615
    let u128_integer: u128 = 128; // 0 - 340282366920938463463374607431768211455
    let usize_integer: usize = 256; // 0 - 18446744073709551615
}

fn integer_type_signed() {
    let i8_integer: i8 = 8; // (-128) - 127
    let i16_integer: i16 = 16; // (-32768) - 32767
    let i32_integer: i32 = 32; // (-2147483648) - 2147483647
    let i64_integer: i64 = 64; // (-9223372036854775808) - 9223372036854775807
    let i128_integer: i128 = 128; // (-170141183460469231731687303715884105728) - 170141183460469231731687303715884105727
    let isize_integer: isize = 256; // (-9223372036854775808) - 9223372036854775807
}

//FLOAT
//f64 by default
fn float_type() {
    let f32_float: f32 = 3.14;
    let f64_float: f64 = 15000.600;
}

//BOOLEAN
fn boolean_type() {
    let isfun: bool = true;
}


//CHARACTER
fn character_type() {
   
    let special_character = '@'; //default
    let alphabet:char = 'A';
    let emoji:char = '😁';
}