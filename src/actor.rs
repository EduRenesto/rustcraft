pub trait Actor {
    fn update(&mut self);
    fn render(&self);
}
