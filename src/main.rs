mod tools;
use tools::*;

use std::{io, num::ParseIntError};
use rand::Rng;

fn read_num () -> Result<i32, ParseIntError> {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error.");

    return input.trim().parse::<i32>();
}

fn read_ans () -> String {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error.");

    return input;
} 

fn parse_game(input: i32) -> Option<tools::Game> {
    match input {
        1 => Some(Game::from(Game::STONE)),
        2 => Some(Game::from(Game::PAPER)),
        3 => Some(Game::from(Game::SCISSOR)),
        _ => None, 
    }
}

fn play_game(round: i32, user_hand: Game) -> ResultU {
       let mut random_input = rand::thread_rng();
       let computer_hand = parse_game(random_input.gen_range(1..=3)).unwrap();
       let hand_result = get_result(&user_hand, &computer_hand);
       return ResultU {
        user: user_hand,
        computer: computer_hand,
        result: hand_result,
        round
       } 
} 

fn get_result(user_hand: &Game, computer_hand: &Game) -> Hand {
     match user_hand {
        Game::STONE => match computer_hand {
            Game::STONE => Hand::DRAW,
            Game::PAPER => Hand::LOST,
            Game::SCISSOR => Hand::WON,
        },
        Game::PAPER => match computer_hand {
            Game::STONE => Hand::WON,
            Game::PAPER => Hand::DRAW,
            Game::SCISSOR => Hand::LOST,
        } 
        Game::SCISSOR => match computer_hand {
            Game::STONE => Hand::LOST,
            Game::PAPER => Hand::WON,
            Game::SCISSOR => Hand::DRAW,
        },
    }
}

fn start_game(round: i32) {
    let mut count = 0;
    let mut collect_result: Vec<ResultU> = Vec::new();

    loop {
        println!("round {}, [1]stone [2]paper [3]scissor", count + 1);
        let num = read_num();

        if num.is_ok() {
            let get_value = parse_game(num.unwrap());

            if get_value.is_some() {
                let res =  play_game(count, get_value.unwrap());
                println!("Result: {:?}, your hand: {:?}, computer hand {:?}", res.result, res.user, res.computer);
                collect_result.push(res);
                count += 1;
            }
        }

        if count == round {
            println!("End Of Game.\nResult");
            collect_result.iter().for_each(|e|  println!("{:?}", e));
            break;
        } 


    }
}


fn main() {
    println!("Hi! Let's play game\nPress [Y] to continue or other key to cancel ...");

    let key = read_ans();
    if !key.trim().eq("y") {
        println!("Exit");
        std::process::abort();
    } 

    loop {
        println!("how many round you want to play?");
        let num = read_num();

        if num.is_ok() {
            start_game(num.unwrap());

            println!("Do you want to play again? press n to exit ");
            let ans = read_ans();

            if ans.trim().eq_ignore_ascii_case("n") {
                println!("bye");
                break;
            }
        } else {
            println!("please write a correct number")
        }
    
    }

}


