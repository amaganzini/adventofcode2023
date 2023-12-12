use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;

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


//parses the Card number portion (just used for debug prints)
fn parse_cardno(cardstr: &str) -> usize{

    let splits = cardstr.split(" ").collect::<Vec<&str>>();

    for split in &splits[1..]{
        //handle empty splits from multiple spaces
        if split.len() == 0{
            continue;
        }
        else{
            return split.parse().unwrap();
        }
    }

    //should error, but this "shouldn't happen"
    println!("Failed to parse card string, returning 0");
    return 0;
}


//parses a string for a vector of numbers separated by spaces
fn parse_numbers(numberstr: &str) -> Vec<usize>{

    let splits = numberstr.split(" ").collect::<Vec<&str>>();
    let mut numbers: Vec<usize> = vec![];

    for split in splits{
        //handle empty splits
        match split.parse() {
            Ok(number) => numbers.push(number),
            _ => continue,
        };
    }

    return numbers;
}

//parses a scratchcard info line
//returns card number, vector of winning numbers, vector of playing numbers
fn parse_scratchcard_line(line: &str) -> (usize, Vec<usize>, Vec<usize>){

    let cardno_split = line.split(":").collect::<Vec<&str>>();
    let cardno = parse_cardno(cardno_split[0]);

    let winners_split = cardno_split[1].split("|").collect::<Vec<&str>>();

    let winners: Vec<usize> = parse_numbers(winners_split[0]);
    let checks: Vec<usize> = parse_numbers(winners_split[1]);

    return (cardno, winners, checks);
}

//Solves some input puzzle :)
fn check_game_input(input: String) -> usize{

    let mut game_results: usize = 0;

    for line in input.lines(){
        //get each part of the line
        let(cardno, winners, checks) = parse_scratchcard_line(line);

        print!("Card {}: ", cardno);

        let winners_set: HashSet<_> = winners.into_iter().collect();
        let checks_set: HashSet<_> = checks.into_iter().collect();

        //we just want a HashSet, not an Intersection type
        let matches: usize = winners_set.intersection(&checks_set)
                                        .cloned()
                                        .collect::<HashSet<_>>()
                                        .len();

        if matches > 0{
            let score = 1 << (matches - 1);
            game_results += score;
            println!("{}", score);
        }
        else{
            println!("No winning numbers");
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
