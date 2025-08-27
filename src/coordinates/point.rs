
/// Describe this struct.
/// 
/// # Fields
/// 
/// - `x` (`f32`) - Describe this field.
/// - `y` (`f32`) - Describe this field.
/// 
/// # Examples
/// 
/// ```
/// use crate::...;
/// 
/// let s = Points {
///     x: value,
///     y: value,
/// };
/// ```
#[derive(Debug)]
pub struct Points {
    x: f32,
    y: f32,
}

impl Points {
    /// Describe this function.
    /// 
    /// # Arguments
    /// 
    /// - `x` (`f32`) - Describe this parameter.
    /// - `y` (`f32`) - Describe this parameter.
    /// 
    /// # Returns
    /// 
    /// - `Self` - Describe the return value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::...;
    /// 
    /// let _ = new();
    /// ```
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }
    /// Describe this function.
    /// 
    /// # Arguments
    /// 
    /// - `&self` (`undefined`) - Describe this parameter.
    /// 
    /// # Returns
    /// 
    /// - `f32` - Describe the return value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::...;
    /// 
    /// let _ = distance();
    /// ```
    pub fn distance(&self) -> f32 {
        return ((self.x.powi(2) + self.y.powi(2)) as f32).sqrt();
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dist() {
        let x = 5.0;
        let n = Points::new(x, x.powi(2));
        let dist = n.distance();

        println!("{:?}", n);
        println!("{}", dist);

        assert_eq!(dist, 25.495098);
    }
}
