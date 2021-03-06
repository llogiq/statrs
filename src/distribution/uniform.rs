use std::f64;
use rand::Rng;
use rand::distributions::{Sample, IndependentSample};
use error::StatsError;
use result::Result;
use super::*;

/// Implements the [Continuous Uniform](https://en.wikipedia.org/wiki/Uniform_distribution_(continuous)) distribution
///
/// # Examples
///
/// ```
/// use statrs::distribution::{Uniform, Mean, Continuous};
///
/// let n = Uniform::new(0.0, 1.0).unwrap();
/// assert_eq!(n.mean(), 0.5);
/// assert_eq!(n.pdf(0.5), 1.0);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Uniform {
    min: f64,
    max: f64,
}

impl Uniform {
    /// Constructs a new uniform distribution with a min of `min` and a max
    /// of `max`
    ///
    /// # Errors
    ///
    /// Returns an error if `min` or `max` are `NaN`
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::Uniform;
    /// use std::f64;
    ///
    /// let mut result = Uniform::new(0.0, 1.0);
    /// assert!(result.is_ok());
    ///
    /// result = Uniform::new(f64::NAN, f64::NAN);
    /// assert!(result.is_err());
    /// ```
    pub fn new(min: f64, max: f64) -> Result<Uniform> {
        if min > max || min.is_nan() || max.is_nan() {
            Err(StatsError::BadParams)
        } else {
            Ok(Uniform {
                min: min,
                max: max,
            })
        }
    }
}

impl Sample<f64> for Uniform {
    /// Generate a random sample from a continuous uniform
    /// distribution using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details
    fn sample<R: Rng>(&mut self, r: &mut R) -> f64 {
        super::Distribution::sample(self, r)
    }
}

impl IndependentSample<f64> for Uniform {
    /// Generate a random independent sample from a continuous uniform
    /// distribution using `r` as the source of randomness.
    /// Refer [here](#method.sample-1) for implementation details
    fn ind_sample<R: Rng>(&self, r: &mut R) -> f64 {
        super::Distribution::sample(self, r)
    }
}

impl Distribution<f64> for Uniform {
    /// Generate a random sample from the continuous uniform distribution
    /// using `r` as the source of randomness in the range `[min, max]`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate statrs;
    /// use rand::StdRng;
    /// use statrs::distribution::{Uniform, Distribution};
    ///
    /// # fn main() {
    /// let mut r = rand::StdRng::new().unwrap();
    /// let n = Uniform::new(0.0, 5.0).unwrap();
    /// print!("{}", n.sample::<StdRng>(&mut r));
    /// # }
    /// ```
    fn sample<R: Rng>(&self, r: &mut R) -> f64 {
        r.gen_range(self.min, self.max + 1.0)
    }
}

impl Univariate<f64, f64> for Uniform {
    /// Calculates the cumulative distribution function for the uniform distribution
    /// at `x`
    ///
    /// # Remarks
    ///
    /// Returns `0.0` if `x < min` and `1.0` if `x >= max`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (x - min) / (max - min)
    /// ```
    fn cdf(&self, x: f64) -> f64 {
        if x <= self.min {
            return 0.0;
        }
        if x >= self.max {
            return 1.0;
        }
        (x - self.min) / (self.max - self.min)
    }

    fn min(&self) -> f64 {
        self.min
    }

    fn max(&self) -> f64 {
        self.max
    }
}

impl Mean<f64, f64> for Uniform {
    /// Returns the mean for the continuous uniform distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (min + max) / 2
    /// ```
    fn mean(&self) -> f64 {
        (self.min + self.max) / 2.0
    }
}

impl Variance<f64, f64> for Uniform {
    /// Returns the variance for the continuous uniform distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (max - min)^2 / 12
    /// ```
    fn variance(&self) -> f64 {
        (self.max - self.min) * (self.max - self.min) / 12.0
    }

    /// Returns the standard deviation for the continuous uniform distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// sqrt((max - min)^2 / 12)
    /// ```
    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl Entropy<f64> for Uniform {
    /// Returns the entropy for the continuous uniform distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln(max - min)
    /// ```
    fn entropy(&self) -> f64 {
        (self.max - self.min).ln()
    }
}

impl Skewness<f64, f64> for Uniform {
    /// Returns the skewness for the continuous uniform distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 0
    /// ```
    fn skewness(&self) -> f64 {
        0.0
    }
}

impl Median<f64> for Uniform {
    /// Returns the median for the continuous uniform distribution
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (min + max) / 2
    /// ```
    fn median(&self) -> f64 {
        (self.min + self.max) / 2.0
    }
}

impl Mode<f64, f64> for Uniform {
    /// Returns the mode for the continuous uniform distribution
    ///
    /// # Remarks
    ///
    /// Since every element has an equal probability, mode simply
    /// returns the middle element
    ///
    /// # Formula
    ///
    /// ```ignore
    /// N/A // (max + min) / 2 for the middle element
    /// ```
    fn mode(&self) -> f64 {
        (self.min + self.max) / 2.0
    }
}

impl Continuous<f64, f64> for Uniform {
    /// Calculates the probability density function for the continuous uniform
    /// distribution at `x`
    ///
    /// # Remarks
    ///
    /// Returns `0.0` if `x` is not in `[min, max]`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// 1 / (max - min)
    /// ```
    fn pdf(&self, x: f64) -> f64 {
        if x < self.min || x > self.max {
            0.0
        } else {
            1.0 / (self.max - self.min)
        }
    }

    /// Calculates the log probability density function for the continuous uniform
    /// distribution at `x`
    ///
    /// # Remarks
    ///
    /// Returns `f64::NEG_INFINITY` if `x` is not in `[min, max]`
    ///
    /// # Formula
    ///
    /// ```ignore
    /// ln(1 / (max - min))
    /// ```
    fn ln_pdf(&self, x: f64) -> f64 {
        if x < self.min || x > self.max {
            f64::NEG_INFINITY
        } else {
            -(self.max - self.min).ln()
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use std::f64;
    use distribution::*;

    fn try_create(min: f64, max: f64) -> Uniform {
        let n = Uniform::new(min, max);
        assert!(n.is_ok());
        n.unwrap()
    }

    fn create_case(min: f64, max: f64) {
        let n = try_create(min, max);
        assert_eq!(n.min(), min);
        assert_eq!(n.max(), max);
    }

    fn bad_create_case(min: f64, max: f64) {
        let n = Uniform::new(min, max);
        assert!(n.is_err());
    }

    fn test_case<F>(min: f64, max: f64, expected: f64, eval: F)
        where F: Fn(Uniform) -> f64
    {

        let n = try_create(min, max);
        let x = eval(n);
        assert_eq!(expected, x);
    }

    fn test_almost<F>(min: f64, max: f64, expected: f64, acc: f64, eval: F)
        where F: Fn(Uniform) -> f64
    {

        let n = try_create(min, max);
        let x = eval(n);
        assert_almost_eq!(expected, x, acc);
    }

    #[test]
    fn test_create() {
        create_case(0.0, 0.0);
        create_case(0.0, 0.1);
        create_case(0.0, 1.0);
        create_case(10.0, 10.0);
        create_case(-5.0, 11.0);
        create_case(-5.0, 100.0);
    }

    #[test]
    fn test_bad_create() {
        bad_create_case(f64::NAN, 1.0);
        bad_create_case(1.0, f64::NAN);
        bad_create_case(f64::NAN, f64::NAN);
        bad_create_case(1.0, 0.0);
    }

    #[test]
    fn test_variance() {
        test_case(-0.0, 2.0, 1.0 / 3.0, |x| x.variance());
        test_case(0.0, 2.0, 1.0 / 3.0, |x| x.variance());
        test_almost(0.1, 4.0, 1.2675, 1e-15, |x| x.variance());
        test_case(10.0, 11.0, 1.0 / 12.0, |x| x.variance());
        test_case(0.0, f64::INFINITY, f64::INFINITY, |x| x.variance());
    }

    #[test]
    fn test_std_dev() {
        test_case(-0.0, 2.0, (1f64 / 3.0).sqrt(), |x| x.std_dev());
        test_case(0.0, 2.0, (1f64 / 3.0).sqrt(), |x| x.std_dev());
        test_almost(0.1, 4.0, (1.2675f64).sqrt(), 1e-15, |x| x.std_dev());
        test_case(10.0, 11.0, (1f64 / 12.0).sqrt(), |x| x.std_dev());
        test_case(0.0, f64::INFINITY, f64::INFINITY, |x| x.std_dev());
    }

    #[test]
    fn test_entropy() {
        test_case(-0.0, 2.0, 0.6931471805599453094172, |x| x.entropy());
        test_case(0.0, 2.0, 0.6931471805599453094172, |x| x.entropy());
        test_almost(0.1, 4.0, 1.360976553135600743431, 1e-15, |x| x.entropy());
        test_case(1.0, 10.0, 2.19722457733621938279, |x| x.entropy());
        test_case(10.0, 11.0, 0.0, |x| x.entropy());
        test_case(0.0, f64::INFINITY, f64::INFINITY, |x| x.entropy());
    }

    #[test]
    fn test_skewness() {
        test_case(-0.0, 2.0, 0.0, |x| x.skewness());
        test_case(0.0, 2.0, 0.0, |x| x.skewness());
        test_case(0.1, 4.0, 0.0, |x| x.skewness());
        test_case(1.0, 10.0, 0.0, |x| x.skewness());
        test_case(10.0, 11.0, 0.0, |x| x.skewness());
        test_case(0.0, f64::INFINITY, 0.0, |x| x.skewness());
    }

    #[test]
    fn test_mode() {
        test_case(-0.0, 2.0, 1.0, |x| x.mode());
        test_case(0.0, 2.0, 1.0, |x| x.mode());
        test_case(0.1, 4.0, 2.05, |x| x.mode());
        test_case(1.0, 10.0, 5.5, |x| x.mode());
        test_case(10.0, 11.0, 10.5, |x| x.mode());
        test_case(0.0, f64::INFINITY, f64::INFINITY, |x| x.mode());
    }

    #[test]
    fn test_median() {
        test_case(-0.0, 2.0, 1.0, |x| x.median());
        test_case(0.0, 2.0, 1.0, |x| x.median());
        test_case(0.1, 4.0, 2.05, |x| x.median());
        test_case(1.0, 10.0, 5.5, |x| x.median());
        test_case(10.0, 11.0, 10.5, |x| x.median());
        test_case(0.0, f64::INFINITY, f64::INFINITY, |x| x.median());
    }

    #[test]
    fn test_pdf() {
        test_case(0.0, 0.0, 0.0, |x| x.pdf(-5.0));
        test_case(0.0, 0.0, f64::INFINITY, |x| x.pdf(0.0));
        test_case(0.0, 0.0, 0.0, |x| x.pdf(5.0));
        test_case(0.0, 0.1, 0.0, |x| x.pdf(-5.0));
        test_case(0.0, 0.1, 10.0, |x| x.pdf(0.05));
        test_case(0.0, 0.1, 0.0, |x| x.pdf(5.0));
        test_case(0.0, 1.0, 0.0, |x| x.pdf(-5.0));
        test_case(0.0, 1.0, 1.0, |x| x.pdf(0.5));
        test_case(0.0, 0.1, 0.0, |x| x.pdf(5.0));
        test_case(0.0, 10.0, 0.0, |x| x.pdf(-5.0));
        test_case(0.0, 10.0, 0.1, |x| x.pdf(1.0));
        test_case(0.0, 10.0, 0.1, |x| x.pdf(5.0));
        test_case(0.0, 10.0, 0.0, |x| x.pdf(11.0));
        test_case(-5.0, 100.0, 0.0, |x| x.pdf(-10.0));
        test_case(-5.0, 100.0, 0.009523809523809523809524, |x| x.pdf(-5.0));
        test_case(-5.0, 100.0, 0.009523809523809523809524, |x| x.pdf(0.0));
        test_case(-5.0, 100.0, 0.0, |x| x.pdf(101.0));
        test_case(0.0, f64::INFINITY, 0.0, |x| x.pdf(-5.0));
        test_case(0.0, f64::INFINITY, 0.0, |x| x.pdf(10.0));
        test_case(0.0, f64::INFINITY, 0.0, |x| x.pdf(f64::INFINITY));
    }

    #[test]
    fn test_ln_pdf() {
        test_case(0.0, 0.0, f64::NEG_INFINITY, |x| x.ln_pdf(-5.0));
        test_case(0.0, 0.0, f64::INFINITY, |x| x.ln_pdf(0.0));
        test_case(0.0, 0.0, f64::NEG_INFINITY, |x| x.ln_pdf(5.0));
        test_case(0.0, 0.1, f64::NEG_INFINITY, |x| x.ln_pdf(-5.0));
        test_almost(0.0, 0.1, 2.302585092994045684018, 1e-15, |x| x.ln_pdf(0.05));
        test_case(0.0, 0.1, f64::NEG_INFINITY, |x| x.ln_pdf(5.0));
        test_case(0.0, 1.0, f64::NEG_INFINITY, |x| x.ln_pdf(-5.0));
        test_case(0.0, 1.0, 0.0, |x| x.ln_pdf(0.5));
        test_case(0.0, 0.1, f64::NEG_INFINITY, |x| x.ln_pdf(5.0));
        test_case(0.0, 10.0, f64::NEG_INFINITY, |x| x.ln_pdf(-5.0));
        test_case(0.0, 10.0, -2.302585092994045684018, |x| x.ln_pdf(1.0));
        test_case(0.0, 10.0, -2.302585092994045684018, |x| x.ln_pdf(5.0));
        test_case(0.0, 10.0, f64::NEG_INFINITY, |x| x.ln_pdf(11.0));
        test_case(-5.0, 100.0, f64::NEG_INFINITY, |x| x.ln_pdf(-10.0));
        test_case(-5.0, 100.0, -4.653960350157523371101, |x| x.ln_pdf(-5.0));
        test_case(-5.0, 100.0, -4.653960350157523371101, |x| x.ln_pdf(0.0));
        test_case(-5.0, 100.0, f64::NEG_INFINITY, |x| x.ln_pdf(101.0));
        test_case(0.0, f64::INFINITY, f64::NEG_INFINITY, |x| x.ln_pdf(-5.0));
        test_case(0.0, f64::INFINITY, f64::NEG_INFINITY, |x| x.ln_pdf(10.0));
        test_case(0.0, f64::INFINITY, f64::NEG_INFINITY, |x| x.ln_pdf(f64::INFINITY));
    }

    #[test]
    fn test_cdf() {
        test_case(0.0, 0.0, 0.0, |x| x.cdf(-5.0));
        test_case(0.0, 0.0, 0.0, |x| x.cdf(0.0));
        test_case(0.0, 0.0, 1.0, |x| x.cdf(5.0));
        test_case(0.0, 0.1, 0.0, |x| x.cdf(-5.0));
        test_case(0.0, 0.1, 0.5, |x| x.cdf(0.05));
        test_case(0.0, 0.1, 1.0, |x| x.cdf(5.0));
        test_case(0.0, 1.0, 0.0, |x| x.cdf(-5.0));
        test_case(0.0, 1.0, 0.5, |x| x.cdf(0.5));
        test_case(0.0, 0.1, 1.0, |x| x.cdf(5.0));
        test_case(0.0, 10.0, 0.0, |x| x.cdf(-5.0));
        test_case(0.0, 10.0, 0.1, |x| x.cdf(1.0));
        test_case(0.0, 10.0, 0.5, |x| x.cdf(5.0));
        test_case(0.0, 10.0, 1.0, |x| x.cdf(11.0));
        test_case(-5.0, 100.0, 0.0, |x| x.cdf(-10.0));
        test_case(-5.0, 100.0, 0.0, |x| x.cdf(-5.0));
        test_case(-5.0, 100.0, 0.04761904761904761904762, |x| x.cdf(0.0));
        test_case(-5.0, 100.0, 1.0, |x| x.cdf(101.0));
        test_case(0.0, f64::INFINITY, 0.0, |x| x.cdf(-5.0));
        test_case(0.0, f64::INFINITY, 0.0, |x| x.cdf(10.0));
        test_case(0.0, f64::INFINITY, 1.0, |x| x.cdf(f64::INFINITY));
    }
}
