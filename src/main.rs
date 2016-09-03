use std::io;

enum Cell
{
    E,
    X,
    O,
}

impl Copy for Cell {}
impl Clone for Cell {
     fn clone(&self) -> Cell { *self } 
}

struct Field
{
    field : Vec<Vec<Cell>>,
}

impl Field
{
    fn new (size: usize) -> Field
    {
        Field { field: vec![vec!(Cell::E;size); size] }
    }

    fn out (&self)
    {
       for row in self.field.iter()
       {
           for _ in row.iter() { print!("+---"); }
           println!("+");
           print!("|");
           for element in row.iter()
           {
                match *element
                {
                    Cell::E => print!("   |"),  
                    Cell::X => print!(" X |"),  
                    Cell::O => print!(" O |"),  
                }
           }
           println!("");
       }
       for _ in  self.field[0].iter() { print!("+---"); }
       println!("+");
    }

    fn try_set_cell(&mut self, column : usize, row : usize, value : Cell) -> bool
    {
        if column>=self.field[0].len() {return false;}
        if row>=self.field.len() {return false;}
        match self.field[row][column]
        {
            Cell::E => {
                self.field[row][column] = value; 
                return true;
            },
            Cell::X => return false,
            Cell::O => return false,
        }
    }

    fn winning_row(&mut self, player : Cell) -> i32
    {
        for row in self.field.iter()
        {

        }
        return -1;
    }
}

trait HasEmtyCells
{
    fn has_empty_cells (&self) -> bool;
}

impl HasEmtyCells for Vec<Cell>
{
    fn has_empty_cells (&self) -> bool
    {
        for element in self.iter()
        {
            match *element
            {
                Cell::E => return true, 
                Cell::X => continue,
                Cell::O => continue,
            }
        }
        return false;
    }
}

impl HasEmtyCells for Field
{
    fn has_empty_cells (&self) -> bool
    {
        for row in self.field.iter()
        {
            if row.has_empty_cells()
            {
                return true;
            }
        }
        return false;
    }
}

fn main() 
{
    let mut field = Field::new(3);
    let mut turn = Cell::X;

    while field.has_empty_cells()
    {
        field.out();

        println!("Column?");
        let column = read_int();
        println!("Row?");
        let row = read_int();

        if field.try_set_cell(column, row, turn)
        {
            println!("Move Accepted");
            match turn
            {
                Cell::E => panic!("Something is wrong! : turn of empty"),
                Cell::X => {turn = Cell::O; println!("Turn of O now")},
                Cell::O => {turn = Cell::X; println!("Turn of X now")},
            }
        }
    }

    field.out();
}

fn read_int() -> usize
{
    loop 
    {     
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}
