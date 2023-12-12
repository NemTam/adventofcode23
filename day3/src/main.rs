use std::fs;

fn main() {
    let data = fs::read_to_string("./input/input.txt").expect("Unable to read file");
    let vec_str: Vec<&str> = data.split("\n").collect();
    let v: Vec<Vec<char>> = Vec::new();
    vec_str.iter().for_each(|x| println!("{x}"));
    let v: Vec<Vec<char>> = vec_str.iter().map(|x| x.chars().collect()).collect();
    let mut result: Vec<Vec<(char, bool)>> = Vec::new();
    for (i, row) in v.iter().enumerate() {
        let mut numbers: Vec<Vec<(char, bool)>> = Vec::new();
        let mut helper_vec: Vec<(char, bool)> = Vec::new();
        for (j, _) in row.iter().enumerate() {
            println!("x: {i} y: {j} {}", v[i][j]);
            let mut elem = &v[i][j];
            if elem.is_digit(10) {
                let mut valid_num = false;
                println!("{elem}");
                // numbers.push(*elem); // sztem ez meg ne itt legyen hanem amikor tudjuk hogy van e csillag korulotte
                // break;
                // Korbenezunk balra fel
                if (i > 0) && (j > 0) {
                    if !v[i - 1][j - 1].is_digit(10) && v[i - 1][j - 1] != '.' {
                        println!("balra fent specko: {}", v[i - 1][j - 1]);
                        valid_num = true
                    }
                }
                // Korbenezunk fel
                if i > 0 {
                    if !v[i - 1][j].is_digit(10) && v[i - 1][j] != '.' {
                        println!(" fent specko: {}", v[i - 1][j]);
                        valid_num = true
                    }
                }
                // Korbenezunk jobbra fel
                if i > 0 && j < row.len() - 1 {
                    if !v[i - 1][j + 1].is_digit(10) && v[i - 1][j + 1] != '.' {
                        println!("jobbra fent specko: {}", v[i - 1][j + 1]);
                        valid_num = true
                    }
                }
                // Korbenezunk balra
                if j > 0 {
                    if !v[i][j - 1].is_digit(10) && v[i][j - 1] != '.' {
                        println!("balra specko: {}", v[i][j - 1]);
                        valid_num = true
                    }
                }
                // Korbenezunk jobbra
                if j < row.len() - 1 {
                    if !v[i][j + 1].is_digit(10) && v[i][j + 1] != '.' {
                        println!("jobbra specko: {}", v[i][j + 1]);
                        valid_num = true
                    }
                }
                // Korbenezunk balra le
                if (i < v.len() - 1) && (j > 0) {
                    if !v[i + 1][j - 1].is_digit(10) && v[i + 1][j - 1] != '.' {
                        println!("balra lent specko: {}", v[i + 1][j - 1]);
                        valid_num = true
                    }
                }
                // Korbenezunk le
                if i < v.len() - 1 {
                    if !v[i + 1][j].is_digit(10) && v[i + 1][j] != '.' {
                        println!("lent specko: {}", v[i + 1][j]);
                        valid_num = true
                    }
                }
                // Korbenezunk jobbra le
                if (i < v.len() - 1) && j < row.len() - 1 {
                    if !v[i + 1][j + 1].is_digit(10) && v[i + 1][j + 1] != '.' {
                        println!("jobbra lent specko: {}", v[i + 1][j + 1]);
                        valid_num = true
                    }
                }
                helper_vec.push((v[i][j], valid_num));
                println!("helper vekker{:?}", helper_vec);

                // Korbenezunk jobbra, hogy vege e a szamnak
                if j < row.len() - 1 {
                    if !v[i][j + 1].is_digit(10) {
                        println!("jobbra vesszo: {}", v[i][j + 1]);
                        helper_vec.push((',', false));
                    }
                } else if j == row.len() - 1 {
                    println!("jobbra vege: {}", v[i][j]);
                    helper_vec.push((',', false));
                }
            }
        }
        let mut helper_vec_2: Vec<(char, bool)> = Vec::new();
        for elem in &helper_vec {
            println!("{:?}", elem);
            println!("{:?}", helper_vec_2);
            if elem.0 == ',' {
                numbers.push(helper_vec_2.clone());
                helper_vec_2.clear();
            } else {
                helper_vec_2.push(*elem);
            }
        }
        result.extend(numbers.clone());
        numbers.clear();
    }
    let mut sum = 0;
    for elem in &result {
        if elem.iter().any(|x| x.1) {
            let number: String = elem.iter().map(|x| x.0).collect();
            let number: u32 = number.parse().unwrap();
            sum += number;
        }
    }
    println!("{sum}");
}
