pub fn greets() {
	println!("Hi from `Init lib/bin commit.`")
}

pub mod vec_factory {

	/// Returns a vector of length n initialized with all zeros
	///
	/// # Examples
	///
	/// ```
	/// let vec = img2graph::vec_factory::create_i32_vec_zeros(3);
	/// assert_eq!(vec, vec![0; 3]);
	/// ```
	///
	/// # Panics
	///
	/// ```rust,should_panic
	/// let vec = img2graph::vec_factory::create_i32_vec_zeros(0);
	/// ```
	pub fn create_i32_vec_zeros(n: usize) -> Vec<i32> {
		if n == 0 {
			panic!("Cowardly refusing to create vector of length zero.");
		}
		vec![0; n]
	}

	pub fn create_i32_vec_range(n: usize) -> Vec<i32> {
		let vec: Vec<i32> = (0..n as i32).collect();
		vec
	}

	pub fn create_f32_vec_random(n: usize) -> Vec<f32> {
		use xorshift::{Rand, Rng, SeedableRng, SplitMix64, Xorshift128};

		let mut sm: SplitMix64 = SeedableRng::from_seed(42);
		let mut rng: Xorshift128 = Rand::rand(&mut sm);
		rng.gen_iter::<f32>().take(n).collect::<Vec<f32>>()
	}

	#[allow(unused_mut)]
	#[allow(unused_variables)]
	pub fn create_f32_vec_random_min_max(n: usize, min: f32, max: f32) -> Vec<f32> {
		use rand::distributions::Uniform;
		use rand::{thread_rng, Rng};

		let mut rng = thread_rng();
		let range = Uniform::new(min, max);

		// From slowest to fastest... ;-)
		// let vec: Vec<f32> = (0..n as i32).map(|_| rng.gen_range(min, max)).collect();
		// let vec: Vec<f32> = (0..n as i32).map(|_| rng.sample(&range)).collect();
		let vec: Vec<f32> = rng.sample_iter(&range).take(n).collect();
		vec
	}
}
