use std::f64;
use error::StatsError;
use super::*;

impl Statistics for [f64] {
    fn min(&self) -> f64 {
        if self.len() == 0 {
            return f64::NAN;
        }

        self.iter().fold(f64::INFINITY,
                         |acc, &x| if x < acc || x.is_nan() { x } else { acc })
    }

    fn max(&self) -> f64 {
        if self.len() == 0 {
            return f64::NAN;
        }

        self.iter().fold(f64::NEG_INFINITY,
                         |acc, &x| if x > acc || x.is_nan() { x } else { acc })
    }

    fn abs_min(&self) -> f64 {
        if self.len() == 0 {
            return f64::NAN;
        }

        self.iter()
            .map(|x| x.abs())
            .fold(f64::INFINITY,
                  |acc, x| if x < acc || x.is_nan() { x } else { acc })
    }

    fn abs_max(&self) -> f64 {
        if self.len() == 0 {
            return f64::NAN;
        }

        self.iter()
            .map(|x| x.abs())
            .fold(f64::NEG_INFINITY,
                  |acc, x| if x > acc || x.is_nan() { x } else { acc })
    }

    fn mean(&self) -> f64 {
        if self.len() == 0 {
            return f64::NAN;
        }

        let mut m = 0.0;
        self.iter()
            .fold(0.0, |acc, &x| {
                m += 1.0;
                acc + (x - acc) / m
            })
    }

    fn geometric_mean(&self) -> f64 {
        if self.len() == 0 {
            return f64::NAN;
        }

        (self.iter().fold(0.0, |acc, &x| if x < 0.0 { f64::NAN } else { acc + x.ln() }) /
         self.len() as f64)
            .exp()
    }

    fn harmonic_mean(&self) -> f64 {
        if self.len() == 0 {
            return f64::NAN;
        }

        self.len() as f64 /
        self.iter().fold(0.0,
                         |acc, &x| if x < 0.0 { f64::NAN } else { acc + 1.0 / x })
    }

    fn variance(&self) -> f64 {
        if self.len() <= 1 {
            return f64::NAN;
        }

        unsafe {
            let mut var = 0.0;
            let mut t = *self.get_unchecked(0);
            for i in 1..self.len() {
                let x = *self.get_unchecked(i);
                t += x;
                let diff = (i as f64 + 1.0) * x - t;
                var += (diff * diff) / ((i + 1) * i) as f64;
            }
            var / (self.len() - 1) as f64
        }
    }

    fn population_variance(&self) -> f64 {
        if self.len() == 0 {
            return f64::NAN;
        }

        unsafe {
            let mut var = 0.0;
            let mut t = *self.get_unchecked(0);
            for i in 1..self.len() {
                let x = *self.get_unchecked(i);
                t += x;
                let diff = (i as f64 + 1.0) * x - t;
                var += (diff * diff) / ((i + 1) * i) as f64
            }
            var / self.len() as f64
        }
    }

    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn population_std_dev(&self) -> f64 {
        self.population_variance().sqrt()
    }

    fn covariance(&self, other: &[f64]) -> f64 {
        let n1 = self.len();
        let n2 = other.len();
        assert!(n1 == n2,
                format!("{}", StatsError::ContainersMustBeSameLength));
        if n1 <= 1 {
            return f64::NAN;
        }

        let mean1 = self.mean();
        let mean2 = other.mean();
        self.iter()
            .zip(other.iter())
            .fold(0.0, |acc, x| acc + (x.0 - mean1) * (x.1 - mean2)) / (n1 - 1) as f64
    }

    fn population_covariance(&self, other: &[f64]) -> f64 {
        let n1 = self.len();
        let n2 = other.len();
        assert!(n1 == n2,
                format!("{}", StatsError::ContainersMustBeSameLength));
        if n1 == 0 {
            return f64::NAN;
        }

        let mean1 = self.mean();
        let mean2 = other.mean();
        self.iter()
            .zip(other.iter())
            .fold(0.0, |acc, x| acc + (x.0 - mean1) * (x.1 - mean2)) / n1 as f64
    }

    fn quadratic_mean(&self) -> f64 {
        if self.len() == 0 {
            return f64::NAN;
        }

        let mut m = 0.0;
        self.iter()
            .fold(0.0, |acc, &x| {
                m += 1.0;
                acc + (x * x - acc) / m
            })
            .sqrt()
    }

    /// Returns the order statistic `(order 1..N)` from the data
    ///
    /// # Remarks
    ///
    /// No sorting is assumed. Order must be one-based (between `1` and `N` inclusive).
    /// Returns `f64::NAN` if order is outside the viable range or data is empty.
    ///
    /// **NOTE:** This method works inplace for arrays and may cause the array to be reordered
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::statistics::Statistics;
    ///
    /// let mut x = [];
    /// assert!(x.order_statistic(1).is_nan());
    ///
    /// let mut y = [0.0, 3.0, -2.0];
    /// assert!(y.order_statistic(0).is_nan());
    /// assert!(y.order_statistic(4).is_nan());
    /// assert_eq!(y.order_statistic(2), 0.0);
    /// assert!(y != [0.0, 3.0, -2.0]);
    /// ```
    fn order_statistic(&mut self, order: usize) -> f64 {
        let n = self.len();
        match order {
            1 => self.min(),
            _ if order == n => self.max(),
            _ if order < 1 || order > n => f64::NAN,
            _ => select_inplace(self, order - 1),
        }
    }

    /// Returns the median value from the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty
    ///
    /// **NOTE:** This method works inplace for arrays and may cause the array to be reordered
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::statistics::Statistics;
    ///
    /// let mut x = [];
    /// assert!(x.median().is_nan());
    ///
    /// let mut y = [0.0, 3.0, -2.0];
    /// assert_eq!(y.median(), 0.0);
    /// assert!(y != [0.0, 3.0, -2.0]);
    /// ```
    fn median(&mut self) -> f64 {
        let k = self.len() / 2;
        if self.len() % 2 != 0 {
            select_inplace(self, k)
        } else {
            (select_inplace(self, k.saturating_sub(1)) + select_inplace(self, k)) / 2.0
        }
    }

    /// Estimates the tau-th quantile from the data. The tau-th quantile
    /// is the data value where the cumulative distribution function crosses tau.
    ///
    /// # Remarks
    ///
    /// No sorting is assumed. Tau must be between `0` and `1` inclusive.
    /// Returns `f64::NAN` if data is empty or tau is outside the inclusive range.
    ///
    /// **NOTE:** This method works inplace for arrays and may cause the array to be reordered
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::statistics::Statistics;
    ///
    /// let mut x = [];
    /// assert!(x.quantile(0.5).is_nan());
    ///
    /// let mut y = [0.0, 3.0, -2.0];
    /// assert!(y.quantile(-1.0).is_nan());
    /// assert!(y.quantile(2.0).is_nan());
    /// assert_eq!(y.quantile(0.5), 0.0);
    /// assert!(y != [0.0, 3.0, -2.0]);
    /// ```
    fn quantile(&mut self, tau: f64) -> f64 {
        if tau < 0.0 || tau > 1.0 || self.len() == 0 {
            return f64::NAN;
        }

        let h = (self.len() as f64 + 1.0 / 3.0) * tau + 1.0 / 3.0;
        let hf = h as i64;

        if hf <= 0 || tau == 0.0 {
            return self.min();
        }
        if hf >= self.len() as i64 || tau == 1.0 {
            return self.max();
        }

        let a = select_inplace(self, (hf as usize).saturating_sub(1));
        let b = select_inplace(self, hf as usize);
        a + (h - hf as f64) * (b - a)
    }

    /// Estimates the p-Percentile value from the data.
    ///
    /// # Remarks
    ///
    /// Use quantile for non-integer percentiles. `p` must be between `0` and `100` inclusive.
    /// Returns `f64::NAN` if data is empty or `p` is outside the inclusive range.
    ///
    /// **NOTE:** This method works inplace for arrays and may cause the array to be reordered
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::Statistics;
    ///
    /// let mut x = [];
    /// assert!(x.percentile(0).is_nan());
    ///
    /// let mut y = [1.0, 5.0, 3.0, 4.0, 10.0, 9.0, 6.0, 7.0, 8.0, 2.0];
    /// assert_eq!(y.percentile(0), 1.0);
    /// assert_eq!(y.percentile(50), 5.5);
    /// assert_eq!(y.percentile(100), 10.0);
    /// assert!(y.percentile(105).is_nan());
    /// assert!(y != [1.0, 5.0, 3.0, 4.0, 10.0, 9.0, 6.0, 7.0, 8.0, 2.0]);
    /// ```
    fn percentile(&mut self, p: usize) -> f64 {
        self.quantile(p as f64 / 100.0)
    }

    /// Estimates the first quartile value from the data.
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty
    ///
    /// **NOTE:** This method works inplace for arrays and may cause the array to be reordered
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate statrs;
    ///
    /// use statrs::statistics::Statistics;
    ///
    /// # fn main() {
    /// let mut x = [];
    /// assert!(x.lower_quartile().is_nan());
    ///
    /// let mut y = [2.0, 1.0, 3.0, 4.0];
    /// assert_almost_eq!(y.lower_quartile(), 1.416666666666666, 1e-15);
    /// assert!(y != [2.0, 1.0, 3.0, 4.0]);
    /// # }
    /// ```
    fn lower_quartile(&mut self) -> f64 {
        self.quantile(0.25)
    }

    /// Estimates the third quartile value from the data.
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty
    ///
    /// **NOTE:** This method works inplace for arrays and may cause the array to be reordered
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate statrs;
    ///
    /// use statrs::statistics::Statistics;
    ///
    /// # fn main() {
    /// let mut x = [];
    /// assert!(x.lower_quartile().is_nan());
    ///
    /// let mut y = [2.0, 1.0, 3.0, 4.0];
    /// assert_almost_eq!(y.upper_quartile(), 3.5833333333333333, 1e-15);
    /// assert!(y != [2.0, 1.0, 3.0, 4.0]);
    /// # }
    /// ```
    fn upper_quartile(&mut self) -> f64 {
        self.quantile(0.75)
    }

    /// Estimates the inter-quartile range from the data.
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty
    ///
    /// **NOTE:** This method works inplace for arrays and may cause the array to be reordered
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate statrs;
    ///
    /// use statrs::statistics::Statistics;
    ///
    /// # fn main() {
    /// let mut x = [];
    /// assert!(x.interquartile_range().is_nan());
    ///
    /// let mut y = [2.0, 1.0, 3.0, 4.0];
    /// assert_almost_eq!(y.interquartile_range(), 2.166666666666667, 1e-15);
    /// assert!(y != [2.0, 1.0, 3.0, 4.0]);
    /// # }
    /// ```
    fn interquartile_range(&mut self) -> f64 {
        self.upper_quartile() - self.lower_quartile()
    }

    /// Evaluates the rank of each entry of the data.
    ///
    /// # Remarks
    ///
    /// **NOTE:** This method works inplace for arrays and may cause the array to be reordered
    ///
    /// # Examples
    ///
    /// ```
    /// use statrs::statistics::{Statistics, RankTieBreaker};
    ///
    /// let mut x = [];
    /// assert_eq!(x.ranks(RankTieBreaker::Average).len(), 0);
    ///
    /// let y = [1.0, 3.0, 2.0, 2.0];
    /// assert_eq!((&mut y.clone()).ranks(RankTieBreaker::Average), [1.0, 4.0, 2.5, 2.5]);
    /// assert_eq!((&mut y.clone()).ranks(RankTieBreaker::Min), [1.0, 4.0, 2.0, 2.0]);
    /// ```
    fn ranks(&mut self, tie_breaker: RankTieBreaker) -> Vec<f64> {
        let n = self.len();
        let mut ranks: Vec<f64> = vec![0.0; n];
        let mut index: Vec<usize> = (0..n).collect();

        match tie_breaker {
            RankTieBreaker::First => {
                quick_sort_all(self, &mut *index, 0, n - 1);
                unsafe {
                    for i in 0..ranks.len() {
                        ranks[*index.get_unchecked(i)] = (i + 1) as f64;
                    }
                }
                ranks
            }
            _ => {
                sort(self, &mut *index);
                let mut prev_idx = 0;
                unsafe {
                    for i in 1..n {
                        if (*self.get_unchecked(i) - *self.get_unchecked(prev_idx)).abs() <= 0.0 {
                            continue;
                        }
                        if i == prev_idx + 1 {
                            ranks[*index.get_unchecked(prev_idx)] = i as f64;
                        } else {
                            handle_rank_ties(&mut *ranks,
                                             &*index,
                                             prev_idx as isize,
                                             i as isize,
                                             tie_breaker);
                        }
                        prev_idx = i;
                    }
                }

                handle_rank_ties(&mut *ranks,
                                 &*index,
                                 prev_idx as isize,
                                 n as isize,
                                 tie_breaker);
                ranks
            }
        }
    }
}

fn handle_rank_ties(ranks: &mut [f64],
                    index: &[usize],
                    a: isize,
                    b: isize,
                    tie_breaker: RankTieBreaker) {

    let rank = match tie_breaker {
        RankTieBreaker::Average => (b + a - 1) as f64 / 2.0 + 1.0,
        RankTieBreaker::Min => (a + 1) as f64,
        RankTieBreaker::Max => b as f64,
        RankTieBreaker::First => unreachable!(),
    };
    unsafe {
        for i in a..b {
            ranks[*index.get_unchecked(i as usize)] = rank
        }
    }
}

// Selection algorithm from Numerical Recipes
// See: https://en.wikipedia.org/wiki/Selection_algorithm
fn select_inplace(arr: &mut [f64], rank: usize) -> f64 {
    if rank == 0 {
        return arr.min();
    }
    if rank > arr.len() - 1 {
        return arr.max();
    }

    unsafe {
        let mut low = 0;
        let mut high = arr.len() - 1;
        loop {
            if high <= low + 1 {
                if high == low + 1 && *arr.get_unchecked(high) < *arr.get_unchecked(low) {
                    arr.swap(low, high)
                }
                return *arr.get_unchecked(rank);
            }

            let middle = (low + high) >> 1;
            arr.swap(middle, low + 1);

            if *arr.get_unchecked(low) > *arr.get_unchecked(high) {
                arr.swap(low, high);
            }
            if *arr.get_unchecked(low + 1) > *arr.get_unchecked(high) {
                arr.swap(low + 1, high);
            }
            if *arr.get_unchecked(low) > *arr.get_unchecked(low + 1) {
                arr.swap(low, low + 1);
            }

            let mut begin = low + 1;
            let mut end = high;
            let pivot = *arr.get_unchecked(begin);
            loop {
                loop {
                    begin += 1;
                    if *arr.get_unchecked(begin) >= pivot {
                        break;
                    }
                }
                loop {
                    end -= 1;
                    if *arr.get_unchecked(end) <= pivot {
                        break;
                    }
                }
                if end < begin {
                    break;
                }
                arr.swap(begin, end);
            }

            arr[low + 1] = *arr.get_unchecked(end);
            arr[end] = pivot;

            if end >= rank {
                high = end - 1;
            }
            if end <= rank {
                low = begin;
            }
        }
    }
}

// sorts a primary slice and re-orders the secondary slice automatically. Uses insertion sort on small
// containers and quick sorts for larger ones
fn sort(primary: &mut [f64], secondary: &mut [usize]) {
    assert!(primary.len() == secondary.len(),
            format!("{}", StatsError::ContainersMustBeSameLength));

    let n = primary.len();
    if n <= 1 {
        return;
    }
    if n == 2 {
        unsafe {
            if *primary.get_unchecked(0) > *primary.get_unchecked(1) {
                primary.swap(0, 1);
                secondary.swap(0, 1);
            }
            return;
        }
    }

    // insertion sort for really short containers
    if n <= 10 {
        unsafe {
            for i in 1..n {
                let key = *primary.get_unchecked(i);
                let item = *secondary.get_unchecked(i);
                let mut j = i as isize - 1;
                while j >= 0 && *primary.get_unchecked(j as usize) > key {
                    primary[j as usize + 1] = *primary.get_unchecked(j as usize);
                    secondary[j as usize + 1] = *secondary.get_unchecked(j as usize);
                    j -= 1;
                }
                primary[j as usize + 1] = key;
                secondary[j as usize + 1] = item;
            }
            return;
        }
    }

    quick_sort(primary, secondary, 0, n - 1);
}

// quick sorts a primary slice and re-orders the secondary slice automatically
fn quick_sort(primary: &mut [f64], secondary: &mut [usize], left: usize, right: usize) {
    assert!(primary.len() == secondary.len(),
            format!("{}", StatsError::ContainersMustBeSameLength));

    // shadow left and right for mutability in loop
    let mut left = left;
    let mut right = right;

    unsafe {
        loop {
            // Pivoting
            let mut a = left;
            let mut b = right;
            let p = a + ((b - a) >> 1);

            if *primary.get_unchecked(a) > *primary.get_unchecked(p) {
                primary.swap(a, p);
                secondary.swap(a, p);
            }
            if *primary.get_unchecked(a) > *primary.get_unchecked(b) {
                primary.swap(a, b);
                secondary.swap(a, b);
            }
            if *primary.get_unchecked(p) > *primary.get_unchecked(b) {
                primary.swap(p, b);
                secondary.swap(p, b);
            }

            let pivot = *primary.get_unchecked(p);

            // Hoare partitioning
            loop {
                while *primary.get_unchecked(a) < pivot {
                    a += 1;
                }
                while pivot < *primary.get_unchecked(b) {
                    b -= 1;
                }
                if a > b {
                    break;
                }
                if a < b {
                    primary.swap(a, b);
                    secondary.swap(a, b);
                }

                a += 1;
                b -= 1;

                if a > b {
                    break;
                }
            }

            // In order to limit recursion depth to log(n), sort the shorter
            // partition recursively and the longer partition iteratively.
            //
            // Must cast to isize as it's possible for left > b or a > right/
            // TODO: make this more robust
            if (b as isize - left as isize) <= (right as isize - a as isize) {
                if left < b {
                    quick_sort(primary, secondary, left, b);
                }
                left = a;
            } else {
                if a < right {
                    quick_sort(primary, secondary, a, right);
                }
                right = b;
            }

            if left >= right {
                break;
            }
        }
    }
}

// quick sorts a primary slice and re-orders the secondary slice automatically.
// Sorts secondarily by the secondary slice on primary key duplicates
fn quick_sort_all(primary: &mut [f64], secondary: &mut [usize], left: usize, right: usize) {
    assert!(primary.len() == secondary.len(),
            format!("{}", StatsError::ContainersMustBeSameLength));

    // shadow left and right for mutability in loop
    let mut left = left;
    let mut right = right;

    unsafe {
        loop {
            // Pivoting
            let mut a = left;
            let mut b = right;
            let p = a + ((b - a) >> 1);

            if *primary.get_unchecked(a) > *primary.get_unchecked(p) ||
               *primary.get_unchecked(a) == *primary.get_unchecked(p) &&
               *secondary.get_unchecked(a) > *secondary.get_unchecked(p) {

                primary.swap(a, p);
                secondary.swap(a, p);
            }
            if *primary.get_unchecked(a) > *primary.get_unchecked(b) ||
               *primary.get_unchecked(a) == *primary.get_unchecked(b) &&
               *secondary.get_unchecked(a) > *secondary.get_unchecked(b) {

                primary.swap(a, b);
                secondary.swap(a, b);
            }
            if *primary.get_unchecked(p) > *primary.get_unchecked(b) ||
               *primary.get_unchecked(p) == *primary.get_unchecked(b) &&
               *secondary.get_unchecked(p) > *secondary.get_unchecked(b) {

                primary.swap(p, b);
                secondary.swap(p, b);
            }

            let pivot1 = *primary.get_unchecked(p);
            let pivot2 = *secondary.get_unchecked(p);

            // Hoare partitioning
            loop {
                while *primary.get_unchecked(a) < pivot1 ||
                      *primary.get_unchecked(a) == pivot1 && *secondary.get_unchecked(a) < pivot2 {
                    a += 1;
                }
                while pivot1 < *primary.get_unchecked(b) ||
                      pivot1 == *primary.get_unchecked(b) && pivot2 < *secondary.get_unchecked(b) {
                    b -= 1;
                }
                if a > b {
                    break;
                }
                if a < b {
                    primary.swap(a, b);
                    secondary.swap(a, b);
                }

                a += 1;
                b -= 1;

                if a > b {
                    break;
                }
            }

            // In order to limit recursion depth to log(n), sort the shorter
            // partition recursively and the longer partition iteratively.
            //
            // Must cast to isize as it's possible for left > b or a > right/
            // TODO: make this more robust
            if (b as isize - left as isize) <= (right as isize - a as isize) {
                if left < b {
                    quick_sort_all(primary, secondary, left, b);
                }
                left = a;
            } else {
                if a < right {
                    quick_sort_all(primary, secondary, a, right);
                }
                right = b;
            }

            if left >= right {
                break;
            }
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use std::f64::{self, consts};
    use generate;
    use statistics::*;
    use testing;

    #[test]
    fn test_mean() {
        let mut data = testing::load_data("nist/lottery.txt");
        assert_almost_eq!((&data).mean(), 518.958715596330, 1e-12);

        data = testing::load_data("nist/lew.txt");
        assert_almost_eq!((&data).mean(), -177.435000000000, 1e-13);

        data = testing::load_data("nist/mavro.txt");
        assert_almost_eq!((&data).mean(), 2.00185600000000, 1e-15);

        data = testing::load_data("nist/michaelso.txt");
        assert_almost_eq!((&data).mean(), 299.852400000000, 1e-13);

        data = testing::load_data("nist/numacc1.txt");
        assert_eq!((&data).mean(), 10000002.0);

        data = testing::load_data("nist/numacc2.txt");
        assert_almost_eq!((&data).mean(), 1.2, 1e-15);

        data = testing::load_data("nist/numacc3.txt");
        assert_eq!((&data).mean(), 1000000.2);

        data = testing::load_data("nist/numacc4.txt");
        assert_almost_eq!((&data).mean(), 10000000.2, 1e-8);
    }

    #[test]
    fn test_std_dev() {
        let mut data = testing::load_data("nist/lottery.txt");
        assert_almost_eq!((&data).std_dev(), 291.699727470969, 1e-13);

        data = testing::load_data("nist/lew.txt");
        assert_almost_eq!((&data).std_dev(), 277.332168044316, 1e-12);

        data = testing::load_data("nist/mavro.txt");
        assert_almost_eq!((&data).std_dev(), 0.000429123454003053, 1e-15);

        data = testing::load_data("nist/michaelso.txt");
        assert_almost_eq!((&data).std_dev(), 0.0790105478190518, 1e-13);

        data = testing::load_data("nist/numacc1.txt");
        assert_eq!((&data).std_dev(), 1.0);

        data = testing::load_data("nist/numacc2.txt");
        assert_almost_eq!((&data).std_dev(), 0.1, 1e-16);

        data = testing::load_data("nist/numacc3.txt");
        assert_almost_eq!((&data).std_dev(), 0.1, 1e-10);

        data = testing::load_data("nist/numacc4.txt");
        assert_almost_eq!((&data).std_dev(), 0.1, 1e-9);
    }

    #[test]
    fn test_min_max_short() {
        let data = [-1.0, 5.0, 0.0, -3.0, 10.0, -0.5, 4.0];
        assert_eq!(data.min(), -3.0);
        assert_eq!(data.max(), 10.0);
    }

    #[test]
    fn test_order_statistic_short() {
        let mut data = [-1.0, 5.0, 0.0, -3.0, 10.0, -0.5, 4.0, 1.0, 6.0];
        assert!(data.order_statistic(0).is_nan());
        assert_eq!(data.order_statistic(1), -3.0);
        assert_eq!(data.order_statistic(2), -1.0);
        assert_eq!(data.order_statistic(3), -0.5);
        assert_eq!(data.order_statistic(7), 5.0);
        assert_eq!(data.order_statistic(8), 6.0);
        assert_eq!(data.order_statistic(9), 10.0);
        assert!(data.order_statistic(10).is_nan());
    }

    #[test]
    fn test_quantile_short() {
        let mut data = [-1.0, 5.0, 0.0, -3.0, 10.0, -0.5, 4.0, 0.2, 1.0, 6.0];
        assert_eq!(data.quantile(0.0), -3.0);
        assert_eq!(data.quantile(1.0), 10.0);
        assert_almost_eq!(data.quantile(0.5), 3.0 / 5.0, 1e-15);
        assert_almost_eq!(data.quantile(0.2), -4.0 / 5.0, 1e-15);
        assert_eq!(data.quantile(0.7), 137.0 / 30.0);
        assert_eq!(data.quantile(0.01), -3.0);
        assert_eq!(data.quantile(0.99), 10.0);
        assert_almost_eq!(data.quantile(0.52), 287.0 / 375.0, 1e-15);
        assert_almost_eq!(data.quantile(0.325), -37.0 / 240.0, 1e-15);
    }

    // TODO: need coverage for case where data.length > 10 to cover quick sort
    #[test]
    fn test_ranks() {
        let mut sorted_distinct = [1.0, 2.0, 4.0, 7.0, 8.0, 9.0, 10.0, 12.0];
        let mut sorted_ties = [1.0, 2.0, 2.0, 7.0, 9.0, 9.0, 10.0, 12.0];
        assert_eq!(sorted_distinct.ranks(RankTieBreaker::Average), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(sorted_ties.ranks(RankTieBreaker::Average), [1.0, 2.5, 2.5, 4.0, 5.5, 5.5, 7.0, 8.0]);
        assert_eq!(sorted_distinct.ranks(RankTieBreaker::Min), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(sorted_ties.ranks(RankTieBreaker::Min), [1.0, 2.0, 2.0, 4.0, 5.0, 5.0, 7.0, 8.0]);
        assert_eq!(sorted_distinct.ranks(RankTieBreaker::Max), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(sorted_ties.ranks(RankTieBreaker::Max), [1.0, 3.0, 3.0, 4.0, 6.0, 6.0, 7.0, 8.0]);
        assert_eq!(sorted_distinct.ranks(RankTieBreaker::First), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        assert_eq!(sorted_ties.ranks(RankTieBreaker::First), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);

        let distinct = [1.0, 8.0, 12.0, 7.0, 2.0, 9.0, 10.0, 4.0];
        let ties = [1.0, 9.0, 12.0, 7.0, 2.0, 9.0, 10.0, 2.0];
        assert_eq!(distinct.clone().ranks(RankTieBreaker::Average), [1.0, 5.0, 8.0, 4.0, 2.0, 6.0, 7.0, 3.0]);
        assert_eq!(ties.clone().ranks(RankTieBreaker::Average), [1.0, 5.5, 8.0, 4.0, 2.5, 5.5, 7.0, 2.5]);
        assert_eq!(distinct.clone().ranks(RankTieBreaker::Min), [1.0, 5.0, 8.0, 4.0, 2.0, 6.0, 7.0, 3.0]);
        assert_eq!(ties.clone().ranks(RankTieBreaker::Min), [1.0, 5.0, 8.0, 4.0, 2.0, 5.0, 7.0, 2.0]);
        assert_eq!(distinct.clone().ranks(RankTieBreaker::Max), [1.0, 5.0, 8.0, 4.0, 2.0, 6.0, 7.0, 3.0]);
        assert_eq!(ties.clone().ranks(RankTieBreaker::Max), [1.0, 6.0, 8.0, 4.0, 3.0, 6.0, 7.0, 3.0]);
        assert_eq!(distinct.clone().ranks(RankTieBreaker::First), [1.0, 5.0, 8.0, 4.0, 2.0, 6.0, 7.0, 3.0]);
        assert_eq!(ties.clone().ranks(RankTieBreaker::First), [1.0, 5.0, 8.0, 4.0, 2.0, 6.0, 7.0, 3.0]);
    }

    #[test]
    fn test_median_short() {
        let mut even = [-1.0, 5.0, 0.0, -3.0, 10.0, -0.5, 4.0, 0.2, 1.0, 6.0];
        assert_eq!(even.median(), 0.6);

        let mut odd = [-1.0, 5.0, 0.0, -3.0, 10.0, -0.5, 4.0, 0.2, 1.0];
        assert_eq!(odd.median(), 0.2);
    }

    #[test]
    fn test_median_long_constant_seq() {
        let mut even = vec![2.0; 100000];
        assert_eq!(2.0, even.median());

        let mut odd = vec![2.0; 100001];
        assert_eq!(2.0, odd.median());
    }

    #[test]
    fn test_mean_variance_stability() {
        // TODO: Implement tests. Depends on Mersenne Twister RNG implementation.
        // Currently hesistant to bring extra dependency just for test
    }

    #[test]
    fn test_covariance_consistent_with_variance() {
        let mut data = testing::load_data("nist/lottery.txt");
        assert_almost_eq!(data.variance(), data.covariance(&data), 1e-10);

        data = testing::load_data("nist/lew.txt");
        assert_almost_eq!(data.variance(), data.covariance(&data), 1e-10);

        data = testing::load_data("nist/mavro.txt");
        assert_almost_eq!(data.variance(), data.covariance(&data), 1e-10);

        data = testing::load_data("nist/michaelso.txt");
        assert_almost_eq!(data.variance(), data.covariance(&data), 1e-10);

        data = testing::load_data("nist/numacc1.txt");
        assert_almost_eq!(data.variance(), data.covariance(&data), 1e-10);
    }

    #[test]
    fn test_pop_covar_consistent_with_pop_var() {
        let mut data = testing::load_data("nist/lottery.txt");
        assert_almost_eq!(data.population_variance(), data.population_covariance(&data), 1e-10);

        data = testing::load_data("nist/lew.txt");
        assert_almost_eq!(data.population_variance(), data.population_covariance(&data), 1e-10);

        data = testing::load_data("nist/mavro.txt");
        assert_almost_eq!(data.population_variance(), data.population_covariance(&data), 1e-10);

        data = testing::load_data("nist/michaelso.txt");
        assert_almost_eq!(data.population_variance(), data.population_covariance(&data), 1e-10);

        data = testing::load_data("nist/numacc1.txt");
        assert_almost_eq!(data.population_variance(), data.population_covariance(&data), 1e-10);
    }

    #[test]
    fn test_covariance_is_symmetric() {
        let data_a = &testing::load_data("nist/lottery.txt")[0..200];
        let data_b = &testing::load_data("nist/lew.txt")[0..200];
        assert_eq!(data_a.covariance(data_b), data_b.covariance(data_a));
        assert_eq!(data_a.population_covariance(data_b), data_b.population_covariance(data_a));
    }

    #[test]
    fn test_empty_data_returns_nan() {
        let data = [0.0; 0];
        assert!(data.min().is_nan());
        assert!(data.max().is_nan());
        assert!(data.mean().is_nan());
        assert!(data.quadratic_mean().is_nan());
        assert!(data.variance().is_nan());
        assert!(data.population_variance().is_nan());
    }

    // TODO: test codeplex issue 5667 (Math.NET)

    // TODO: test github issue 136 (Math.NET)

    #[test]
    fn test_median_robust_on_infinities() {
        let mut data3 = [2.0, f64::NEG_INFINITY, f64::INFINITY];
        assert_eq!(data3.median(), 2.0);

        data3 = [f64::NEG_INFINITY, 2.0, f64::INFINITY];
        assert_eq!(data3.median(), 2.0);

        data3 = [f64::NEG_INFINITY, f64::INFINITY, 2.0];
        assert_eq!(data3.median(), 2.0);

        let mut data4 = [f64::NEG_INFINITY, 2.0, 3.0, f64::INFINITY];
        assert_eq!(data4.median(), 2.5);
    }

    #[test]
    fn test_large_samples() {
        let shorter = generate::periodic(4*4096, 4.0, 1.0);
        let longer = generate::periodic(4*32768, 4.0, 1.0);
        assert_almost_eq!(shorter.mean(), 0.375, 1e-14);
        assert_almost_eq!(longer.mean(), 0.375, 1e-14);
        assert_almost_eq!(shorter.quadratic_mean(), (0.21875f64).sqrt(), 1e-14);
        assert_almost_eq!(longer.quadratic_mean(), (0.21875f64).sqrt(), 1e-14);
    }

    #[test]
    fn test_quadratic_mean_of_sinusoidal() {
        let data = generate::sinusoidal(128, 64.0, 16.0, 2.0);
        assert_almost_eq!(data.quadratic_mean(), 2.0 / consts::SQRT_2, 1e-15);
    }
}
