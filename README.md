# Rust implementation of Ray Tracing in One Weekend

This code for a ray tracer follows the book series [_Ray Tracing In One Weekend_](https://raytracing.github.io/) by Peter Shirley. Most of the chapters are implemented and added parallel processing as well as `obj` file rendering.

## Trying it out

After Rust is installed example scenes can be rendered with `make`:

```
SCENE=cornell make image
```

<p align="center">
    <img src="/out/one-weekend.png">
</p>

<p align="center">
    <img src="/out/cornell.png">
    <img src="/out/dragon.png">
</p>

<p align="center">
    <img src="/out/spheres.png">
    <img src="/out/teapot.png">
</p>
