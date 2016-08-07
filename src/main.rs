
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
    fn new(size: usize) -> Field
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
}

fn main() {

    let mut field=Field::new(3);
    field.field[1][1]=Cell::X;
    field.field[1][2]=Cell::O;

    field.out();
}
