use std::env;
use std::fs;
use std::ops::Index;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let contArr : Vec<&str>= contents.split("\n\n").collect();
    let mut finalArr: Vec<i32> = Vec::new();

    for str in contArr{
        let temp:Vec<&str> = str.split("\n").collect();
        let mut total = 0;
        for tStr in temp {
            total += tStr.parse::<i32>().unwrap_or(0);
        }
        finalArr.push(total);
    }
    if finalArr.is_empty() {
        println!("Vector empty!");
        return;
    }



    finalArr.sort();
    let mut newTotal = 0;
    for i in 0..3 {
        newTotal += finalArr.pop().unwrap();
    }


    // match finalArr.iter().max() {
    //     Some(max) => println!("Max is {}", max),
    //     None => println!("Vector is empty"),
    // }

    println!("Total is: {}", newTotal);
}
