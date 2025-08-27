

#[derive(Debug)]
pub struct Points{
    x: f32,
    y:f32,
}

impl Points{
    pub fn new(x:f32, y:f32) -> Self{ Self { x: x, y: y }}
    pub fn distance(&self) ->f32{
        return ((self.x.powi(2) + self.y.powi(2)) as f32).sqrt();
    }
}



#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_dist(){
        let x = 5.0;
        let n = Points::new(x, x.powi(2));

        println!("{:?}", n);

        assert_eq!(n.distance(),25.495098);
    }

}