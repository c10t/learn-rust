`$ cargo run --release --example benchmark -- 26`

```
sorting 67108864 integers (256.0 MB)
cpu info: 4 physical cores, 8 logical cores
seq_sort: sorted 67108864 integers in 26.1947198 seconds
par_sort: sorted 67108864 integers in 6.6930977 seconds
speed up: 3.91x
```
