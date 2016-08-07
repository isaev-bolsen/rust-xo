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

    fn has_empty_cells (&self) -> bool
    {
        for row in self.field.iter()
        {
           for element in row.iter()
           {
                match *element
                {
                    Cell::E => return true, 
                    Cell::X => continue,
                    Cell::O => continue,
                }
           }
        }
        return false;
    }

    fn try_set_cell(&mut self, column : usize, row : usize, value : Cell) -> bool
    {
        self.field[row][column] = Cell::X;
        return true;
    }
}

fn main() 
{
    let mut field = Field::new(3);

    field.out();

    while field.has_empty_cells()
    {
        println!("Column?");
        let column = read_int();
        println!("Row?");
        let row = read_int();

        field.try_set_cell(column, row, Cell::X);
        field.out();
    }
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
