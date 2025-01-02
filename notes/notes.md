## 1/2/24

I am now learning Rust!

- note that for the book Programming Rust (Blandy) we have each subdir corresponding to a chapter under src/
- each rust file within each subdir will be called mod.rs
- to run a program within a chapter subdir, modify the top of main.rs to import the mod file of the chapter you want to run by doing: mod ch2_a_tour_of_rust (for example)
  - this is how you link modules in rust
- also note that one cannot have a - or ; when naming a subdir
- to run a chapter's mod.rs file, do: cargo run
- using evcxr_jupyter, when making a new .ipynb file make sure to select the rust kernel
