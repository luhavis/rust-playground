use std::io::{self, Read};

fn main() {
    let x: f32 = 2.2;
    println!("{}", x);

    let y: f32 = 3.0;
    println!("{}", y);

    let sum = 5 + 10;
    println!("{}", sum);

    let difference = 95.5 - 4.3;
    println!("{}", difference);

    let product = 4 * 30;
    println!("{}", product);

    let quotient = 56.7 / 32.2;
    println!("{}", quotient);

    let runcated = -5 / 3;
    println!("{}", runcated);

    let remainder = 43 % 5;
    println!("{}", remainder);


    let t = true;
    println!("{}", t);

    let f: bool = false;
    println!("{}", f);


    let c = 'z';
    println!("{}", c);

    let z: char = 'ℤ';
    println!("{}", z);

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    let tupple: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tupple;

    println!("{}, {}, {}", x, y, z);

    let five_hundred = tupple.0;
    let six_point_four = tupple.1;
    let one = tupple.2;

    println!("{}, {}, {}", five_hundred, six_point_four, one);


    let arr = [1, 2, 3, 4, 5];
    
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];

    let arr_first = arr[0];
    let arr_second = arr[1];
    println!("arr_first: {}", arr_first);
    println!("arr_second: {}", arr_second);

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("index : {}, element: {}", index, element);

}
