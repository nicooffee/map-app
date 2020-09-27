pub enum Dir {
    Up,
    Down,
    Left,
    Right
}


impl Dir {
    pub fn of(self,pos: (i32,i32)) -> (i32,i32) {
        let (x,y) = pos;
        match self {
            Dir::Up => (x,y-1),
            Dir::Down => (x,y+1),
            Dir::Left => (x-1,y),
            Dir::Right => (x+1,y)
        }
    }
}
