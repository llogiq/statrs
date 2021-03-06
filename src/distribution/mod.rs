//! Defines common interfaces for interacting with statistical distributions and provides
//! concrete implementations for a variety of distributions.

use rand::Rng;

pub use self::bernoulli::Bernoulli;
pub use self::beta::Beta;
pub use self::binomial::Binomial;
pub use self::chi::Chi;
pub use self::chi_squared::ChiSquared;
pub use self::discrete_uniform::DiscreteUniform;
pub use self::exponential::Exponential;
pub use self::gamma::Gamma;
pub use self::log_normal::LogNormal;
pub use self::normal::Normal;
pub use self::poisson::Poisson;
pub use self::students_t::StudentsT;
pub use self::triangular::Triangular;
pub use self::uniform::Uniform;
pub use self::weibull::Weibull;

mod bernoulli;
mod beta;
mod binomial;
mod chi;
mod chi_squared;
mod discrete_uniform;
mod exponential;
mod gamma;
mod log_normal;
mod normal;
mod poisson;
mod students_t;
mod triangular;
mod uniform;
mod weibull;

/// The `Distribution` trait is used to specify an interface
/// for sampling statistical distributions
pub trait Distribution<T> {
    /// Draws a random sample using the supplied random number generator
    /// # Examples
    ///
    /// A trivial implementation that just samples from the supplied
    /// random number generator
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate statrs;
    ///
    /// use rand::Rng;
    /// use statrs::distribution::Distribution;
    ///
    /// struct Foo;
    ///
    /// impl Distribution<f64> for Foo {
    ///     fn sample<R: Rng>(&self, r: &mut R) -> f64 {
    ///         r.next_f64()
    ///     }
    /// }
    ///
    /// # fn main() { }
    /// ```
    fn sample<R: Rng>(&self, r: &mut R) -> T;
}

/// The `Univariate` trait is used to specify an interface for univariate
/// distributions e.g. distributions that have a closed form cumulative distribution
/// function
pub trait Univariate<T, K>: Distribution<K> {
    /// Returns the cumulative distribution function calculated
    /// at `x` for a given distribution. May panic depending
    /// on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Univariate, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(0.5, n.cdf(0.5));
    /// ```
    fn cdf(&self, x: K) -> K;

    /// Returns the minimum value in the domain of a given distribution
    /// representable by a double-precision float. May panic depending on
    /// the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Univariate, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(0.0, n.min());
    /// ```
    fn min(&self) -> T;

    /// Returns the maximum value in the domain of a given distribution
    /// representable by a double-precision float. May panic depending on
    /// the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Univariate, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(1.0, n.max());
    /// ```
    fn max(&self) -> T;
}

/// The `Mean` trait specifies a distribution that has a closed form
/// solution for its mean(s)
pub trait Mean<T, K>: Distribution<K> {
    /// Returns the mean for a given distribution. May panic depending
    /// on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Mean, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(0.5, n.mean());
    /// ```
    fn mean(&self) -> T;
}

/// The `Variance` trait specifies a distribution that has a closed form solution for
/// its variance(s). Requires `Mean` since a closed form solution to
/// variance by definition requires a closed form mean.
pub trait Variance<T, K>: Mean<T, K> {
    /// Returns the variance for a given distribution. May panic depending
    /// on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Variance, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(1.0 / 12.0, n.variance());
    /// ```
    fn variance(&self) -> T;

    /// Returns the standard deviation for a given distribution. May panic depending
    /// on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Variance, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!((1f64 / 12f64).sqrt(), n.std_dev());
    /// ```
    fn std_dev(&self) -> T;
}

/// The `Entropy` trait specifies a distribution with a closed form solution
/// for its entropy
pub trait Entropy<T>: Distribution<T> {
    /// Returns the entropy for a given distribution. May panic depending
    /// on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Entropy, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(0.0, n.entropy());
    /// ```
    fn entropy(&self) -> T;
}

/// The `Skewness` trait specifies a distributions with a closed form solution
/// for its skewness(s)
pub trait Skewness<T, K>: Distribution<K> {
    /// Returns the skewness for a given distribution. May panic depending
    /// on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Skewness, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(0.0, n.skewness());
    /// ```
    fn skewness(&self) -> T;
}

/// The `Median` trait specifies a distribution with a closed form solution
/// for its median
pub trait Median<T>: Distribution<T> {
    /// Returns the median for a given distribution. May panic depending
    /// on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Median, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(0.5, n.median());
    /// ```
    fn median(&self) -> T;
}

/// The `Mode` trait specififies a distribution with a closed form solution
/// for its mode(s)
pub trait Mode<T, K>: Distribution<K> {
    /// Returns the mode for a given distribution. May panic depending on
    /// the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Mode, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(0.5, n.mode());
    /// ```
    fn mode(&self) -> T;
}

/// The `Continuous` trait extends the `Distribution`
/// trait and provides an interface for interacting with continuous
/// statistical distributions
///
/// # Remarks
///
/// All methods provided by the `Continuous` trait are unchecked, meaning
/// they can panic if in an invalid state or encountering invalid input
/// depending on the implementing distribution.
pub trait Continuous<T, K>: Distribution<K> {
    /// Returns the probability density function calculated at `x` for a given distribution.
    /// May panic depending on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Continuous, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(1.0, n.pdf(0.5));
    /// ```
    fn pdf(&self, x: T) -> K;

    /// Returns the log of the probability density function calculated at `x` for a given distribution.
    /// May panic depending on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Continuous, Uniform};
    ///
    /// let n = Uniform::new(0.0, 1.0).unwrap();
    /// assert_eq!(0.0, n.ln_pdf(0.5));
    /// ```
    fn ln_pdf(&self, x: T) -> K;
}

/// The `Discrete` trait extends the `Distribution`
/// trait and provides an interface for interacting with discrete
/// statistical distributions
///
/// # Remarks
///
/// All methods provided by the `Discrete` trait are unchecked, meaning
/// they can panic if in an invalid state or encountering invalid input
/// depending on the implementing distribution.
pub trait Discrete<T, K>: Distribution<K> {
    /// Returns the probability mass function calculated at `x` for a given distribution.
    /// May panic depending on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Discrete, Binomial};
    /// use statrs::prec;
    ///
    /// let n = Binomial::new(0.5, 10).unwrap();
    /// assert!(prec::almost_eq(n.pmf(5), 0.24609375, 1e-15));
    /// ```
    fn pmf(&self, x: T) -> K;

    /// Returns the log of the probability mass function calculated at `x` for a given distribution.
    /// May panic depending on the implementor.
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::distribution::{Discrete, Binomial};
    /// use statrs::prec;
    ///
    /// let n = Binomial::new(0.5, 10).unwrap();
    /// assert!(prec::almost_eq(n.ln_pmf(5), (0.24609375f64).ln(), 1e-15));
    /// ```
    fn ln_pmf(&self, x: T) -> K;
}
