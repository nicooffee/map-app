pub struct Scene{
    width: u16,
    height: u16
}

impl Scene {
    pub fn new(width:u16,height:u16) -> Scene{
        if width < 1 || height < 1{
            panic!("Las dimensiones deben ser positivas! w:{} h:{}",width,height);
        }
        Scene{width:width,height:height}
    }
}
