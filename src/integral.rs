use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Bound;
use std::ops::RangeBounds;

pub struct Function<F>
    where
        F: Fn(f64) -> f64,
{
    action: F,
}

impl<F> Function<F>
    where
        F: Fn(f64) -> f64,
{
    pub fn new(f: F) -> Self {
        Self { action: f }
    }
}

pub trait Integration {
    fn integrate<R>(&self, bounds: R, n: u32) -> Result<f64, UnboundedError>
        where
            R: RangeBounds<f64>;
    fn evaluate(&self, parameter: f64) -> f64;
}

impl<F> Integration for Function<F>
    where
        F: Fn(f64) -> f64,
{
    fn integrate<R>(&self, bounds: R, n: u32) -> Result<f64, UnboundedError>
        where
            R: RangeBounds<f64>,
    {
        match (bounds.start_bound(), bounds.end_bound()) {
            (Bound::Included(&a), Bound::Included(&b))
            | (Bound::Included(&a), Bound::Excluded(&b)) => {
                let h = (b - a) / n as f64;
                let (first, second) =
                    (1..n).fold((self.evaluate(a + h / 2.), 0.), |(im, imm), ele| {
                        (
                            im + self.evaluate(a + h * (ele as f64 + h / 2.)),
                            imm + self.evaluate(a + h * ele as f64),
                        )
                    });

                Ok(h / 6. * (self.evaluate(a) + self.evaluate(b) + 4. * first + 2. * second))
            }
            _ => Err(UnboundedError {}),
        }
    }
    fn evaluate(&self, parameter: f64) -> f64 {
        (self.action)(parameter)
    }
}

#[derive(Clone, Debug)]
pub struct UnboundedError {}

impl Error for UnboundedError {}

impl Display for UnboundedError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Unbounded ranges are not supported")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let func = Function::new(|x| x * 2.);
        let qs = func.integrate(0.0..=1.0, 100).unwrap();
        dbg!(qs);
    }
}
