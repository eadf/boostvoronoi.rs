# boostvoronoi.rs
Boost 1.75.0 polygon::voronoi ported to 100% rust.
This implementaton of Forune's algorithm works on line segments as well as points, making it useful for finding centerlines or whatnot.

![Rusty voronoi](img.png)

Quick example:
```
set -x LIBRARY_PATH /opt/local/lib/ #or whatever you store your SDL
cargo run --example piston_gui
```

All credit goes to the original authors (Andrii Sydorchuk), except the porting mistakes. They are all mine.
