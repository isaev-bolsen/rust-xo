
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


fn main() {

    let mut field=[[Cell::E;3];3];
    field[1][1]=Cell::X;
    field[1][2]=Cell::O;

    println!("Hello, world!");
}
