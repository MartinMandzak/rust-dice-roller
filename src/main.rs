use std::io;

use rand::Rng;

fn main() {
    let running = true;
    while running == true {
        println!("your input (type help for more info): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.to_lowercase().trim_end() {
            "help" => {
                println!("input looks like this: 4d20 -> rolls 4 20-sided die \ntype end to finish")
            }
            "end" => break,
            _ => println!(
                "Total: {}!",
                roll(input.trim_end().to_lowercase().split("d").collect())
            ),
        }
    }

    fn roll(input: Vec<&str>) -> i32 {
        let mut result: i32 = 0;
        for i in (0..input.len()).step_by(2) {
            let mut rng = rand::thread_rng();
            //(0..input[i].parse().unwrap()).map(|_| println!("hi"));
            let rolls: Vec<i32> = (0..input[i].parse().unwrap())
                .map(|_: i32| -> i32 { rng.gen_range(1..input[i + 1].parse().unwrap()) })
                .collect();
            for e in rolls {
                println!("{}", e);
                result += e;
            }
        }
        return result;
    }
}
