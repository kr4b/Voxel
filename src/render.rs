pub trait Render {
    fn render(&self);

    fn update(&mut self, key: &str, delta: (f64, f64));
}