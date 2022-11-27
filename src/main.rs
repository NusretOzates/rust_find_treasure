use std::io::{stdin};

/** Get possible moves by looking current coordination for 4*4 map

# Example
```rust
let position = (0,0);
let result = get_moves(position);
let expected = vec!["G","D"];

assert_eq!(result, expected);
```
 */
fn get_moves(coordination: (i32, i32)) -> Vec<&'static str> {
    let (x, y) = coordination;
    let mut moves = vec!["K", "G", "D", "B"];
    if x - 1 < 0 {
        moves.remove(3);
    }

    if x + 1 > 3 {
        moves.remove(2);
    }

    if y + 1 > 3 {
        moves.remove(1);
    }
    if y - 1 < 0 {
        moves.remove(0);
    }

    return moves;
}


/** Check if the user can move to the desired coordination in a 4*4 map

# Example
```rust
let position = (0,0);
let result = can_move(position);
let expected = true;

assert_eq!(result, expected);
```
 */
fn can_move(desired_coordination: (i32, i32)) -> bool {
    let (x, y) = desired_coordination;

    return x >= 0 && x < 4 && y >= 0 && y < 4;
}

/** Get user's input
 */
fn get_input(coordination: (i32, i32)) -> String {
    let (x, y) = coordination;
    let moves = get_moves(coordination);

    println!("You are in x:{x},y:{y}");
    println!("You can move to: {:#?}! So where do you want to go? Q for exit!", moves);

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error: Unable to read user input");
    return String::from(input.to_uppercase().trim_end());
}

/** Game loop

Creates treasure and player location, get user input and move the player until it finds
the treasure or press Q
 */
fn main() {
    let _game_map = [[0, 0, 0, 0]; 4];
    let mut player_coordination = (0, 0);
    let treasure_coordination = (3, 3);

    loop {
        let direction = get_input(player_coordination);
        let (x, y) = player_coordination;

        if direction == "K" {

            // Can we move to K?
            if !can_move((x, y - 1)) {
                println!("Nope you can't go that way!");
                continue;
            }

            player_coordination.1 -= 1;
        }
        if direction == "G" {

            // Can we move to K?
            if !can_move((x, y + 1)) {
                println!("Nope you can't go that way!");
                continue;
            }

            player_coordination.1 += 1;
        }
        if direction == "D" {

            // Can we move to K?
            if !can_move((x + 1, y)) {
                println!("Nope you can't go that way!");
                continue;
            }

            player_coordination.0 += 1;
        }
        if direction == "B" {
            println!("You choose: {direction}");
            // Can we move to K?
            if !can_move((x - 1, y)) {
                println!("Nope you can't go that way!");
                continue;
            }

            player_coordination.0 -= 1;
        }
        if direction == "Q" {
            break;
        }


        if player_coordination == treasure_coordination {
            println!("You won!");
            break;
        }
    }
}