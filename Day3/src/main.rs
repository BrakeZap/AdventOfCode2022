use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't open file!");
    let inventory: Vec<&str> = contents.split("\n").collect();
    let mut total = 0;

    for i in (0..inventory.len()).step_by(6){
        let group1 = &inventory[i..i+3];
        let group2 = &inventory[i+3..i+6];


        for c in group1[0].chars(){
            if (group1[1].chars().filter(|b| b == &c).count() > 0 && group1[2].chars().filter(|b| b == &c).count() > 0){
                if (!c.is_uppercase()){
                        total += (c as i32) - 96;
                }else{
                        total += (c as i32) - 38;
                }
                break;
            }
        }

        for c in group2[0].chars(){
            if (group2[1].chars().filter(|b| b == &c).count() > 0 && group2[2].chars().filter(|b| b == &c).count() > 0){
                if (!c.is_uppercase()){
                        total += (c as i32) - 96;
                }else{
                        total += (c as i32) - 38;
                }
                break;
            }
        }
    }
    
    
    print!("{}", total);

}
