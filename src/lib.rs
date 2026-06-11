/// Compute the Feigenbaum constant δ ≈ 4.6692... via period-doubling bifurcations
pub fn compute_feigenbaum_delta(iterations: usize) -> f64 {
    let mut a_n = 1.0_f64;
    let mut a_n1 = 0.0_f64;
    let mut delta = 0.0_f64;
    for i in 1..=iterations {
        let a_new = a_n + (a_n - a_n1) / delta.max(1e-10);
        // Bisection to find the bifurcation point
        let mut a_low = a_n;
        let mut a_high = a_new * 2.0;
        for _ in 0..200 {
            let a_mid = (a_low + a_high) / 2.0;
            let period = 1 << i;
            let mut x = 0.5_f64;
            for _ in 0..1000 { x = a_mid * x * (1.0 - x); }
            let mut x_save = x;
            for _ in 0..period { x = a_mid * x * (1.0 - x); }
            if x > x_save { a_low = a_mid; } else { a_high = a_mid; }
        }
        let a_bif = (a_low + a_high) / 2.0;
        if i > 1 {
            let new_delta = (a_n1 - a_n) / (a_bif - a_n);
            if new_delta.is_finite() && new_delta.abs() > 0.1 {
                delta = new_delta;
            }
        }
        a_n1 = a_n;
        a_n = a_bif;
    }
    delta
}

/// Logistic map iteration
pub fn logistic_map(a: f64, x0: f64, steps: usize) -> Vec<f64> {
    let mut result = Vec::with_capacity(steps);
    let mut x = x0;
    for _ in 0..steps {
        x = a * x * (1.0 - x);
        result.push(x);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_feigenbaum() {
        let delta = compute_feigenbaum_delta(10);
        assert!((delta - 4.669).abs() < 0.1);
    }
    #[test]
    fn test_logistic() {
        let vals = logistic_map(3.2, 0.5, 100);
        assert!(vals.iter().all(|v| v >= &0.0 && v <= &1.0));
    }
}
