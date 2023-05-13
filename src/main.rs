use std::io::{ self, Write };
use rand::Rng;

struct Player {
    name: String,
    color: Color,
}

enum Color {
    Blue,
    Red,
}

const SIZE: usize = 9;

fn main() {
    let mut user1_name = String::new();
    let mut user2_name = String::new();

    print!("Player 1 choose a username: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut user1_name).expect("failed to read line");

    print!("Player 2 choose a username: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut user2_name).expect("failed to read line");

    user1_name = user1_name.trim_end().to_string();
    user2_name = user2_name.trim_end().to_string();

    let mut player1 = Player {
        name: user1_name.clone(),
        color: Color::Blue,
    };

    let mut player2 = Player {
        name: user2_name.clone(),
        color: Color::Red,
    };

    let mut turn_control = false;

    println!("--------------- Head means Blue, tail means Red. ---------------");

    loop {
        print!("{} is guessing, type '1' for head(Blue), type '2' for tail(Red): ", user1_name);
        let _ = io::stdout().flush();

        let mut user1_choice = String::new();
        io::stdin().read_line(&mut user1_choice).expect("failed to read line");

        match user1_choice.trim().parse::<u32>() {
            Ok(i) => {
                if i < 1 || i > 2 {
                    println!("Input must be 1 or 2. Try again.");
                    continue;
                }
                let random_num = rand::thread_rng().gen_range(1..=2);
                println!("--------------- The result is {} ---------------\n", random_num);
                turn_control = random_num != i;
                if (random_num == i && i == 1) || (random_num != i && i == 2) {
                    println!("--------------- {} is Blue  --------------------", user1_name);
                    println!("--------------- {} is Red  ---------------------", user2_name);

                    player1.color = Color::Blue;
                    player2.color = Color::Red;
                } else {
                    println!("--------------- {} is Red  --------------------", user1_name);
                    println!("--------------- {} is Blue  ---------------------", user2_name);

                    player1.color = Color::Red;
                    player2.color = Color::Blue;
                }
                break;
            }
            Err(..) => println!("Input must be number. Try Again."),
        }
    }

    let mut game_board: [[char; SIZE]; SIZE] = [[' '; SIZE]; SIZE];
    display_board(&mut game_board);
    loop {
        let currentPlayer = if turn_control { &player2 } else { &player1 };
        let hasWinner: bool = make_move(&mut game_board, currentPlayer);
        display_board(&mut game_board);
        if hasWinner {
            println!("{} won!", currentPlayer.name);
            break;
        }
        turn_control = !turn_control;
    }
}

fn get_enum_value(color: &Color) -> char {
    match color {
        Color::Blue => 'B',
        Color::Red => 'R',
    }
}

fn make_move(game_board: &mut [[char; SIZE]; SIZE], player: &Player) -> bool {
    loop {
        let mut player_choice = String::new();
        print!("{} Enter Slot to Drop (1-{}): ", player.name, SIZE);
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut player_choice).expect("failed to read line");
        match player_choice.trim().parse::<usize>() {
            Ok(i) => {
                if i < 1 || i > SIZE {
                    println!("Input must be in 1 to 10. Try again.");
                    continue;
                }
                let mut count = 0;
                for j in (0..SIZE).rev() {
                    if game_board[i - 1][j] == ' ' {
                        game_board[i - 1][j] = get_enum_value(&player.color);
                        return check_winner(game_board, i - 1, j);
                    } else {
                        count += 1;
                    }
                }
                if count == SIZE {
                    println!("This column is full. Choose different column.");
                    continue;
                }
                break;
            }
            Err(..) => println!("Input must be positive number. Try Again."),
        }
    }
    return false;
}

fn display_board(game_board: &mut [[char; SIZE]; SIZE]) {
    println!("\n----------------------- Game Board -----------------------\n");

    let mut j = 0;
    let mut i;

    print!("   ");
    let _ = io::stdout().flush();

    for i in 1..=SIZE {
        print!("   {}  ", i);
        let _ = io::stdout().flush();
    }
    println!();

    while j < SIZE {
        i = 0;
        print!("{} |", j + 1);
        while i < SIZE {
            print!("|  {}  ", game_board[i][j]);

            if i == SIZE - 1 {
                print!("||");
            }

            i = i + 1;
        }
        println!();
        print!("----");

        for i in 0..=SIZE * 6 {
            print!("-");
            let _ = io::stdout().flush();
        }
        println!();

        j = j + 1;
    }
}

// fn check_winner(game_board: &mut [[char; SIZE]; SIZE], col: usize, row: usize) -> bool {
//     // Vertical control
//     if
//         row <= 5 &&
//         game_board[col][row] == game_board[col][row + 1] &&
//         game_board[col][row + 1] == game_board[col][row + 2] &&
//         game_board[col][row + 2] == game_board[col][row + 3]
//     {
//         return true;
//     } else if
//         col > 2 &&
//         game_board[col][row] == game_board[col - 1][row] &&
//         game_board[col - 1][row] == game_board[col - 2][row] &&
//         game_board[col - 2][row] == game_board[col - 3][row]
//     {
//         return true;
//     } else if
//         col > 0 &&
//         col < 6 &&
//         game_board[col][row] == game_board[col - 1][row] &&
//         game_board[col][row] == game_board[col + 1][row] &&
//         game_board[col + 1][row] == game_board[col + 2][row]
//     {
//         return true;
//     }

//     return false;
// }

fn check_winner(board: &mut [[char; SIZE]; SIZE], col: usize, row: usize) -> bool {
    let directions = [
        (0, 1),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
    ];

    for &(dx, dy) in directions.iter() {
        let mut count = 1;

        for step in 1..=3 {
            let check_col = ((col as isize) + dx * (step as isize)) as usize;
            let check_row = ((row as isize) + dy * (step as isize)) as usize;

            if check_col >= SIZE || check_row >= SIZE {
                break;
            }

            if board[check_col][check_row] != board[col][row] {
                break;
            }

            count += 1;
        }

        if count >= 4 {
            return true;
        }
    }

    false
}