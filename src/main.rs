use std::io::{ self, Write };
use std::fs;
use rand::Rng;
use std::fs::OpenOptions;

mod minimax;
// minimax::func()
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
    loop {
        start();
        let mut user_choice = String::new();
        println!("----------------------Press 1 for go back to menu.----------------------");
        print!("Operation: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut user_choice).expect("failed to read line");
        if user_choice.trim() != "1" {
            break;
        }
    }
}

fn start() {
    let (player1, player2, mut turn_control, mut move_counter, mut game_board);
    loop {
        println!(
            "---------------------------------MENU---------------------------------\n1) Recover last session.\n2) Create new Game."
        );
        let mut user_choice = String::new();

        print!("Operation: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut user_choice).expect("failed to read line");

        match user_choice.trim().parse::<u32>() {
            Ok(i) => {
                if i == 1 {
                    (player1, player2, turn_control, move_counter, game_board) = recover_session();
                    break;
                } else if i == 2 {
                    (player1, player2, turn_control, move_counter, game_board) = create_new_game();
                    break;
                } else {
                    println!("Input must be 1 or 2. Try again.");
                    continue;
                }
            }
            Err(..) => println!("Input must be number. Try Again."),
        }
    }

    display_board(&mut game_board);

    println!("\nPlayer can always leave by pressing the 'q'. Let the game begin.\n");
    loop {
        let current_player = if turn_control { &player2 } else { &player1 };
        println!(
            "++++++++++++++++++ {}({}) is playing. Move {} +++++++++++++++++++ ",
            current_player.name,
            enum_type_to_value(&current_player.color),
            move_counter
        );
        let has_winner: bool = make_move(&mut game_board, current_player);
        display_board(&mut game_board);
        if has_winner {
            println!("{} won!", current_player.name);
            reset_files();
            break;
        }
        move_counter += 1;
        if move_counter >= 80 {
            println!("Draw!");
            reset_files();
            break;
        }
        turn_control = !turn_control;
        update_board_file(
            &player1.name,
            enum_type_to_value(&player1.color).to_string(),
            &player2.name,
            enum_type_to_value(&player2.color).to_string(),
            turn_control.to_string(),
            move_counter.to_string(),
            &mut game_board
        );
    }
}

fn create_new_game() -> (Player, Player, bool, i32, [[char; SIZE]; SIZE]) {
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

    let turn_control;

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

    let game_board: [[char; SIZE]; SIZE] = [[' '; SIZE]; SIZE];
    (player1, player2, turn_control, 1, game_board)
}

fn recover_session() -> (Player, Player, bool, i32, [[char; SIZE]; SIZE]) {
    let data = fs::read_to_string("tahta.txt").expect("Should have been able to read the file");
    if data == "" {
        println!("There is no recorded game. New game created. ");
        return create_new_game();
    }
    let data_array: Vec<_> = data.split("\n").collect();

    let turn_control: bool = match data_array[4] {
        "true" => true,
        "false" => false,
        &_ => false,
    };

    let move_counter = data_array[5].trim().parse::<i32>().unwrap();

    let p1 = Player {
        name: data_array[0].to_string(),
        color: value_to_enum_type(data_array[1]),
    };

    let p2 = Player {
        name: data_array[2].to_string(),
        color: value_to_enum_type(data_array[3]),
    };

    let mut game_board: [[char; SIZE]; SIZE] = [[' '; SIZE]; SIZE];
    let board_data = data_array[6];

    let mut i = 0;

    for j in 0..9 {
        for k in 0..9 {
            game_board[j][k] = board_data.as_bytes()[i] as char;
            i += 1;
        }
    }

    (p1, p2, turn_control, move_counter, game_board)
}

fn reset_files() {
    fs::write("tahta.txt", "").expect("Unable to write file");
    fs::write("hamle.txt", "").expect("Unable to write file");
}

fn update_board_file(
    p1_name: &String,
    p1_color: String,
    p2_name: &String,
    p2_color: String,
    turn_control: String,
    move_counter: String,
    board: &mut [[char; SIZE]; SIZE]
) {
    let game_data = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        p1_name,
        p1_color,
        p2_name,
        p2_color,
        turn_control,
        move_counter
    );

    let mut board_data = String::new();
    for i in 0..SIZE {
        for j in 0..SIZE {
            board_data.push_str(&board[i][j].to_string());
        }
    }
    let data = format!("{}\n{}", game_data, board_data);
    fs::write("tahta.txt", data).expect("Unable to write file");
}

fn update_move_file(col: usize, row: usize, color: char) {
    let mut file = OpenOptions::new().append(true).open("hamle.txt").expect("cannot open file");

    let data = format!("{}{}=>{}\n", col.to_string(), row.to_string(), color);
    file.write_all(data.as_bytes()).expect("write failed");
}

fn enum_type_to_value(color: &Color) -> char {
    match color {
        Color::Blue => 'B',
        Color::Red => 'R',
    }
}

fn value_to_enum_type(color: &str) -> Color {
    match color {
        "B" => Color::Blue,
        "R" => Color::Red,
        &_ => Color::Blue,
    }
}

fn make_move(game_board: &mut [[char; SIZE]; SIZE], player: &Player) -> bool {
    loop {
        let mut player_choice = String::new();
        print!("{} Enter Slot to Drop (1-{}): ", player.name, SIZE);
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut player_choice).expect("failed to read line");
        if player_choice.trim() == "q" {
            println!("Exiting the game..");
            std::process::exit(0);
        }
        match player_choice.trim().parse::<usize>() {
            Ok(i) => {
                if i < 1 || i > SIZE {
                    println!("Input must be in 1 to 10. Try again.");
                    continue;
                }
                let mut count = 0;
                for j in (0..SIZE).rev() {
                    if game_board[i - 1][j] == ' ' {
                        let clr_char: char = enum_type_to_value(&player.color);
                        game_board[i - 1][j] = clr_char;
                        update_move_file(i, j + 1, clr_char);
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

        for _ in 0..=SIZE * 6 {
            print!("-");
            let _ = io::stdout().flush();
        }
        println!();

        j = j + 1;
    }
}

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