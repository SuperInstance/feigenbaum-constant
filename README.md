# Feigenbaum Constant

The **Feigenbaum constant δ** (delta) is a universal constant ≈ 4.6692 that governs the rate at which period-doubling bifurcations converge to chaos in dynamical systems. This crate computes δ numerically via the **logistic map** `x_{n+1} = a·x_n·(1 − x_n)` and bisection-based bifurcation detection.

## Why It Matters

Discovered by Mitchell Feigenbaum in 1975, the constant δ is one of mathematics' most profound discoveries: **the ratio of successive bifurcation intervals is the same for all unimodal maps** — the logistic map, the sine map, and any smooth map with a single hump. This universality means that the onset of chaos follows a precise geometric law, independent of the specific system. It connects to fluid turbulence, heart arrhythmias, population dynamics, and the period-3 theorem of Li and Yorke. Computing δ numerically demonstrates how deterministic iteration produces both order and chaos.

## How It Works

### The Logistic Map

The map `f_a(x) = a·x·(1 − x)` maps `[0,1] → [0, a/4]`. For parameter `a ∈ [0, 4]`:

```
a ∈ [1, 3):        Fixed point (period 1)
a ≈ 3.0:           First bifurcation → period 2
a ≈ 3.449:         Second bifurcation → period 4
a ≈ 3.544:         Third bifurcation → period 8
...
a ≈ 3.5699:        Chaos onset (∞ period)
```

### Feigenbaum's Discovery

The bifurcation points `a_n` satisfy:

$$\delta = \lim_{n \to \infty} \frac{a_{n-1} - a_{n-2}}{a_n - a_{n-1}} \approx 4.6692\,01616\,029$$

### Algorithm

1. **Bisection** to find each bifurcation point `a_n`: iterate the map for `2^i` steps starting from `x = 0.5`, and check whether the trajectory returns to its starting value (period detection).
2. **Ratio estimation**: once two consecutive bifurcation points are known, compute δ from the ratio of differences.
3. **Newton-like update**: use the previous δ estimate to refine the search window for the next bifurcation.

### Complexity

Each bifurcation point requires O(200) bisection iterations × O(1000 + 2^i) map iterations. For `i` bifurcations: **O(i · 200 · 2^i)** time, **O(1)** space.

## Quick Start

```rust
// Compute δ from 10 period-doubling bifurcations
let delta = compute_feigenbaum_delta(10);
// delta ≈ 4.6692 (within ~0.1 tolerance)

// Iterate the logistic map at a specific parameter
let trajectory = logistic_map(3.2, 0.5, 100);
// At a=3.2, the system settles into a period-2 cycle
```

## API

| Function | Signature | Description |
|----------|-----------|-------------|
| `compute_feigenbaum_delta` | `fn(iterations: usize) → f64` | Compute δ via bisection-based bifurcation detection |
| `logistic_map` | `fn(a: f64, x0: f64, steps: usize) → Vec<f64>` | Iterate `x_{n+1} = a·x_n·(1−x_n)` |

## Architecture Notes

Part of the **SuperInstance** mathematical computing library. The Feigenbaum constant computation demonstrates the bridge between deterministic computation and emergent complexity — a core principle of the SuperInstance architecture where **γ + η = C**: γ (mathematical rigor) and η (numerical efficiency) combine to produce correct computational models of chaos.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for the full system design.

## References

1. Feigenbaum, M. J. "Quantitative Universality for a Class of Nonlinear Transformations." *Journal of Statistical Physics* 19, 25–52, 1978.
2. Strogatz, S. H. *Nonlinear Dynamics and Chaos*. Westview Press, 2nd ed., 2014. Chapter 10.
3. May, R. M. "Simple Mathematical Models with Very Complicated Dynamics." *Nature* 261, 459–467, 1976.

## License

MIT
