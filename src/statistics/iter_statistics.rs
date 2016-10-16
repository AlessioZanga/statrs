use std::f64;
use std::borrow::Borrow;

/// The `IterStatistics` trait provides the same host of statistical
/// utilities as the `Statistics` traited ported for use with iterators
/// which requires a mutable borrow.
///
/// # Remarks
///
/// `min` and `max` are not implemented for this trait since the `Iterator`
/// trait already defines a `min` and ` max`
pub trait IterStatistics<T> {
    /// Returns the minimum absolute value in the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use statrs::statistics::IterStatistics;
    ///
    /// let x: Vec<f64> = vec![];
    /// assert!(x.iter().abs_min().is_nan());
    ///
    /// let y = [0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.iter().abs_min().is_nan());
    ///
    /// let z = [0.0, 3.0, -2.0];
    /// assert_eq!(z.iter().abs_min(), 0.0);
    /// ```
    fn abs_min(mut self) -> T;

    /// Returns the maximum absolute value in the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// use std::f64;
    /// use statrs::statistics::IterStatistics;
    ///
    /// let x: Vec<f64> = vec![];
    /// assert!(x.iter().abs_max().is_nan());
    ///
    /// let y = [0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.iter().abs_max().is_nan());
    ///
    /// let z = [0.0, 3.0, -2.0, -8.0];
    /// assert_eq!(z.iter().abs_max(), 8.0);
    /// ```
    fn abs_max(mut self) -> T;

    /// Evaluates the sample mean, an estimate of the population
    /// mean.
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate statrs;
    ///
    /// use std::f64;
    /// use statrs::statistics::IterStatistics;
    ///
    /// # fn main() {
    /// let x = [];
    /// assert!(x.iter().mean().is_nan());
    ///
    /// let y = [0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.iter().mean().is_nan());
    ///
    /// let z = [0.0, 3.0, -2.0];
    /// assert_almost_eq!(z.iter().mean(), 1.0 / 3.0, 1e-15);
    /// # }
    /// ```
    fn mean(mut self) -> T;

    /// Evaluates the geometric mean of the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`.
    /// Returns `f64::NAN` if an entry is less than `0`. Returns `0`
    /// if no entry is less than `0` but there are entries equal to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate statrs;
    ///
    /// use std::f64;
    /// use statrs::statistics::IterStatistics;
    ///
    /// # fn main() {
    /// let x: Vec<f64> = vec![];
    /// assert!(x.iter().geometric_mean().is_nan());
    ///
    /// let y = [0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.iter().geometric_mean().is_nan());
    ///
    /// let mut z = [0.0, 3.0, -2.0];
    /// assert!(z.iter().geometric_mean().is_nan());
    ///
    /// z = [0.0, 3.0, 2.0];
    /// assert_eq!(z.iter().geometric_mean(), 0.0);
    ///
    /// z = [1.0, 2.0, 3.0];
    /// // test value from online calculator, could be more accurate
    /// assert_almost_eq!(z.iter().geometric_mean(), 1.81712, 1e-5);
    /// # }
    /// ```
    fn geometric_mean(mut self) -> T;

    /// Evaluates the harmonic mean of the data
    ///
    /// # Remarks
    ///
    /// Returns `f64::NAN` if data is empty or an entry is `f64::NAN`, or if any value
    /// in data is less than `0`. Returns `0` if there are no values less than `0` but
    /// there exists values equal to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate statrs;
    ///
    /// use std::f64;
    /// use statrs::statistics::IterStatistics;
    ///
    /// # fn main() {
    /// let x = [];
    /// assert!(x.iter().harmonic_mean().is_nan());
    ///
    /// let y = [0.0, f64::NAN, 3.0, -2.0];
    /// assert!(y.iter().harmonic_mean().is_nan());
    ///
    /// let mut z = [0.0, 3.0, -2.0];
    /// assert!(z.iter().harmonic_mean().is_nan());
    ///
    /// z = [0.0, 3.0, 2.0];
    /// assert_eq!(z.iter().harmonic_mean(), 0.0);
    ///
    /// z = [1.0, 2.0, 3.0];
    /// // test value from online calculator, could be more accurate
    /// assert_almost_eq!(z.iter().harmonic_mean(), 1.63636, 1e-5);
    /// # }
    /// ```
    fn harmonic_mean(self) -> T;
}

impl<T> IterStatistics<f64> for T
    where T: Iterator,
          T::Item: Borrow<f64>
{
    fn abs_min(mut self) -> f64 {
        match self.next() {
            None => f64::NAN,
            Some(init) => {
                self.map(|x| x.borrow().abs())
                    .fold(init.borrow().abs(),
                          |acc, x| if x < acc || x.is_nan() { x } else { acc })
            }
        }
    }

    fn abs_max(mut self) -> f64 {
        match self.next() {
            None => f64::NAN,
            Some(init) => {
                self.map(|x| x.borrow().abs())
                    .fold(init.borrow().abs(),
                          |acc, x| if x > acc || x.is_nan() { x } else { acc })
            }
        }
    }

    fn mean(self) -> f64 {
        let mut count = 0.0;
        let mut mean = 0.0;
        for x in self {
            count += 1.0;
            mean += (x.borrow() - mean) / count;
        }
        if count > 0.0 { mean } else { f64::NAN }
    }

    fn geometric_mean(self) -> f64 {
        let mut count = 0.0;
        let mut sum = 0.0;
        for x in self {
            count += 1.0;
            sum += x.borrow().ln();
        }
        if count > 0.0 {
            (sum / count).exp()
        } else {
            f64::NAN
        }
    }

    fn harmonic_mean(self) -> f64 {
        let mut count = 0.0;
        let mut sum = 0.0;
        for x in self {
            count += 1.0;

            let val = *x.borrow();
            if val < 0f64 {
                return f64::NAN;
            }
            sum += 1.0 / val;
        }
        if count > 0.0 { count / sum } else { f64::NAN }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use statistics::IterStatistics;
    use testing;

    #[test]
    fn test_mean() {
        let mut data = testing::load_data("nist/lottery.txt");
        assert_almost_eq!(data.iter().mean(), 518.958715596330, 1e-12);

        data = testing::load_data("nist/lew.txt");
        assert_almost_eq!(data.iter().mean(), -177.435000000000, 1e-13);

        data = testing::load_data("nist/mavro.txt");
        assert_almost_eq!(data.iter().mean(), 2.00185600000000, 1e-15);

        data = testing::load_data("nist/michaelso.txt");
        assert_almost_eq!(data.iter().mean(), 299.852400000000, 1e-13);

        data = testing::load_data("nist/numacc1.txt");
        assert_eq!(data.iter().mean(), 10000002.0);

        data = testing::load_data("nist/numacc2.txt");
        assert_almost_eq!(data.iter().mean(), 1.2, 1e-15);

        data = testing::load_data("nist/numacc3.txt");
        assert_eq!(data.iter().mean(), 1000000.2);

        data = testing::load_data("nist/numacc4.txt");
        assert_almost_eq!(data.iter().mean(), 10000000.2, 1e-8);
    }
}
