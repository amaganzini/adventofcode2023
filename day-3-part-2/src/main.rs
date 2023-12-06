use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILENAME: &str = "input";

const LINE_OFFSET: usize = 141;

//reads an input file as a String
fn read_input_file(filename: String) -> String{
    
    let path = Path::new(&filename);

    let mut file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}",
                           path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();

    match file.read_to_string(&mut s){
        Err(why) => panic!("couldn't read {}: {}",
                           FILENAME.to_string(), why),
        Ok(_) => print!("Successfully read {}\n",
                        FILENAME.to_string())
    }

    return s;
}


//is_numeric() for u8
fn is_num(input: u8) -> bool{
    return input >= b'0' && input <= b'9';
}

//parses a str slice to the left and right for a number
//trying out Result return
fn parse_number(input: &[u8], pos_check: isize) -> Result<usize, &'static str>{

    let mut num_str = String::new();

    //bounds check before any indexing
    if pos_check < 0 || pos_check as usize > input.len(){
        return Err("Out of bounds");
    }

    let pos = pos_check as usize;
    
    //check if a number exists at all
    if !is_num(input[pos]){
        return Err("No number");
    }

    let mut i: usize = pos;

    //look to the left
    while is_num(input[i]){
        num_str.push(input[i] as char);

        if i.checked_sub(1) == None{
            //we hit the start of our input
            break;
        }
        i -= 1;
    }

    //unreverse our reversed number string
    num_str = num_str.chars().rev().collect::<String>();

    i = pos + 1;

    //look to the right
    while is_num(input[i]){
        num_str.push(input[i] as char);

        i += 1;
        if i >= input.len(){
            //we hit end of input
            break;
        }
    }
    
    let returnval = num_str.parse().unwrap();

    Ok(returnval)
}


//calculates the gear ratio for a bonding box
//full of matches because I wanted to return Result from parse_number...
fn calc_bounding_box_ratio(input: &[u8], pos: usize) -> usize{

    let check_pos: isize = pos as isize;
    let mut num_adjacent = 0;

    //we're multiplying, so start with the identity
    let mut ratio = 1;

    //check sides first
    for side in [check_pos - 1, check_pos + 1]{
        match parse_number(input, side){
            Ok(val) => {
                ratio *= val;
                num_adjacent += 1;
            }
            _ => {}
        };
    }

    let middle_row = [check_pos - 1, check_pos, check_pos + 1];
    let top: Vec<_> = middle_row.iter().map(|&x| x - LINE_OFFSET as isize).collect();
    let bot: Vec<_> = middle_row.iter().map(|&x| x + LINE_OFFSET as isize).collect();

    //check top and bottom rows
    //for each row, we will check middle
    //OR parse corners
    for row in [top, bot]{

        let mut middle_check_val = 0;

        //check middle bounds and check middle
        match parse_number(input, row[1]){
            Ok(val) => {
                middle_check_val = val;
                ratio *= val;
                num_adjacent += 1;
            }
            _ => {}
        };

        //check_ratio = 1, either middle dne, or middle = 1
        //if middle parsed = 1, then corners are empty
        if middle_check_val < 1{
            for corner in [row[0], row[2]]{
                match parse_number(input, corner){
                    Ok(val) => {
                        ratio *= val;
                        num_adjacent += 1;
                    }
                    _ => {}
                };
            }
        }
    }

    if num_adjacent >= 2{
        return ratio;
    }
    return 0;
}


//Solves some input puzzle :)
fn check_game_input(input: String) -> usize{

    let mut game_results: usize = 0;

    //convert to u8 array so we can index (much faster than String)
    let str_bytes = input.as_bytes();

    for i in 0..input.len(){
        if str_bytes[i] == b'*'{
            println!("Found {} at position {}, line {} col {}",
                   '*', i, i / LINE_OFFSET, i % LINE_OFFSET);

            //We found a gear! Find the ratio
            let ratio = calc_bounding_box_ratio(&str_bytes, i);
                            //create_bounding_box_for_char(input.len(), i));

            if ratio > 1{
                println!("\tvalid! with ratio {}", ratio);
                game_results += ratio;
            }
        }
    }

    return game_results;
}


//main entry
fn main() {

    let input = read_input_file(FILENAME.to_string());

    let game_sum = check_game_input(input);

    println!("game result: {}", game_sum);
}
