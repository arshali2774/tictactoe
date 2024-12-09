use std::io;

fn main() {
    // 9 spaces to represent the board
    let mut board = [' ';9];
    // players array
    let players = ['X', 'O'];
    let mut turn = 0;
    print_board(board);
// Start an infinite loop that continues until explicitly broken
  loop { 
    // Prompt user for input
    println!("Enter position for '{}' or Type 'quit' to exit", players[turn]);
    // Get user input and store Result in index
    let index = get_index_from_input();
    // If there's an error, print it and continue the loop
   if let Err(e) = index {
    println!("{}", e);
    continue;
   }
   // If index is Ok, unwrap it to get the value
   // This converts the Result<Option<usize>, String> into Option<usize>
   let index = index.unwrap();
   // If index is None, break the loop
   if let None = index{
    break;
   }
   // If index is Some, unwrap it to get the value
   // This converts the Option<usize> into usize
   let index = index.unwrap();
   // check if the position is already taken
   if board[index] != ' ' {
    println!("Position {} already taken", index + 1);
    continue;
   }
   // Place players[turn] at the specified index on the board
   board[index] = players[turn];
   // Print the updated board
   print_board(board);
   turn = (turn + 1) % 2;
  }
}

fn print_board(board: [char; 9]) {
    println!("
    +---+---+---+
    | {} | {} | {} |
    +---+---+---+
    | {} | {} | {} |
    +---+---+---+
    | {} | {} | {} |
    +---+---+---+
    ", board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]);
}
// Function returns a Result that contains either:
// - Ok(Option<usize>): A valid board position or None for "quit"
// - Err(String): An error message if input is invalid
fn get_index_from_input() -> Result<Option<usize>, String> {
    // Create a new empty String to store user input
    let mut input = String::new();

    // Read a line from standard input (keyboard)
    // map_err converts any IO error to a String
    // ? operator will return early if there's an error
    let _res = io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;

    // Remove whitespace and newlines from both ends of the input
    let input = input.trim();

    // Check if user wants to quit
    if input == "quit" {
        return Ok(None);  // Return None wrapped in Ok
    }

    // Try to parse the input string into an unsigned integer
    // map_err provides a custom error message if parsing fails
    // ? operator returns early if parsing fails
    let index = input.parse::<usize>().map_err(|_e| format!("Invalid input: Input {} should be a number", input))?;

    // Validate that number is between 1 and 9
    if index < 1 || index > 9 { 
        return Err(format!("Invalid input: Input {} should be a number between 1 and 9", input));
    }

    // If all validation passes, return the index (converted to 0-based)
    // Some() because it's a valid move, wrapped in Ok because it's a success
    Ok(Some(index - 1))
}