use std::fs;
use std::str::FromStr;
use std::str;

fn main() {
    let input_file_location: String = String::from("C:\\Development\\aoc2023\\task2\\input.txt");
    let lines: Vec<String> = file_to_vector(input_file_location);
    let results: Vec<GameResult> = lines.iter().map(|x| x.parse::<GameResult>().unwrap()).collect();

    //Magic values given to me as part of the task.
    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    //To count the number of games that are within the parameters given in the task.
    let mut running_total = 0;
    let mut power_total = 0;

    for result in results {

        power_total += result.highest_red * result.highest_green * result.highest_blue;

        if result.highest_red > max_red || result.highest_green > max_green || result.highest_blue > max_blue
        {
            continue;
        }

        running_total += result.game_number;
    }

    println!("Sum of games where the results were possible: {:?}", running_total);
    println!("Sum of cube powers: {:?}", power_total);

}

#[derive(Debug)]
struct GameResult {
    game_number: u32,
    highest_red: u32,
    highest_green: u32,
    highest_blue: u32
}

#[derive(Debug)]
struct ParseGameResultError;

/* 
Implementing the FromStr trait for GameResult lets us call .parse::<GameResult>() on a string to extract a GameResult object.
I wish I could say I built this outright, but actually I worked out most of the code in a hacky method and then tidied up.
*/
impl FromStr for GameResult {
    type Err = ParseGameResultError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game_number: u32 = 0;
        let mut highest_red: u32 = 0;
        let mut highest_green: u32 = 0;
        let mut highest_blue: u32 = 0;

        //Split on whitespace to produce a string that is token-rich with a number of delimiters.
        let no_whitespace: Vec<&str> = s.split(' ').collect();
        let no_whitespace = no_whitespace.concat();

        //Split on the semicolon which gives us a string of "GameXXX" where XXX is the digit representing the game ID.
        let temp: Vec<&str> = no_whitespace.split(':').collect();
        //Game is 4 characters long, so get the slice from pos 4 onwards for the game ID.
        let game_value = &temp[0][4..];
        //Store game number as we need this as one of the fields of our GameResult struct.
        game_number = game_value.parse::<u32>().unwrap();

        let remaining_string = &temp[1];

        //Splitting on semicolon gives us each game round as its own string.
        let game_results: Vec<&str> = remaining_string.split(';').collect();

        for individual_results in game_results {
            for items in individual_results.split(',').enumerate() {
                let mut text_starts_pos: usize = 0;
                for (pos, char) in items.1.chars().enumerate() {
                    match char.to_digit(10) {
                        Some(_value) => { continue; },
                        None => {
                            text_starts_pos = pos;
                            break;
                        }
                    }
                }

                let number_of_cubes: &u32 = &items.1[..text_starts_pos].parse().unwrap();
                let colour_of_cube: &str = &items.1[text_starts_pos..];

                println!("Number of cubes: {:?}", number_of_cubes);
                println!("Colour of cubes: {:?}", colour_of_cube);

                match colour_of_cube {
                    "red" => { if number_of_cubes > &highest_red { highest_red = *number_of_cubes } },
                    "green" => { if number_of_cubes > &highest_green { highest_green = *number_of_cubes } },
                    "blue" => { if number_of_cubes > &highest_blue { highest_blue = *number_of_cubes } }
                    _ => { return Err(ParseGameResultError) }
                }
            }
        }

        Ok(GameResult::new(game_number, highest_red, highest_green, highest_blue))
    }
}
 
impl GameResult {

    fn new(game_number: u32, highest_red: u32, highest_green: u32, highest_blue: u32) -> Self {
        Self {
            game_number,
            highest_red,
            highest_green,
            highest_blue,
        }
    }
}

fn file_to_vector(file_path: String) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut line_vec: Vec<String> = Vec::new();
    for line in contents.lines() {
        line_vec.push(line.to_string());
    }
   line_vec
}