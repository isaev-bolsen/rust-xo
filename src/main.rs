use std::io;

enum Cell
{
    E,
    X,
    O,
}

struct Coord
{
    column : usize,
    row : usize,
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
}

fn main() 
{
    let mut field = Field::new(3);

    while field.has_empty_cells()
    {
        println!("Column?");
        let column = read_int();
        println!("Row?");
        let row=read_int();

    field.field[row][column] = Cell::X;
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
