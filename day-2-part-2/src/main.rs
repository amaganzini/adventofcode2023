use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::cmp::max;

const FILENAME: &str = "input";

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";

//Opens and reads the input file, returns String with contents
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


//We take a string in the form of 'x color1, y color2, ...'
//returns "power" of game (product of min colors possible)
fn validate_reveal(reveal: String) -> (u32, u32, u32){

    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    for num_color_str in reveal.split(", "){

        let num_color_pair = num_color_str.split(" ").collect::<Vec<&str>>();

        let num: u32 = match num_color_pair[0].parse(){
            Err(why) => panic!("Failed to parse number of colors: {}", why),
            Ok(parse_val) => parse_val
        };

        let color = num_color_pair[1];

        match color{
            RED => max_red = max(num, max_red),
            GREEN => max_green = max(num, max_green),
            BLUE => max_blue = max(num, max_blue),
            _ => panic!("Failed to match color.")
        };
    }

    return (max_red, max_green, max_blue);
}


//returns game number on successful game series, 0 if not
fn process_cube_game(game: String) -> u32{

    let game_split = game
                     .split(":")
                     .collect::<Vec<&str>>();

    let game_num: u32 = match game_split[0]
               .get(5..)
               .unwrap()
               .parse(){
        Err(why) => {
            println!("Failed to parse a Game entry: {}", why);
            return 0
        },
        Ok(parse_val) => parse_val
    };


    let mut min_red: u32 = 0;
    let mut min_green: u32 = 0;
    let mut min_blue: u32 = 0;
    
    for reveal in game_split[1].split(";"){
        let min_tuple = validate_reveal(reveal.get(1..).unwrap().to_string());
        min_red = max(min_tuple.0, min_red);
        min_green = max(min_tuple.1, min_green);
        min_blue = max(min_tuple.2, min_blue);
    }

    println!("game number: {}, red: {}, green: {}, blue: {}, prod: {}",
             game_num, min_red, min_green, min_blue,
             min_red * min_green * min_blue);
    return min_red * min_green * min_blue;
}


fn check_game_input(input: String) -> u32{

    let mut game_results: u32 = 0;
    let mut game_val: u32;

    for line in input.lines(){
        game_val = process_cube_game(line.to_string());
        game_results += game_val;

        if game_val > 0{
            println!("+ {}, total = {}", game_val, game_results);
        }
        else
        {
            println!("not increasing total");
        }
    }

    return game_results;
}


fn main() {

    let input = read_input_file(FILENAME.to_string());

    let game_sum = check_game_input(input);

    println!("game result: {}", game_sum);

    
}
