//! This enables the `test` create without having to specify it in Cargo.toml
#![feature(test)]

extern crate img2graph;
extern crate test;


#[cfg(test)]
mod workout_bench {
  // use test::bench::black_box;
  use img2graph::vec_factory;
  use test::Bencher;

  const ITER: usize = 1 as usize;
  const SIZE: usize = 1e4 as usize;

  #[bench]
  fn create_zeroed_i32_vecs(bench: &mut Bencher) {
    bench.iter(|| {
      for _i in 1..=ITER {
        let _v: Vec<i32> = vec_factory::create_i32_vec_zeros(SIZE);
      }
    })
  }

  #[bench]
  fn create_ranged_i32_vecs(bench: &mut Bencher) {
    bench.iter(|| {
      for _i in 1..=ITER {
        let _v: Vec<i32> = vec_factory::create_i32_vec_range(SIZE);
      }
    })
  }

  #[bench]
  fn create_random_f32_vecs(bench: &mut Bencher) {
    bench.iter(|| {
      for _i in 1..=ITER {
        let _v: Vec<f32> = vec_factory::create_f32_vec_random(SIZE);
      }
    })
  }

  #[bench]
  fn create_random_f32_vecs_min_max(bench: &mut Bencher) {
    bench.iter(|| {
      for _i in 1..=ITER {
        let _v: Vec<f32> = vec_factory::create_f32_vec_random_min_max(SIZE, -32.3, 42.4);
      }
    })
  }
}
