mod Point;

use crate::Point::Points;

fn main(){
    let np = Points::new(21.1, 22.11);

    let f = np.distance();

    println!("{:?}", np);
    println!("{}", f);
}