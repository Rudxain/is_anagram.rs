use std::collections::{BTreeMap, HashMap};

#[must_use]
const fn eq(a: &[usize], b: &[usize]) -> bool {
	if a.len() != b.len() {
		return false;
	}
	let mut i = 0;
	while i < a.len() {
		if a[i] != b[i] {
			return false;
		}
		i += 1;
	}
	true
}

/// Clear/Reset all the ints
const fn zeroed<const N: usize>(s: &mut [usize; N]) {
	let mut i = 0;
	while i < s.len() {
		s[i] = 0;
		i += 1;
	}
}

#[must_use]
pub const fn all_anagrams_c(seq: &[&[u8]]) -> bool {
	let Some(init) = seq.first() else {
		return true;
	};

	let mut main_counts = [0_usize; 0x100];
	let mut i = 0;
	while i < init.len() {
		main_counts[init[i] as usize] += 1;
		i += 1;
	}
	let main_counts = main_counts;

	let mut run_counts = [0_usize; 0x100];

	let mut i = 0;
	while i < seq.len() {
		// avoid copying a zeroed array
		zeroed(&mut run_counts); //run_counts.fill(0);

		let mut j = 0;
		while j < seq[i].len() {
			run_counts[seq[i][j] as usize] += 1;
			j += 1;
		}
		// avoid Copy, again
		if !eq(&main_counts, &run_counts) {
			return false;
		}
		i += 1;
	}
	true
}

// pub fn all_anagrams_nstd<'seq, 's, T: Ord + Eq>(seq: &'seq mut [&'s mut [T]]) -> bool {
// 	let Some(init) = seq.first_mut() else {
// 		return true;
// 	};
// 	init.sort();
// 	seq.all(|mut s| {
// 		if init.len() != s.len() {
// 			return false;
// 		}
// 		s.sort_unstable();
// 		init == s
// 	})
// }

#[must_use]
pub fn all_anagrams_b<T: Ord + Eq, I: IntoIterator<Item = Box<[T]>>>(it: I) -> bool {
	let mut it = it.into_iter();
	let Some(mut init) = it.next() else {
		return true;
	};
	init.sort_unstable();
	it.all(|mut s| {
		if init.len() != s.len() {
			return false;
		}
		s.sort_unstable();
		init == s
	})
}

#[must_use]
pub fn all_anagrams_bt<T: Ord, I: IntoIterator<Item = T>, II: IntoIterator<Item = I>>(
	it2: II,
) -> bool {
	let mut it2 = it2.into_iter();
	let Some(init) = it2.next() else {
		return true;
	};

	let mut main_counts: BTreeMap<T, usize> = BTreeMap::new();
	for x in init {
		main_counts.entry(x).and_modify(|c| *c += 1).or_insert(1);
	}
	let main_counts = main_counts;

	let mut run_counts: BTreeMap<T, usize> = BTreeMap::new();

	it2.all(|it| {
		run_counts.clear();
		for x in it {
			run_counts.entry(x).and_modify(|c| *c += 1).or_insert(1);
		}
		main_counts == run_counts
	})
}

#[must_use]
pub fn all_anagrams_v<T: Ord, I: IntoIterator<Item = T>, II: IntoIterator<Item = I>>(
	it2: II,
) -> bool {
	let mut it2 = it2.into_iter();
	let Some(init) = it2.next() else {
		return true;
	};

	let mut main_counts: Vec<(T, usize)> = Vec::new();
	for x in init {
		match main_counts.binary_search_by_key(&&x, |(k, _)| k) {
			Ok(i) => main_counts[i].1 += 1,
			Err(i) => main_counts.insert(i, (x, 1)),
		}
	}
	let main_counts = main_counts;

	let mut run_counts: Vec<(T, usize)> = Vec::new();

	it2.all(|it| {
		run_counts.clear();
		for x in it {
			match run_counts.binary_search_by_key(&&x, |(x, _)| x) {
				Ok(i) => run_counts[i].1 += 1,
				Err(i) => run_counts.insert(i, (x, 1)),
			}
		}
		main_counts == run_counts
	})
}

#[must_use]
pub fn all_anagrams_h<
	T: std::hash::Hash + Eq,
	I: IntoIterator<Item = T>,
	II: IntoIterator<Item = I>,
>(
	it2: II,
) -> bool {
	let mut it2 = it2.into_iter();
	let Some(init) = it2.next() else {
		return true;
	};

	let mut main_counts: HashMap<T, usize> = HashMap::new();
	for x in init {
		main_counts.entry(x).and_modify(|c| *c += 1).or_insert(1);
	}
	let main_counts = main_counts;

	let mut run_counts: HashMap<T, usize> = HashMap::new();

	it2.all(|it| {
		run_counts.clear();
		for x in it {
			run_counts.entry(x).and_modify(|c| *c += 1).or_insert(1);
		}
		main_counts == run_counts
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
