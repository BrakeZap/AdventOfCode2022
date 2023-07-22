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
            'X' => {total+=1;
                    myNum = 0;},
            'Y' => {total+=2;
                    myNum = 1},
            'Z' => {total+=3;
                    myNum = 2},
            _ => total+=0
        }

        match opponentChar {
            'A' => oppNum = 0,
            'B' => oppNum = 1,
            'C' => oppNum = 2,
            _ => oppNum = 0
        }
        
        let resultArr = [["tie", "me", "opp"],
                                        ["opp", "tie", "me"],
                                        ["me", "opp", "tie"]];
        
        match resultArr[oppNum][myNum] {
            "tie" => total+=3,
            "me" => total+=6,
            _ => total+=0
        }



    }

    print!("Total is: {}",total)
}
