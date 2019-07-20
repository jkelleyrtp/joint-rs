struct Position {
    x = usize,
    y = usize
}

impl Position {
    fn new(&self) -> Position {
        Position {
            x = 0;
            y = 0
        }
    }

    fn dist_from(&self, another_position: Position)  -> usize {


    }

    fn Eq (&self, another_position: Position) -> bool {

        ( ( (&self.x - another_position.x) == 0) && ( (&self.y - another_position.y) == 0 ) )
    } 



}




#[test]
fn test_position() {
    pos = Position { x: 0, y: 0}




}