# Statistical Analysis of [Fastiron][1]

This repository contains code used for statistical analysis of the behavior
of [Fastiron][1], a Monte-Carlo particle transport code written in Rust.

## Usage

The program can be run like any other cargo projects: 

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

The user will be prompted first on which computations he wishes to do, only then 
will specific data be requested for processing.

### Comparison Study

The user will be asked to provide two timers `.csv` files, referred to as the 
_old_ one and the _new_ one (read _previous_ and _current_ in the context of 
versions). Percents will be computed using [this][3] definition and saved in 
a formatted MarkDown table `percents.md`.

### Benchmark Statistics

The user will be asked to provide a single tallies `.csv` file. From the values
of this file will be built random variables, each taking a value according to
the cycle index. Specific correlation coefficients are then computed to evaluate 
the influence of tallied event on the execution time of each main sections: 
`PopulationControl`, `CycleTracking` and `CycleSync`.

### Scaling Graph

The user will be asked to provide four parameters for the program to run correctly:

- The common root name of the multiple timers files
- The number of particles in the first simulation
- The step (common difference) defining the (arithmetic) progression
- The number of iteration, i.e. the number of samples

An example is given to the user at runtime to ensure clarity.

## TO-DO

- Add full support for geometric progressions in number of particle scaling.
- Add complete correlation matrix computation instead of the current specific ones.
- Add property check on random variables (notably in the case where variance is zero).
- Try to compile timers file for scaling into one and handle it accordingly?


## References

- Fastiron [repository][1]
- `gnuplot` heatmap [examples][2]
- Relative difference [definition][3]

[1]: https://github.com/cea-hpc/fastiron
[2]: https://gnuplot.sourceforge.net/demo/heatmaps.html
[3]: https://en.wikipedia.org/wiki/Relative_change_and_difference#Definition