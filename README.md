# Ray Tracing in Rust

This is a simple ray tracer written in Rust.
It is based on the book [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by
Peter Shirley and the follow-up books.

## Benchmark

* Image Resolution: `400 x 225`
* Number of Samples Per Pixel: `20`
* Depth of Ray Bounces: `50`
* Number of Objects: `484` (Spheres)

| Method               | Times (Sec) | Speedup |
|----------------------|-------------|---------|
| Single-threaded      | 381.24      | 1.0     |
| BVH                  | 101.07      | 3.77 x  |
| Multi-threaded       | 45.75       | 8.33 x  |
| Multi-threaded + BVH | 12.5        | 30.5 x  |
