// In board.rs
use std::fmt;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Wood, Brick, Sheep, Wheat, Ore, Desert
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resource::Wood => write!(f, "Wood"),
            Resource::Brick => write!(f, "Brick"),
            Resource::Sheep => write!(f, "Sheep"),
            Resource::Wheat => write!(f, "Wheat"),
            Resource::Ore => write!(f, "Ore"),
            Resource::Desert => write!(f, "Desert"),
        }
    }
}

fn remove_one(numbers: &[u8], to_remove: u8) -> Vec<u8> {
    let mut removed = false;
    numbers.iter()
        .filter_map(|&n| {
            if n == to_remove && !removed {
                removed = true;
                None
            } else {
                Some(n)
            }
        })
        .collect()
}


#[derive(Debug, Clone)]
pub struct Tile {
    pub resource: Resource,
    pub number: Option<u8>,
    pub adjacent_indices: Vec<usize>,
}

pub struct CatanBoard {
    pub tiles: Vec<Tile>,
}

impl CatanBoard {
    pub fn new() -> Self {
        let adjacency_map = create_adjacency_map();
        let mut tiles = Vec::with_capacity(19);
        
        // Initialize tiles with adjacency but no resources/numbers yet
        for adjacent in adjacency_map {
            tiles.push(Tile {
                resource: Resource::Desert, // temporary
                number: None,
                adjacent_indices: adjacent,
            });
        }
        
        CatanBoard { tiles }
    }
    
    pub fn generate_valid_board(&mut self) -> bool {
        println!("Starting board generation...");
        
        // First distribute resources randomly
        self.distribute_resources();
        println!("Resources distributed:");
        self.print_resources();
        
        // Then place numbers with constraints
        let desert_index = self.tiles.iter().position(|t| t.resource == Resource::Desert).unwrap();
        println!("Desert is at position: {}", desert_index);
        
        // Available numbers for distribution (Catan uses 2-12 excluding 7)
        let mut numbers = vec![2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12];
        numbers.shuffle(&mut rand::thread_rng());
        println!("Numbers to place: {:?}", numbers);
        
        // Start recursive placement
        let result = self.place_numbers_backtracking(0, &numbers, desert_index);
        
        if result {
            println!("Successfully generated board!");
            self.print_full_board();
        } else {
            println!("Failed to generate a valid board with the given constraints.");
        }
        
        result
    }
    
    fn distribute_resources(&mut self) {
        let mut resources = vec![
            Resource::Wood, Resource::Wood, Resource::Wood, Resource::Wood,
            Resource::Brick, Resource::Brick, Resource::Brick,
            Resource::Sheep, Resource::Sheep, Resource::Sheep, Resource::Sheep,
            Resource::Wheat, Resource::Wheat, Resource::Wheat, Resource::Wheat,
            Resource::Ore, Resource::Ore, Resource::Ore,
            Resource::Desert // 1 desert
        ];
        resources.shuffle(&mut rand::thread_rng());
        
        for (idx, resource) in resources.into_iter().enumerate() {
            self.tiles[idx].resource = resource;
        }
    }
    
    fn place_numbers_backtracking(&mut self, tile_idx: usize, available_numbers: &[u8], desert_idx: usize) -> bool {
        // Skip over the desert tile and already placed tiles
        if tile_idx >= self.tiles.len() {
            return true; // All tiles have been processed
        }
        if tile_idx == desert_idx || self.tiles[tile_idx].number.is_some() {
            return self.place_numbers_backtracking(tile_idx + 1, available_numbers, desert_idx);
        }
        
        println!("Placing number at tile {}, resource: {}", tile_idx, self.tiles[tile_idx].resource);
        
        // Try each available number
        for &num in available_numbers {
            if self.is_valid_number_placement(tile_idx, num) {
                // Place the number
                self.tiles[tile_idx].number = Some(num);
                println!("  Trying number {} at tile {}", num, tile_idx);
                
                // Create new available numbers list without the used number
                let new_available = remove_one(available_numbers, num);

                
                // Recurse to the next tile
                if self.place_numbers_backtracking(tile_idx + 1, &new_available, desert_idx) {
                    return true;
                }
                
                // Backtrack if placement doesn't work
                println!("  Backtracking from number {} at tile {}", num, tile_idx);
                self.tiles[tile_idx].number = None;
            } else {
                println!("  Number {} not valid at tile {}", num, tile_idx);
            }
        }
        
        println!("No valid placement for tile {}", tile_idx);
        false // No valid placement found
    }
    
    fn is_valid_number_placement(&self, tile_idx: usize, number: u8) -> bool {
        // Check if any adjacent tile has the same number or conflicting numbers
        for &adj_idx in &self.tiles[tile_idx].adjacent_indices {
            if let Some(adj_num) = self.tiles[adj_idx].number {
                // No same numbers adjacent
                if adj_num == number {
                    return false;
                }
                
                // No 6/8 adjacent
                if (number == 6 || number == 8) && (adj_num == 6 || adj_num == 8) {
                    return false;
                }
                
                // No 2/12 adjacent
                if (number == 2 || number == 12) && (adj_num == 2 || adj_num == 12) {
                    return false;
                }
            }
        }
        
        true
    }
    
    // Helper methods for printing
    pub fn print_resources(&self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            println!("Tile {}: {}", i, tile.resource);
        }
    }
    
    pub fn print_full_board(&self) {
        // Prints board in the same layout as seen in the game
        let layout = vec![
            vec![None, None, Some(0), Some(1), Some(2), None, None],
            vec![None, Some(3), Some(4), Some(5), Some(6), None, None],
            vec![Some(7), Some(8), Some(9), Some(10), Some(11)],
            vec![None, Some(12), Some(13), Some(14), Some(15), None, None],
            vec![None, None, Some(16), Some(17), Some(18), None, None],
        ];

        
        for row in &layout {
            for cell in row {
                if let Some(idx) = cell {
                    let tile = &self.tiles[*idx];
                    let num_str = match tile.number {
                        Some(n) => format!("{:2}", n),
                        None => "--".to_string()
                    };
        
                    print!(" {:<5}:{} ", tile.resource.to_string(), num_str);
                } else {
                    print!("            ");
                }
            }
            println!();
        }
        
    }
}

pub fn create_adjacency_map() -> Vec<Vec<usize>> {
    vec![
        vec![1, 3, 4],       // Tile 0 (top left)
        vec![0, 2, 4, 5],    // Tile 1
        vec![1, 5, 6],       // Tile 2 (top right)
        vec![0, 4, 7, 8],    // Tile 3
        vec![0, 1, 3, 5, 8, 9], // Tile 4
        vec![1, 2, 4, 6, 9, 10], // Tile 5
        vec![2, 5, 10, 11],  // Tile 6
        vec![3, 8, 12],      // Tile 7 (middle left)
        vec![3, 4, 7, 9, 12, 13], // Tile 8
        vec![4, 5, 8, 10, 13, 14], // Tile 9
        vec![5, 6, 9, 11, 14, 15], // Tile 10
        vec![6, 10, 15],     // Tile 11 (middle right)
        vec![7, 8, 13, 16],  // Tile 12
        vec![8, 9, 12, 14, 16, 17], // Tile 13
        vec![9, 10, 13, 15, 17, 18], // Tile 14
        vec![10, 11, 14, 18], // Tile 15
        vec![12, 13, 17],    // Tile 16 (bottom left)
        vec![13, 14, 16, 18], // Tile 17
        vec![14, 15, 17]     // Tile 18 (bottom right)
    ]
}