# Raytracer

Raytracer written in Rust to practise the language.
Made using [this](https://raytracing.github.io/books/RayTracingInOneWeekend.html) step-by-step guide.

# How to run:

- Check `src/scenes.rs` and pick or create you own scene.
- Load the correct scene in `src/main.rs`
- Run `make`

# Optimization

| Benchmark             | three_balls | phone_cover    |
| --------------------- | ----------- | -------------- |
| Dev build             | 01:10       | Probably hours |
| Release build         | 00:02       | 04:19          |
| Parallelize per row   | 00:01       | 01:31          |
| Parallelize per pixel | 00:01       | 01:41          |

Parallelization is done using `rayon`.

# Reference

- https://raytracing.github.io/
