struct Point{
    x: f32,
    y:f32
}
impl Point{
    fn new(x:f32, y:f32) -> Self{ Self { x: x, y: y }}
    fn distance(&self) ->f32{
        return ((self.x.powi(2) + self.y.powi(2)) as f32).sqrt();
    }
}

fn main(){

}