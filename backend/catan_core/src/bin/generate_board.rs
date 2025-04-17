use catan_core::board::CatanBoard;

fn main() {
    println!("Catan Board Generator");
    println!("=====================");
    
    let mut board = CatanBoard::new();
    
    println!("Attempting to generate a valid board...");
    let success = board.generate_valid_board();
    
    if success {
        println!("\nGeneration successful!");
    } else {
        println!("\nFailed to generate a valid board with the given constraints.");
    }
}