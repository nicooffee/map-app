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
            Dir::Up     => (x,y-1),
            Dir::Down   => (x,y+1),
            Dir::Left   => (x-1,y),
            Dir::Right  => (x+1,y)
        }
    }

    pub fn of_u16(self,pos: (u16,u16)) -> (u16,u16) {
        let (x,y) = pos;
        match self {
            Dir::Up     => (x,match y.checked_sub(1) {Some(res) => res, None => 0}),
            Dir::Down   => (x,y+1),
            Dir::Left   => (match x.checked_sub(1) {Some(res) => res, None => 0},y),
            Dir::Right  => (x+1,y)
        }
    }
}
