# Statistical Analysis of [Fastiron][1]

This repository contains code used for statistical analysis of the behavior
of [Fastiron][1], a Monte-Carlo particle transport code written in Rust.

## Usage

The program can be run like any cargo projects: 

```
cargo run --release
```

The executable does not take any argument through the command line. Instead,
prompts are used to fetch the necessary inputs. There are currently three 
supported computations:

- **Version comparison**: Simple relative differences computation between two timer
  reports. This is useful to have numbers quickly and easily. The presented 
  results are percentages and **positiveness / negativeness have meaning**. 
- **Correlation study**: Computes correlation coefficients between tallied events 
  and section lengths. The results are formatted in a `.dat` file that can be 
  visualized using the corresponding `gnuplot` scripts.
- **Scaling study**: Compiles data from a collection of timer to a `.dat` file
  that can be used to plot section lengths depending on total number of particles
  using a `gnuplot` script. Currently, only arithmetic progression is supported 
  for the scaling number of particle. Geometric progression support will be added 
  in the future, as well as a specific script to plot this data using a logarithmic
  scale.

### Comparison Study

### Benchmark Statistics

### Scaling Graph


## TO-DO

- Add full support for geometric progressions in number of particle scaling


## References

- Fastiron [repository][1]
- `gnuplot` heatmap [examples][2]
- Relative difference [definition][3]

[1]: https://github.com/cea-hpc/fastiron
[2]: https://gnuplot.sourceforge.net/demo/heatmaps.html
[3]: https://en.wikipedia.org/wiki/Relative_change_and_difference#Definition