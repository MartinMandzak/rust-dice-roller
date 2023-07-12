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
                println!("\n#####################\ninput looks like this: 4d20 -> rolls 4 20-sided die\n4d20 5d6 will roll both \ntype end to finish\n#####################\n")
            }
            "end" => break,
            _ => println!(
                "Total: {}!",
                roll(input.trim_end().to_lowercase().split(" ").collect())
            ),
        }
    }

    fn roll(input: Vec<&str>) -> i32 {
        let mut result: i32 = 0;
        for element in input {
            let mut rng = rand::thread_rng();
            let temp: Vec<&str> = element.split("d").collect();
            let rolls: Vec<i32> = (0..temp[0].parse().unwrap())
                .map(|_: i32| -> i32 { rng.gen_range(1..temp[1].parse().unwrap()) })
                .collect();
            for e in rolls {
                println!("{}", e);
                result += e;
            }
        }
        return result;
    }
}
