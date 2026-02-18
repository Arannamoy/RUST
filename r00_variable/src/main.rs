fn main() {
    let a=16;   // immutable variable. can not change variable  
    println!("{}",a);
    let mut _a=17; // mutable variable. can change value
    println!("{}",_a);
    _a=18;
    println!("{}",_a);
    const INT_32:i32=100; // const declaration syntax. const  ALL_CAPITAL_LETTER: 
    const INT_64:i64=1000;
    const FLOAT_32:f32=10000.000;
    const FLOAT_64:f64=10000.000;
    println!("{}{}{}{}",INT_32,INT_64,FLOAT_32,FLOAT_64 );
    println!("Hello, world!");
}
