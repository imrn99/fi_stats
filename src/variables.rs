use std::iter::zip;

#[derive(Debug)]
pub struct FiniteDiscreteRV {
    pub values: Vec<f64>,
    pub mean: f64,
    pub variance: f64,
}

impl FiniteDiscreteRV {
    pub fn new(values: &[f64]) -> Self {
        let n_val = values.len() as f64;
        let val = values.to_vec();
        let mut mean = val.iter().sum();
        mean /= n_val;
        let mut var = val.iter().map(|xi| (xi - mean) * (xi - mean)).sum();
        var /= n_val;

        Self {
            values: val,
            mean,
            variance: var,
        }
    }

    pub fn n_val(&self) -> usize {
        self.values.len()
    }
}

pub fn covariance(x: &FiniteDiscreteRV, y: &FiniteDiscreteRV) -> f64 {
    assert_eq!(x.n_val(), y.n_val());
    let iter = zip(x.values.iter(), y.values.iter());
    let mut cov = iter.map(|(xi, yi)| (xi - x.mean) * (yi - y.mean)).sum();
    cov /= x.n_val() as f64;
    cov
}

pub fn correlation(x: &FiniteDiscreteRV, y: &FiniteDiscreteRV) -> f64 {
    let cov = covariance(x, y);
    cov / (x.variance * y.variance).sqrt()
}

#[derive(Default, Clone, Copy, Debug)]
pub struct SummarizedVariable {
    pub mean: f64,
    pub lowest: f64,
    pub highest: f64,
    pub total: f64,
}
