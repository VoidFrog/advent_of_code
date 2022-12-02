use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut max_carried_calories:i32 = 0;
    let mut current_calories:i32 = 0;

    let output = read_lines();
    for line in output {
        // println!("{}", line);
        parse_line(line, &mut max_carried_calories, &mut current_calories);

    }
}

fn parse_line(line:String, max: &mut i32, current: &mut i32){
    // println!("{}", line.len())
    if line.len() == 0  {
        // println!("-->space<--");
        if current > max { println!("{}", current) }
        if max < current { *max = *current }
        *current = 0;
    }
    else {
        let number:i32 = line.parse().expect("something went wrong");
        *current += number;
    }

}

fn read_lines() -> Vec<String>{
    let file = File::open("elf_food_list.txt")
        .expect("file does not exist");
    let reader = BufReader::new(file);              //returns BufReader             

    return reader.lines()                           //returns iterator over lines of the file 
            .map(|line| line.expect("parsing"))     //transforms the iterator value, into another one (in this case it checks if there is no `panic!`)    
            .collect();                             //collects all of the iterator's values and groups them into a collection
}