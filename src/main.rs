#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_grid() {
        let grid = ::Grid::generate_empty(5, 13); // Generate empty grid (5x13)
        assert_eq!(grid.size, (5, 13)); // Assert that new grid gets its size set accordingly
        let mut number_of_squares = 0; 

        for square in &grid.squares { // checks that each square is not a being or block
            assert_eq!(square.ground, ::TerrainGround::Soil);
            assert_eq!(square.block, None);
            assert_eq!(square.beings, None);
            number_of_squares += 1; // Increments the counter
        }

        assert_eq!(grid.squares.len(), 5*13); // Should have this many
        assert_eq!(number_of_squares, 5*13); // Actually has this many
    }
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum TerrainGround {
    Soil, 
    Stone
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum TerrainBlock {
    Tree, 
    Soil, 
    Stone
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum Being {
    Orc,
    Human
}

struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>, // "This is a thing of Type T that might not exist"
    beings: Option<Being>
}

struct Grid {
    size: (usize, usize),
    squares: Vec<Square> // Vector<T> means a vector of things type T (In this case, a Generic)
}

impl Grid {
    fn generate_empty(size_x: usize, size_y: usize) -> Grid { // returns a grid
        let number_of_squares = size_x * size_y;
        
        /* create Vec that will hold grid squares via with_capacity method of Vec */
        /*  This pre allocates the vector to desried size */
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);
        
        /* Create all square structs and set ground, beings, and block value */
        for _ in 0..number_of_squares { // compute # of squares in grid
            squares.push(Square{ground: TerrainGround::Soil, block: None, beings: None});
        }
        
        /* Return the Grid */
        Grid {
            size: (size_x, size_y),
            squares: squares
        }
    }
}

fn main() {

}
