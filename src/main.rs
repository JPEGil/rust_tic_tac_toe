use std::fmt;
use std::io::stdin;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Square {
    EMPTY,
    X,
    O,
}

struct Grid([[Square; 3]; 3]);

fn main() {
    let mut turn = Square::X;
    let default_grid: [[Square; 3]; 3] = [[Square::EMPTY; 3]; 3];
    let mut grid: Grid = Grid(default_grid);

    let mut row_str = String::from("");
    let mut col_str = String::from("");

    let input = stdin();

    let mut won = false;
    let mut winner: Square = Square::EMPTY;

    while won == false {
        println!("{}", grid);
        println!("Enter Row: ");
        input.read_line(&mut row_str).expect("Error occured");

        println!("Enter Column");
        input.read_line(&mut col_str).expect("eror");

        let row: usize = row_str.trim().parse().expect("could not parse to number");
        let col: usize = col_str.trim().parse().expect("could not parse to number");

        row_str.clear();
        col_str.clear();

        if grid.0[row][col] != Square::EMPTY {
            println!("Theres an {} in that spot. Try Again", grid.0[row][col]);
            continue;
        }

        grid.0[row][col] = turn;

        change_turn(&mut turn);

        (won, winner) = has_won(&grid);

        //grid.0[1][1] = Square::X;
        //grid.0[2][0] = Square::O;
      
    }
    println!("{} has won", winner);
}

fn has_won(grid: &Grid) -> (bool, Square) {
    //row won
    for r in grid.0 {
        if (r[0] == r[1] && r[1] == r[2] && r[0] == r[2]) && r[0] != Square::EMPTY {
            return (true, r[0]);
        }
    }
    //col won
    for i in 0..2 {
        if (grid.0[0][i] == grid.0[1][i]
            && grid.0[1][i] == grid.0[2][i]
            && grid.0[0][i] == grid.0[2][i])
            && grid.0[0][i] != Square::EMPTY
        {
            return (true, grid.0[0][i]);
        }
    }
    //diag l -> r
    if (grid.0[0][0] == grid.0[1][1]
        && grid.0[1][1] == grid.0[2][2]
        && grid.0[0][0] == grid.0[2][2])
        && grid.0[0][0] != Square::EMPTY
    {
        return (true, grid.0[0][0]);
    }
    //diag r -> l
    if (grid.0[0][2] == grid.0[1][1]
        && grid.0[1][1] == grid.0[2][0]
        && grid.0[0][2] == grid.0[2][0])
        && grid.0[0][2] != Square::EMPTY
    {
        return (true, grid.0[0][2]);
    }
    return (false, Square::EMPTY);
}

fn change_turn(turn: &mut Square) {
    match turn {
        Square::X => *turn = Square::O,
        Square::O => *turn = Square::X,
        //Should never hapen, but rust compiler wont stop yelling at me unless i put this in
        Square::EMPTY => *turn = Square::X,
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = String::from("   0   1   2\n");
        let mut row_num = 0u8;
        for _r in self.0 {
            let mut row = String::from(format!("{}|", row_num));
            row_num += 1u8;
            for s in _r {
                match s {
                    Square::EMPTY => row += "   |",
                    Square::X => row += " X |",
                    Square::O => row += " O |",
                }
            }
            formatted += &(row + "\n");
        }
        write!(f, "{}", formatted)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Square::EMPTY => write!(f, " "),
            Square::X => write!(f, "X"),
            Square::O => write!(f, "O"),
        }
    }
}
