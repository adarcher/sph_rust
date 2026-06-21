# sph_rust (future public)

Refactor of messy WIP to squash unrelated work.

TODO: 
 - incorporate solvers into interation
 - solver settings/setup file inputs
 - output generalized
   - Binary (standards and memory dump)
   - CSV (XLS template file)
   - MP4 (and/or other video format)
 - implement source/sinks correctly

# Install Rust

https://rust-lang.org/tools/install/

## Nightly Requirements

The nightly version may be required if I decide to use a new rust feature... which I think I'm already doing.

You can check by looking at the top of the `src/main.rs` file.  Lines look like:

```
// #![feature(vec_from_fn)]
// #![feature(sort_floats)]
```

These example lines trigger vector creation from iterators and float comparisions without tolerance restrictions.

# Clone & Build & Run

## Clone 

`git clone https://github.com/adarcher/sph_rust.git`

## Build

In the future, for optimal performance, options and settings will be compiled, and then run.

`cd sph_rust`

Release: 

`cargo build --release`

Debug:

`cargo build`

## Run

`cargo run --release`

### Arguments

Current minimal command line arguments:

`"Tree example with element (count, limit, size): (10000,102,10)"`

Example:

`cargo run 100000 100 2 --release`

Command line arguments:
**(removed/pruned and needed refactor)**

- count                  number of elements
- limit                  number of elements per lowest/leaf nodes
- size                   size of test domain
- (distribution          initial element distribution)
- (solver                H,MH,...)
- (options               solver options/settings)

