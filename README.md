# Very rough benchmarking of Rayon multi-threading
  
Needed approx. 10k iterations of `f64.sin()` to see `par_iter` outperform `iter`.
Beyond that, the more iterations, the larger the difference. 
The 'harder' the calc, the greater the difference in performance.  
  
For example, iterating over a vec with 100,000,000 elements;
```Rust
let a: Vec<f64> = vec.iter().map(|x| x.sin().sqrt().atan()).collect();
// => 3.192020145s
```
vs.
```Rust
let a: Vec<f64> = vec.par_iter().map(|x| x.sin().sqrt().atan()).collect();
// => 537.304763ms
```
= 6 times faster (release build, on i5 with 4 cores / 8 threads)  
  
In real example, would need to benchmark different versions (e.g. chain all ops, or split into some `iter` / some `par_iter` etc.), and will also depend on size of vec (as revealed here), as well as what else is running at the time.