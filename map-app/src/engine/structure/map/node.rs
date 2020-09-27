use super::super::direction::Dir as D;
pub struct Node<S> {
    object: S,
    pos: (i32,i32),
    up: (i32,i32),
    down: (i32,i32),
    left: (i32,i32),
    right: (i32,i32)
}

impl<S> Node<S> {
    fn new(object: S,pos: (i32,i32),up: (i32,i32), down: (i32,i32), left: (i32,i32), right: (i32,i32)) -> Node<S>{
        Node{
            object: object,
            pos: pos,
            up: up,
            down: down,
            left: left,
            right: right
        }        
    }

    pub fn from_pos(object: S, pos: (i32,i32)) -> Node<S> {
        Node::new(object, pos, D::Up.of(pos), D::Down.of(pos), D::Left.of(pos), D::Right.of(pos))
    }

}

//Getter
impl<S> Node<S> {
    pub fn object(&self) -> &S {
        &self.object
    }
    pub fn pos(&self) -> (i32,i32) {
        self.pos
    }
    pub fn up(&self) -> (i32,i32) {
        self.up
    }
    pub fn down(&self) -> (i32,i32) {
        self.down
    }
    pub fn left(&self) -> (i32,i32) {
        self.left
    }
    pub fn right(&self) -> (i32,i32) {
        self.right
    }
}