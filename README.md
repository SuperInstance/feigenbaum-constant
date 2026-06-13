# feigenbaum-constant

**Numerical computation of the Feigenbaum constant δ ≈ 4.6692...** via period-doubling bifurcations in the logistic map. This crate implements bisection-based detection of bifurcation points and computes the universal ratio that governs the onset of chaos.

## Why It Matters

The Feigenbaum constant is one of mathematics' great discoveries: a **universal** number that appears whenever a system undergoes period-doubling route to chaos, regardless of the specific system. Whether you study population dynamics (logistic map), fluid turbulence (Rayleigh-Bénard convection), electronic circuits (Chua's circuit), or laser instabilities — the same ratio δ ≈ 4.6692 governs the spacing of bifurcation points.

Mitchell Feigenbaum discovered this in 1975 using an HP-65 calculator. The fact that δ is universal (system-independent) makes it a **new kind of physical constant**, alongside π and e — one that characterizes the transition from order to chaos itself.

This crate provides a clean, tested implementation of the numerical procedure:
1. Find successive bifurcation points a₁, a₂, a₃, ... of the logistic map
2. Compute ratios δ_n = (a_n − a_{n-1}) / (a_{n+1} − a_n)
3. δ_n converges to δ ≈ 4.669201609...

## How It Works

### The Logistic Map

$$x_{t+1} = a \cdot x_t (1 - x_t), \quad x_0 = 0.5, \quad a \in [0, 4]$$

The behavior depends on the parameter a:

| Range of a | Behavior | Period |
|-----------|----------|--------|
| [0, 1) | x → 0 | Fixed point |
| [1, 3) | x → (a−1)/a | Fixed point |
| [3, 1+√6) | Oscillates between 2 values | Period 2 |
| [1+√6, a_∞) | Period doubling: 4, 8, 16, ... | Period 2^n |
| [a_∞, 4] | Chaotic (with periodic windows) | Aperiodic |

Where a_∞ ≈ 3.5699... is the accumulation point.

### Bifurcation Detection via Bisection

For period 2^n, we need to find the parameter value a* where the orbit of period 2^(n-1) loses stability and a stable orbit of period 2^n is born. The algorithm:

1. Start with an interval [a_low, a_high] known to contain the bifurcation
2. Evaluate the logistic map for 1000 iterations (transient decay)
3. Iterate for 2^n more steps and check if the orbit has the expected period
4. Bisect until the interval is tight

**Complexity**: O(200 × (1000 + 2^n)) per bifurcation point. The 200 bisection iterations each require ~1000 + 2^n logistic map evaluations.

### Feigenbaum δ Computation

Given bifurcation points a_1 < a_2 < a_3 < ..., the ratio:

$$\delta_n = \frac{a_n - a_{n-1}}{a_{n+1} - a_n}$$

converges geometrically to δ:

| n | δ_n (approximate) |
|---|-------------------|
| 2 | 4.7514 |
| 3 | 4.6562 |
| 5 | 4.6686 |
| 8 | 4.6692 |
| ∞ | 4.669201609... |

The convergence is itself governed by the Feigenbaum α ≈ 2.5029 (the ratio of successive parameter-space widths).

### Universal Renormalization

The deeper reason for universality: the period-doubling operator T (a renormalization that doubles the period) has a fixed point in function space. The eigenvalue of the linearization at this fixed point is δ. This means:

$$\lim_{n \to \infty} \frac{a_n - a_{n-1}}{a_{n+1} - a_n} = \delta$$

for *any* unimodal map with a quadratic maximum (not just the logistic map).

## Quick Start

```rust
use feigenbaum_constant::{compute_feigenbaum_delta, logistic_map};

// Compute δ with 10 bifurcation levels
let delta = compute_feigenbaum_delta(10);
assert!((delta - 4.669).abs() < 0.1);
println!("Feigenbaum δ ≈ {:.4}", delta);

// Iterate the logistic map at a = 3.2 (period-2 regime)
let orbit = logistic_map(3.2, 0.5, 100);
// After transients, oscillates between ~0.513 and ~0.799
```

## API

### Functions
- `compute_feigenbaum_delta(iterations: usize) -> f64` — Compute δ via period-doubling bifurcations. More iterations → better accuracy. O(200 × 2^n) per level.
- `logistic_map(a: f64, x0: f64, steps: usize) -> Vec<f64>` — Iterate x_{t+1} = a·x_t·(1−x_t) for `steps` iterations. Returns the full orbit.

## Architecture Notes

This crate connects to the γ + η = C framework via dynamical systems theory:

- **γ** (gamma) = the set of stable periodic orbits (ordered dynamics)
- **η** (eta) = the set of chaotic orbits (disordered dynamics)
- **C** (constant) = the full bifurcation diagram [0, 4]

As the parameter a increases through the period-doubling cascade, the system transitions from γ to η. At a_∞, the transition is complete. The Feigenbaum δ quantifies the rate of this transition — it is the "speed limit" of chaos onset.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## References

1. Feigenbaum, M.J. (1978). "Quantitative Universality for a Class of Nonlinear Transformations." *Journal of Statistical Physics, 19(1).* — The original discovery paper.
2. Feigenbaum, M.J. (1979). "The Universal Metric Properties of Nonlinear Transformations." *Journal of Statistical Physics, 21(6).*
3. Strogatz, S.H. (2014). *Nonlinear Dynamics and Chaos,* 2nd ed. Westview Press. Chapter 10 (renormalization).
4. Cvitanović, P. (1989). *Universality in Chaos,* 2nd ed. Adam Hilger. — Collected papers on universality.
5. May, R.M. (1976). "Simple Mathematical Models with Very Complicated Dynamics." *Nature, 261.* — The logistic map in ecology.

## License

MIT
