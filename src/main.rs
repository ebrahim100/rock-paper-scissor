use std::io;
use rand::Rng;

#[derive(Debug)]
enum Game {
    STONE,
    PAPER,
    SCISSOR,
}

#[derive(Debug)]
enum Hand {
    WON,
    LOST,
    DRAW,
}

#[derive(Debug)]
struct Result {
    user: Game,
    computer: Game,
    result: Hand,
    round: i32,
}


fn parse_game(input: i32) -> Option<Game> {
    match input {
        1 => Some(Game::from(Game::STONE)),
        2 => Some(Game::from(Game::PAPER)),
        3 => Some(Game::from(Game::SCISSOR)),
        _ => None, 
    }
}

fn play_game(round: i32, user_hand: Game) -> Result {
       let mut random_input = rand::thread_rng();
       let computer_hand = parse_game(random_input.gen_range(1..=3)).unwrap();
       let hand_result = get_result(&user_hand, &computer_hand);
       return Result {
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

fn main() {
    println!("Hi! Let's play game\nPress Y to continue or other key to cancel ...");

    let mut key = String::new();

    io::stdin()
        .read_line(&mut key)
        .expect("Faild to read line");

    if key.eq_ignore_ascii_case("Y") {
        println!("Exit");
        std::process::abort();
    }

    loop {
        println!("how many round you want to play?");
        let mut round: String = String::new();
        io::stdin().read_line(&mut round)
            .expect("Faild to read line");

        start_game(round.trim().parse().expect("Please type a number"));
        break;
    }

}

fn start_game(round: i32) {
    let mut count = 0;
    let mut collect_result: Vec<Result> = Vec::new();
    loop {
        let mut input: String = String::new();
        println!("round {}, [1]stone [2]paper [3]scissor", count + 1);

        io::stdin()
            .read_line(&mut input)
            .expect("Error.");

        let get_value = parse_game(input.trim().parse().expect("something went wrong"));

        if get_value.is_some() {
           let res =  play_game(count, get_value.unwrap());
           println!("Result: {:?}, your hand: {:?}, computer hand {:?}", res.result, res.user, res.computer);
           collect_result.push(res);
           count += 1;
        }else {
            println!("wrong input: try again");
        }

         

        if count == round {
            println!("End Of Game.\nResult");
            collect_result.iter().for_each(|e|  println!("{:?}", e));
            break;
        } 


    }
}
