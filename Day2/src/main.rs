use std::env;
use std::fs;


fn main() { 
    let contents = fs::read_to_string("input.txt").expect("File could not be opened!");
    let strList: Vec<&str> = contents.split("\n").collect();
    let mut total = 0;
    for input in strList {
        let mut chars = input.chars();
        let opponentChar = chars.next().unwrap();
        let myChar = chars.next_back().unwrap();
        let mut oppNum = 0;
        let mut myNum = 0;
        match myChar {
            'X' => {total+=0;
                    myNum = 0;},
            'Y' => {total+=3;
                    myNum = 1},
            'Z' => {total+=6;
                    myNum = 2},
            _ => total+=0
        }

        match opponentChar {
            'A' => oppNum = 0,
            'B' => oppNum = 1,
            'C' => oppNum = 2,
            _ => oppNum = 0
        }

        let resultArr = [[1, 0, 2],
                                        [2, 1, 0],
                                        [0, 2, 1]];
        
        if myChar == 'Y' {
            myNum = oppNum;
        } else if myChar == 'Z'{
            myNum = resultArr[oppNum].iter().position(|&i| i == 0).unwrap_or(0);
        }else{
            myNum = resultArr[oppNum].iter().position(|&i| i == 2).unwrap_or(0);
        }

        match myNum {
            0 => total+=1,
            1 => total+=2,
            2 => total+=3,
            _ => total+=0
        }

    }

    print!("Total is: {}",total)
}
