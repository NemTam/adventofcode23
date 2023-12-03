use fancy_regex::Regex;
use std::fs;

const RE_RED: &str = r"(\d+)(?=\s*red)";
const RE_GREEN: &str = r"(\d+)(?=\s*green)";
const RE_BLUE: &str = r"(\d+)(?=\s*blue)";

const RE_GAME: &str = r"Game (\K\d+)";

fn main() {
    let data = fs::read_to_string("input/input.txt").expect("Unable to read file");
    let vec_str: Vec<&str> = data.split("\n").collect();
    let res = prepare_data(&vec_str);
    const input: (u8, u8, u8) = (12, 13, 14);

    let result: u32 = res
        .iter()
        .filter(|x| {
            x.1.iter()
                .all(|(a, b, c)| (a <= &input.0) && (b <= &input.1) && (c <= &input.2))
        })
        .map(|x| x.0 as u32)
        .sum();

    let result2: u32 = res
        .iter()
        .map(|x| {
            let mut max_red = x.1.iter().map(|x| x.0).max().unwrap();
            let mut max_green = x.1.iter().map(|x| x.1).max().unwrap();
            let mut max_blue = x.1.iter().map(|x| x.2).max().unwrap();
            if max_red == 0 {
                max_red = 1
            }
            if max_green == 0 {
                max_green = 1
            }
            if max_blue == 0 {
                max_blue = 1
            }
            max_red as u32 * max_green as u32 * max_blue as u32
        })
        .sum();
}

fn prepare_data(input: &Vec<&str>) -> Vec<(u8, Vec<(u8, u8, u8)>)> {
    let re_red = Regex::new(RE_RED).unwrap();
    let re_green = Regex::new(RE_GREEN).unwrap();
    let re_blue = Regex::new(RE_BLUE).unwrap();
    let re_game = Regex::new(RE_GAME).unwrap();

    let mut vec_res: Vec<(u8, Vec<(u8, u8, u8)>)> = Vec::new();

    for game in input {
        let sets: Vec<&str> = game.split(";").collect();
        let mut set_res: Vec<(u8, u8, u8)> = Vec::new();
        let index: u8 = re_game
            .find(game)
            .unwrap()
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        for set in &sets {
            // println!("Set: {:?}", set);
            let (red, green, blue);
            if let Some(red_str) = re_red.find(set).expect("Regex Error") {
                // println!("red: {:?}", red_str.as_str());
                red = red_str.as_str().parse().expect("Cannot parse red");
            } else {
                red = 0u8
            }
            if let Some(green_str) = re_green.find(set).expect("Regex Error") {
                // println!("green: {:?}", green_str.as_str());
                green = green_str.as_str().parse().expect("Cannot parse green");
            } else {
                green = 0u8
            }
            if let Some(blue_str) = re_blue.find(set).expect("Regex Error") {
                // println!("blue: {:?}", blue_str.as_str());
                blue = blue_str.as_str().parse().expect("Cannot parse blue")
            } else {
                blue = 0u8
            }
            set_res.push((red, green, blue))
        }
        vec_res.push((index, set_res));
    }
    vec_res
}
