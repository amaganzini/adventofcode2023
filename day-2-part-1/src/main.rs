use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const FILENAME: &str = "input";

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

const RED: &str = "red";
const GREEN: &str = "green";
const BLUE: &str = "blue";


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
//and validate it
fn validate_reveal(reveal: String) -> bool{

    for num_color_str in reveal.split(", "){

        let num_color_pair = num_color_str.split(" ").collect::<Vec<&str>>();

        let num: u32 = match num_color_pair[0].parse(){
            Err(why) => panic!("Failed to parse number of colors: {}", why),
            Ok(parse_val) => parse_val
        };

        let color = num_color_pair[1];
        let color_max = match color{
            RED => MAX_RED,
            BLUE => MAX_BLUE,
            GREEN => MAX_GREEN,
            _ => panic!("Failed to match color.")
        };

        if num > color_max{
            println!("invalid game! {} {}", num, color);
            return false;
        }
    }

    return true;
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

    println!("game number: {}", game_num);

    //let reveals = game_split[1].split(";").collect::<Vec<&str>>();
    
    for reveal in game_split[1].split(";"){
        if validate_reveal(reveal.get(1..).unwrap().to_string()){
            continue;
        }
        else{
            //one of the reveals was not valid, fail game
            return 0;
        }
    }

    return game_num;
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
