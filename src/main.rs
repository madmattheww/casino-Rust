use tokio_postgres::{NoTls, Error};
use std::io;
use std::io::{BufRead, Write};
use rand::Rng;
use std::{time::Duration};
use async_std::task;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut ni = String::new();
    let mut pa = String::new();
    let mut coins: i32 = 0;
    let mut games: i32 = 0;
    let mut wins: i32 = 0;
    let mut loses: i32 = 0;
    let mut is_user = false;

    println!("\n\n\nYou can play guest without saving your game result or you can create account.\n\nChoose what you want:\n1. Log In\n2. Play as guest\n3. Create account");
    let mut guest_create = String::new();
    io::stdin().read_line(&mut guest_create).expect("Failed to read line");
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=YOUR_PASSWORD dbname=casino", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });


    client.execute( //create dbase
                    "CREATE TABLE IF NOT EXISTS casino_info (
                        id BIGSERIAL PRIMARY KEY,
                        nickname TEXT NOT NULL,
                        password TEXT NOT NULL,
                        coins INTEGER NOT NULL,
                        games INTEGER NOT NULL,
                        wins INTEGER NOT NULL,
                        loses INTEGER NOT NULL
                        )",
                    &[],
    ).await?;



    match guest_create.as_str().trim() { //log in or sign up
        "1" => {  // log in
            let query = client
                .query("SELECT nickname FROM casino_info", &[]).await?;

            for row in &query {
                let nickname: &str = row.get(0);
                println!("{}", nickname);
            }

            println!("Write your nickname: ");

            let mut namee = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut namee).unwrap();
            namee = namee.trim().parse().unwrap();

            let name = client
                .query("SELECT * FROM casino_info WHERE nickname = $1", &[&namee]).await?;

            for row in &name {
                let nickname1: &str = row.get(1);
                let password1: &str = row.get(2);
                let coins1: i32 = row.get(3);
                let games1: i32 = row.get(4);
                let wins1: i32 = row.get(5);
                let loses1: i32 = row.get(6);

                println!("Write password: ");
                let mut parol = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut parol).unwrap();
                parol = parol.trim().parse().unwrap();

                if parol == password1 {
                    println!("WELCOME, {}", nickname1);
                    println!("nickname: {}\npassword: {}\ncoins: {}\ngames: {}\nwins: {}\nloses: {}", nickname1, password1, coins1, games1, wins1, loses1);
                    ni = nickname1.to_string();
                    pa = password1.to_string();
                    is_user = true;
                    break
                } else {
                    println!("Wrong password, please try again!")
                }
            }
        },
        "2" => {},
        "3" => {
            print!("Create your nickname: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut ni).unwrap();
            ni = ni.trim().parse().unwrap();


            print!("Create password: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut pa).unwrap();
            println!("It is your password: {}", pa);
            pa = pa.trim().parse().unwrap();

            task::sleep(Duration::from_secs(1)).await;


            client
                .execute("INSERT INTO casino_info (nickname, password, coins, games, wins, loses) VALUES ($1, $2, $3, $4, $5, $6)", &[&ni, &pa, &coins, &games, &wins, &loses], ).await?;

            let query = client
                .query("SELECT * FROM casino_info WHERE nickname = $1", &[&ni]).await?;

            for row in &query {
                let nickname1: &str = row.get(1);
                let password1: &str = row.get(2);
                let coins1: i32 = row.get(3);
                let games1: i32 = row.get(4);
                let wins1: i32 = row.get(5);
                let loses1: i32 = row.get(6);
                println!("nickname: {}\npassword: {}\ncoins: {}\ngames: {}\nwins: {}\nloses: {}", nickname1, password1, coins1, games1, wins1, loses1);
            }
            is_user = true;
        },
        _ => {
            println!("LOl")
        },
    }

    task::sleep(Duration::from_secs(1)).await;

    loop {
        let mut chose = String::new();
        println!("\n\nList:");
        println!("1. 777a");
        println!("2. 777s");
        println!("3. guess number");
        println!("4. roulette");
        println!("5. statistic");
        println!("6. create account");
        println!("7. break");
        println!("Write number: ");
        io::stdin().read_line(&mut chose).expect("Failed to read line");

        match chose.as_str().trim()
        {
            "1" => {  // 777a
                loop {
                    let mut rng = rand::thread_rng();
                    let mut attempt = 0;
                    loop {
                        attempt += 1;
                        games += 1;
                        println!("Attempt {}", attempt);
                        let run_num1 = rng.gen_range(0..10);
                        let run_num2 = rng.gen_range(0..10);
                        let run_num3 = rng.gen_range(0..10);
                        println!("{} {} {}", run_num1, run_num2, run_num3);
                        if run_num1 == run_num2 && run_num1 == run_num3 && run_num2 == run_num3 {
                            println!("YOU ARE WIN!");
                            println!("If you want to play again press ENTER");
                            println!("If you want to stop press any key");
                            wins += 1;
                            break;
                        } else {
                            loses += 1;
                        }
                    };

                    let stdin = io::stdin();

                    print!("If you want to continue press ENTER, if you want to end press any key");

                    let mut input = String::new();
                    stdin.lock().read_line(&mut input).unwrap();

                    if input.trim().is_empty() {
                        println!("AGAIN!")
                    } else {
                        break;
                    }
                }
                if is_user == true {
                    client
                        .execute("UPDATE casino_info SET coins = $3, games = $4, wins = $5, loses = $6 WHERE nickname = $1 AND password = $2",
                                 &[&ni, &pa, &coins, &games, &wins, &loses], ).await?;
                }
            },
            "2" => {  // 777s
                let mut rng = rand::thread_rng();
                let mut attempt = 0;
                loop {
                    attempt += 1;
                    games += 1;
                    println!("Attempt {}", attempt);
                    let run_num1 = rng.gen_range(0..10);
                    let run_num2 = rng.gen_range(0..10);
                    let run_num3 = rng.gen_range(0..10);
                    println!("{} {} {}", run_num1, run_num2, run_num3);
                    if run_num1 == run_num2 && run_num1 == run_num3 && run_num2 == run_num3 {
                        println!("YOU ARE WIN!");
                        wins += 1;
                    } else {
                        loses += 1;
                    }


                    let stdin = io::stdin();
                    println!("\n\nIf you want to continue press ENTER, if you want to end press any key");

                    let mut input = String::new();
                    stdin.lock().read_line(&mut input).unwrap();

                    if input.trim().is_empty() {
                        println!("\n\n\n\nAGAIN!");
                    } else {
                        break;
                    }
                }
                if is_user == true {
                    client
                        .execute("UPDATE casino_info SET coins = $3, games = $4, wins = $5, loses = $6 WHERE nickname = $1 AND password = $2",
                                 &[&ni, &pa, &coins, &games, &wins, &loses], ).await?;
                }
            },
            "3" => {  // guess number
                loop {
                    let mut num_str = String::new();
                    println!("Enter number from 1 to 9: ");
                    games += 1;
                    io::stdin().read_line(&mut num_str).expect("Failed to read line");
                    let mut num: u8 = num_str.trim().parse().expect("Error str to u");
                    if num >= 11 {
                        break
                    }
                    println!("\n\n\n\n\n\n");
                    let mut rng = rand::thread_rng();
                    let ran_num: u8 = rng.gen_range(0..10);
                    if num == ran_num {
                        println!("You are win\nIt was {}", num);
                        wins += 1;
                    } else {
                        println!("You are lose\nIt was not {}\nIT WAS {}", num, ran_num);
                        loses += 1;
                    }
                    println!("If you want to stop write number greater than 10")
                }
                if is_user == true {
                    client
                        .execute("UPDATE casino_info SET coins = $3, games = $4, wins = $5, loses = $6 WHERE nickname = $1 AND password = $2",
                                 &[&ni, &pa, &coins, &games, &wins, &loses], ).await?;
                }
            },
            "4" => { // roulette
                loop {
                    let mut t = String::new();
                    println!("1 - choose nums\n2 - guess color\n3 - guess even or odd\n4 - guess which 12\n5 - break");
                    io::stdin().read_line(&mut t).expect("Failed to read line");
                    let mut num: u8 = t.trim().parse().expect("Error str to u");

                    match num {
                        1 => {
                            loop {
                                games += 1;
                                let mut numbers = Vec::new();
                                let mut rng = rand::thread_rng();
                                let run_num1 = rng.gen_range(0..37);
                                let mut is_win = false;

                                loop {
                                    print!("Enter a number (or type 'done' to finish): ");
                                    io::stdout().flush().unwrap();

                                    let mut input = String::new();
                                    io::stdin().read_line(&mut input).unwrap();

                                    if input.trim() == "done" {
                                        break;
                                    }

                                    let number: i32 = match input.trim().parse() {
                                        Ok(num) => num,
                                        Err(_) => {
                                            println!("Invalid input. Please enter a number.");
                                            continue;
                                        }
                                    };

                                    numbers.push(number);
                                }

                                for &num in &numbers {
                                    if num == run_num1 {
                                        is_win = true;
                                    }
                                }

                                if is_win == true {
                                    println!("YOU ARE WIN!\nYou choose {:?}\nIt is {}", numbers, run_num1);
                                    wins += 1;
                                } else {
                                    println!("YOU ARE LOSE!\nYou choose {:?}\nIt is {}", numbers, run_num1);
                                    loses += 1;
                                }
                                break
                            }
                            client
                                .execute("UPDATE casino_info SET coins = $3, games = $4, wins = $5, loses = $6 WHERE nickname = $1 AND password = $2",
                                         &[&ni, &pa, &coins, &games, &wins, &loses], ).await?;
                        },
                        2 => {
                            loop {
                                games += 1;
                                let mut is_red = false;
                                let mut is_black = false;
                                let mut is_green = false;

                                println!("0 - green\n1 - red\n2 - black");
                                let mut a = String::new();

                                io::stdin().read_line(&mut a).expect("Failed to read line");
                                let mut num: u8 = a.trim().parse().expect("Error str to u");

                                if num == 0 {
                                    is_green = true
                                } else if num == 1 {
                                    is_red = true
                                } else if num == 2 {
                                    is_black = true
                                } else {
                                    println!("ERROR")
                                }

                                println!("\n\n\n\n\n");

                                let mut rng = rand::thread_rng();
                                let run_num1 = rng.gen_range(0..37);

                                if run_num1 == 0 { //green 0
                                    println!("GREEN\n{}", run_num1);
                                    if is_green == true {
                                        println!("You are win");
                                        wins += 1;
                                    } else {
                                        println!("You are lose");
                                        loses += 1;
                                    }
                                } else if run_num1 <= 27 && run_num1 % 2 == 1 { // red 1 - 27
                                    println!("RED\n{}", run_num1);
                                    if is_red == true {
                                        println!("You are win");
                                        wins += 1;
                                    } else {
                                        println!("You are lose");
                                        loses += 1;
                                    }
                                } else if run_num1 > 28 && run_num1 % 2 == 0 { //red 29 - 36
                                    println!("RED\n{}", run_num1);
                                    if is_red == true {
                                        println!("You are win");
                                        wins += 1;
                                    } else {
                                        println!("You are lose");
                                        loses += 1;
                                    }
                                } else if run_num1 <= 28 && run_num1 % 2 == 0 { //black 2 - 28
                                    println!("BLACK\n{}", run_num1);
                                    if is_black == true {
                                        println!("You are win");
                                        wins += 1;
                                    } else {
                                        println!("You are lose");
                                        loses += 1;
                                    }
                                } else if run_num1 > 28 && run_num1 % 2 == 1 { // black 29 - 35
                                    println!("BLACK\n{}", run_num1);
                                    if is_black == true {
                                        println!("You are win");
                                        wins += 1;
                                    } else {
                                        println!("You are lose");
                                        loses += 1;
                                    }
                                } else {
                                    println!("ERROR")
                                }
                                break
                            }
                            client
                                .execute("UPDATE casino_info SET coins = $3, games = $4, wins = $5, loses = $6 WHERE nickname = $1 AND password = $2",
                                         &[&ni, &pa, &coins, &games, &wins, &loses], ).await?;
                        },
                        3 => {
                            loop {
                                games += 1;
                                let mut is_even = false;
                                let mut is_odd = false;
                                let mut rng = rand::thread_rng();
                                let run_num1 = rng.gen_range(0..37);


                                let mut a = String::new();
                                println!("1 - even\n2 - odd");
                                io::stdin().read_line(&mut a).expect("Failed to read line");
                                let mut num: u8 = a.trim().parse().expect("Error str to u");


                                match num {
                                    1 => {
                                        is_even = true
                                    },
                                    2 => {
                                        is_odd = true
                                    },
                                    _ => {
                                        println!("ERROR")
                                    }
                                }


                                if run_num1 % 2 == 0 {
                                    if is_even == true {
                                        println!("YOU ARE WIN\nIT IS {}", run_num1);
                                        wins += 1;
                                    } else {
                                        println!("YOU ARE LOSE\nIT IS {}", run_num1);
                                        loses += 1;
                                    }
                                } else {
                                    if is_odd == true {
                                        println!("YOU ARE WIN\nIT IS {}", run_num1);
                                        wins += 1;
                                    } else {
                                        println!("YOU ARE LOSE\nIT IS {}", run_num1);
                                        loses += 1;
                                    }
                                }
                                break
                            }
                            client
                                .execute("UPDATE casino_info SET coins = $3, games = $4, wins = $5, loses = $6 WHERE nickname = $1 AND password = $2",
                                         &[&ni, &pa, &coins, &games, &wins, &loses], ).await?;
                        },
                        4 => {
                            loop {
                                games += 1;
                                let mut is_f = false;
                                let mut is_s = false;
                                let mut is_t = false;

                                let mut rng = rand::thread_rng();
                                let run_num1 = rng.gen_range(0..37);

                                let mut a = String::new();
                                println!("1 - first 12\n2 - second 12\n3 - third 12");
                                io::stdin().read_line(&mut a).expect("Failed to read line");
                                let mut num: u8 = a.trim().parse().expect("Error str to u");

                                if num == 1 {
                                    is_f = true
                                } else if num == 2 {
                                    is_s = true
                                } else if num == 3 {
                                    is_t = true
                                } else {
                                    println!("ERROR")
                                }

                                match run_num1 {
                                    1..=12 => {
                                        if is_f == true {
                                            println!("YOU ARE WIN!");
                                            println!("IT IS {}", run_num1);
                                            wins += 1;
                                        } else {
                                            println!("YOU ARE LOSE!");
                                            println!("IT IS {}", run_num1);
                                            loses += 1;
                                        }
                                    },
                                    13..=24 => {
                                        if is_s == true {
                                            println!("YOU ARE WIN!");
                                            println!("IT IS {}", run_num1);
                                            wins += 1;
                                        } else {
                                            println!("YOU ARE LOSE!");
                                            println!("IT IS {}", run_num1);
                                            loses += 1;
                                        }
                                    },
                                    25..=36 => {
                                        if is_t == true {
                                            println!("YOU ARE WIN!");
                                            println!("IT IS {}", run_num1);
                                            wins += 1;
                                        } else {
                                            println!("YOU ARE LOSE!");
                                            println!("IT IS {}", run_num1);
                                            loses += 1;
                                        }
                                    },
                                    _ => {
                                        println!("ERROR")
                                    }
                                }
                                break
                            }
                            client
                                .execute("UPDATE casino_info SET coins = $3, games = $4, wins = $5, loses = $6 WHERE nickname = $1 AND password = $2",
                                         &[&ni, &pa, &coins, &games, &wins, &loses], ).await?;
                        },
                        5 => {
                            break;
                        },
                        _ => {
                            println!("ERROR")
                        }
                    }
                }
            },
            "5" => {  // statistic
                if is_user == true {
                    let query = client
                        .query("SELECT * FROM casino_info WHERE nickname = $1", &[&ni]).await?;

                    for row in &query {
                        let nickname1: &str = row.get(1);
                        let coins1: i32 = row.get(3);
                        let games1: i32 = row.get(4);
                        let wins1: i32 = row.get(5);
                        let loses1: i32 = row.get(6);
                        println!("\nnickname: {}\ncoins: {}\ngames: {}\nwins: {}\nloses: {}", nickname1, coins1, games1, wins1, loses1);
                    }
                } else {
                    println!("\nnickname: Guest\ncoins: {}\ngames: {}\nwins: {}\nloses: {}", coins, games, wins, loses);
                }
                let stdin = io::stdin();
                println!("\n\npress ENTER");

                let mut input = String::new();
                stdin.lock().read_line(&mut input).unwrap();

                if input.trim().is_empty() {
                    println!("\n\n\n\nLET'S GO");
                }
            },
            "6" => {  // create account
                if is_user == true {
                    println!("You already have account!");
                    println!("nickname: {}", ni);

                    let stdin = io::stdin();
                    println!("\n\npress ENTER");

                    let mut input = String::new();
                    stdin.lock().read_line(&mut input).unwrap();

                    if input.trim().is_empty() {
                        println!("\n\n\n\nLET'S GO");
                    }
                } else {
                    print!("Create your nickname: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut ni).unwrap();
                    ni = ni.trim().parse().unwrap();


                    print!("Create password: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut pa).unwrap();
                    println!("It is your password: {}", pa);
                    pa = pa.trim().parse().unwrap();

                    task::sleep(Duration::from_secs(1)).await;


                    client
                        .execute("INSERT INTO casino_info (nickname, password, coins, games, wins, loses) VALUES ($1, $2, $3, $4, $5, $6)", &[&ni, &pa, &coins, &games, &wins, &loses], ).await?;

                    let query = client
                        .query("SELECT * FROM casino_info WHERE nickname = $1", &[&ni]).await?;

                    for row in &query {
                        let nickname1: &str = row.get(1);
                        let password1: &str = row.get(2);
                        let coins1: i32 = row.get(3);
                        let games1: i32 = row.get(4);
                        let wins1: i32 = row.get(5);
                        let loses1: i32 = row.get(6);
                        println!("nickname: {}\npassword: {}\ncoins: {}\ngames: {}\nwins: {}\nloses: {}", nickname1, password1, coins1, games1, wins1, loses1);
                    }
                    is_user = true
                }
            },
            "7" => {  // break
                break
            },
            "8" => {
                println!("{}", is_user)
            },
            _ => {
                println!("WRONG NUMBER");
            }
        }
    }
    println!("THANK YOU FOR YOUR GAME!");
    Ok(())
}