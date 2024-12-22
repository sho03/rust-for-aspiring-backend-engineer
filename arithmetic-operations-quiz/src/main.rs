use rand::Rng;

fn main() {
    let mut num_of_correct = 0;
    let mut num_of_wrong = 0;
    while num_of_correct < 3 && num_of_correct + num_of_wrong < 10 {
        let op1 = rand::thread_rng().gen_range(0..100);
        let op2 = rand::thread_rng().gen_range(0..100);
        let quiz_mode = rand::thread_rng().gen_range(1..=2);

        let result = match quiz_mode {
            1 => ("+", op1 + op2),
            2 => ("-", op1 - op2),
            _ => unreachable!(),
        };

        let operation = result.0;
        let correct_answer = result.1;


        loop {
            println!("{} {} {} = ??", op1, operation, op2);
            println!("input answer!");
            // get input from stdin
            let mut ans_input = String::new();
            std::io::stdin().read_line(&mut ans_input).unwrap();
            // parse input string to int
            let ans_input = ans_input.trim().parse::<i32>().ok();

            match ans_input {
                None => {
                    println!("input numbers!");
                    continue;
                }
                Some(ans_input) => {
                    if ans_input == correct_answer {
                        println!("correct!");
                        num_of_correct += 1;
                    } else {
                        println!("wrong!");
                        num_of_wrong += 1;
                    }
                    break;
                }
            }
        }
    }

    println!("done!");
    let answer_count = num_of_correct + num_of_wrong;
    println!("Result: {}/{}", num_of_correct, answer_count);
}
