use std::io;
fn main() {
    let mut counter: u32 = 0;

    'main: loop {
        println!("||{:=^49}||", "");
        println!("||{:^49}||", "print 's' for start or 'e' exit");
        println!("||{:=^49}||", "");

        'start: loop {
            let mut command = String::new();

            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read line");

            match command.trim().parse() {
                Ok('s') => break 'start,
                Ok('e') => break 'main,
                Ok(_) => {
                    println!("|{:=^51}|", "Stupid, type 's' or 'e'");
                    counter = counter + 1;
                    continue;
                }
                Err(_) => {
                    println!("|{:=^51}|", "Stupid, type 's' or 'e'");
                    counter = counter + 1;
                    continue;
                }
            };
        }
        println!("|{:^51}|", "first number:");

        let first_number = take_number();
        counter = &counter + first_number.1;
        let first_number = first_number.0;

        println!("|{:^51}|", "second number:");

        let second_number = take_number();
        counter = counter + second_number.1;
        let second_number = second_number.0;

        println!("|{:^51}|", "operator: */+-");

        let operator = take_operator();
        counter = counter + operator.1;
        let operator = operator.0;

        let result: f64 = match operator {
            '+' => first_number + second_number,
            '-' => first_number - second_number,
            '*' => first_number * second_number,
            '/' => first_number / second_number,
            _ => continue,
        };

        println!("|{:=^51}|", "result");
        println!("|{:^51}|", result);
        println!("|{:=^51}|", "");
    }

    if counter > 10 {
        println!(
            "\n Do you have crooked fingers? Mistakes: {}!!!!!!!!!",
            counter
        );
    } else if counter > 5 && counter <= 10 {
        println!("\n facepalm. Mistakes: {}!!!!", counter);
    } else {
        println!("\n OK. Mistakes: {}.", counter);
    }

    let mut exit = String::new();

    io::stdin()
        .read_line(&mut exit)
        .expect("Failed to read line");
}

fn take_number() -> (f64, u32) {
    let mut counter: u32 = 0;
    loop {
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: f64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("|{:=^51}|", "Stupid, type a number");
                counter = counter + 1;
                continue;
            }
        };

        return (number, counter);
    }
}

fn take_operator() -> (char, u32) {
    let mut counter: u32 = 0;
    loop {
        let mut operator = String::new();

        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read line");

        let operator: char = match operator.trim().parse() {
            Ok('+') => '+',
            Ok('-') => '-',
            Ok('*') => '*',
            Ok('/') => '/',
            Ok(_) => {
                println!("|{:=^51}|", "Stupid, type operator");
                counter = counter + 1;
                continue;
            }
            Err(_) => {
                println!("|{:=^51}|", "Stupid, type operator");
                counter = counter + 1;
                continue;
            }
        };

        return (operator, counter);
    }
}
