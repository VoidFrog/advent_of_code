use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let mut max_carried_calories:[i32; 3] = [0; 3];
    let mut current_calories:i32 = 0;
    let mut sum_of_calories:i32 = 0;

    let output = read_lines();
    for line in output {
        parse_line(line, &mut max_carried_calories, &mut current_calories);
    }

    for item in max_carried_calories {
        println!("{}", item);
        sum_of_calories += item;
    }

    println!("{}", sum_of_calories);
}

fn parse_line(line:String, max: &mut [i32], current: &mut i32){
    // println!("{}", line.len())
    if line.len() == 0  {
        // println!("-->space<--");
        let mut min_value = *max.iter().min().unwrap();                          //getting minimum value of the max array 
        if current > &mut min_value {
            let index = max.iter().position(|&value| value==min_value).unwrap(); //getting index of min value
            max[index] = *current;
        }   
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