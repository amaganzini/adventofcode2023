use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILENAME: &str = "input";


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


//parses a str slice for the first contiguous number
//returns length of characters used to make string
//(val, length)
fn parse_number(input: &str) -> (usize, usize){

    let mut num_str = String::new();

    for c in input.chars(){
        if c.is_numeric(){
            num_str.push(c);
        }
        else{
            break;
        }
    }

    if num_str.len() <= 0{
        return (0, 0);
    }
    
    return(num_str.parse().unwrap(), num_str.len());
}


//checks a bounding box for a symbol
fn check_bounding_box(input: &[u8], bounding_box: Vec<usize>) -> bool{

    const DOT: u8 = b'.';
    const NL: u8 = b'\n';

    for i in bounding_box{
        let check = input[i];

        if (check < b'0' || check > b'9') && check != DOT && check != NL{
            return true;
        }
    }

    return false;
}

//checks the bounding box around some position
fn create_bounding_box(input_len: usize, pos: usize, length: usize) -> Vec<usize>{

    const LINE_OFFSET: usize = 141;

    let mut bounding_box = Vec::<usize>::new();

    // .XXXXX
    // .X123.
    // everything left and above could be negative
    let above_start: isize = pos as isize - LINE_OFFSET as isize - 1;
    let above_end: isize = above_start + length as isize + 1;
    let left: isize = pos as isize - 1;

    for i in above_start..above_end+1{
        if i >= 0{
            bounding_box.push(i as usize);
        }
    }
    if left >= 0{
        bounding_box.push(left as usize);
    }

    // ..123X
    // .XXXXX
    // everything to the right and below could be oob
    let bot_start = pos + LINE_OFFSET - 1;
    let bot_end = bot_start + length + 1;
    let right = pos + length;

    for i in bot_start..bot_end+1{
        if i < input_len{
            bounding_box.push(i);
        }
    }
    if right < input_len{
        bounding_box.push(right);
    }

    return bounding_box;
}


//Solves some input puzzle :)
fn check_game_input(input: String) -> usize{

    let mut game_results: usize = 0;

    //convert to u8 array so we can index (much faster than String)
    let str_bytes = input.as_bytes();

    let mut i = 0;

    while i < input.len(){
        let (val, len) = parse_number(&input[i..]);
        if len > 0{
            print!("Found {}", val);

            //We found a number, create and check bounds
            if check_bounding_box(&str_bytes,
                create_bounding_box(input.len(), i, len)){

                    print!(", valid!");
                    game_results += val;
            }
            println!();

            i += len;
        }
        else{
            i += 1;
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
