
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
}

fn main() {

    let mut field=Field::new(3);
    field.field[1][1]=Cell::X;
    field.field[1][2]=Cell::O;

    println!("Hello, world!");
}
