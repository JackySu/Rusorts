// #![allow(dead_code)]
#![allow(unused)]

const DEBUG_INSERTION_SORT_THRESHOLD: usize = 3;
const RELEASE_INSERTION_SORT_THRESHOLD: usize = 27;
const QUICKSORT_STACK_SIZE: usize = 64;


pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
	#[cfg(debug_assertions)]
	if arr.len() < DEBUG_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	#[cfg(not(debug_assertions))]
	if arr.len() < RELEASE_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	let pivot = arr.len() - 1;

	// partition
	let mut i = 0;
	for j in 0..pivot {
		if arr[j] <= arr[pivot] {
			arr.swap(i, j);
			i += 1;
		}
	}
	arr.swap(i, pivot);
	// if left part has more than 1 element
	if i > 1 {
		quick_sort(&mut arr[..=i - 1]);
	}
	// if right part has more than 1 element
	if i + 2 < arr.len() {
		quick_sort(&mut arr[i + 1..]);
	}

}

// In respect to 
// - https://github.com/veddan/rust-introsort/blob/master/src/sort.rs
// - https://github.com/rosacris/rust-doublepivot-quicksort/blob/master/src/lib.rs
pub fn double_pivot_quicksort<T: PartialOrd>(arr: &mut [T]) {
	let (left, right) = (0, arr.len() - 1);

	#[cfg(debug_assertions)]
	if arr.len() < DEBUG_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	#[cfg(not(debug_assertions))]
	if arr.len() < RELEASE_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	unsafe {
		// pivots
		let pivot1 : *mut T = &mut arr[left];
		let pivot2 : *mut T = &mut arr[right];
		
		// swap pivots if p1 > p2
		if *pivot1 > *pivot2 {
			arr.swap(left, right);
		}

		// partition indexes
		let mut less = left + 1;
		let mut greater = right - 1;

		// sorting
		let mut k = less;
		while k <= greater {
			if arr[k] <= *pivot1 {
					arr.swap(k, less);
					less = less + 1;
			}
			else {
				if arr[k] >= *pivot2 {
					// find the rightmost element less than pivot2
					while k < greater && arr[greater] > *pivot2 {
						greater = greater - 1;
					}
					// swap it with arr[k]
					arr.swap(k, greater);
					greater = greater - 1;

					// if the swapped element is less than pivot1
					// then swap it with arr[less]
					if arr[k] <= *pivot1 {
						arr.swap(k, less);
						less = less + 1;
					}
				}
			}
			k = k + 1;
		}
		
		arr.swap(less - 1, left);
		arr.swap(greater + 1, right);

		if less > left + 2 {
			double_pivot_quicksort(&mut arr[left..=less - 2]);
		}

		if greater + 2 < right {
			double_pivot_quicksort(&mut arr[greater + 2..=right]);
		}

		if less < greater && *pivot1 < *pivot2 { // some elements are equal to pivot1 or pivot2
			double_pivot_quicksort(&mut arr[less..=greater]);
		}
	}
}

pub fn triple_pivot_quicksort<T: PartialOrd + Copy>(arr: &mut [T]) {
	let (left, right) = (0, arr.len() - 1);
	
	#[cfg(debug_assertions)]
	if arr.len() < DEBUG_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	#[cfg(not(debug_assertions))]
	if arr.len() < RELEASE_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	unsafe {

		let pivot1: *mut T = &mut arr[left];
		// let pivot2: *mut T = &mut arr[(left + right) / 2];
		let pivot3: *mut T = &mut arr[right];

		// if *pivot1 > *pivot2 { arr.swap(left, (left + right) / 2); }
		if *pivot1 > *pivot3 { arr.swap(left, right); }
		// if *pivot2 > *pivot3 { arr.swap((left + right) / 2, right); }

		let pivot1_val = *pivot1.clone();
		// let pivot2_val = *pivot2.clone();
		let pivot3_val = *pivot3.clone();
		// partition indexes
		let mut less = left + 1;
		let mut greater = right - 1;

		// sorting
		let mut k = less;
		while k <= greater {
			if arr[k] <= pivot1_val {
					arr.swap(k, less);
					less = less + 1;
			}
			else {
				if arr[k] >= pivot3_val {
					while k < greater && arr[greater] > pivot3_val {
						greater = greater - 1;
					}
					// swap it with arr[k]
					arr.swap(k, greater);
					greater = greater - 1;

					// if the swapped element is less than pivot1
					// then swap it with arr[less]
					if arr[k] <= pivot1_val {
						arr.swap(k, less);
						less = less + 1;
					}
				}
			}
			k = k + 1;
		}
		
		arr.swap(less - 1, left);
		arr.swap(greater + 1, right);

		if less > left + 2 {
			triple_pivot_quicksort(&mut arr[left..=less - 2]);
		}

		if greater + 2 < right {
			triple_pivot_quicksort(&mut arr[greater + 2..=right]);
		}

		if less < greater && pivot1_val < pivot3_val {
			let mut i = less;
			let pivot2_val = arr[greater];
			for j in less..greater {
				if arr[j] <= pivot2_val {
					arr.swap(i, j);
					i += 1;
				}
			}
			arr.swap(i, greater);
			// i > less + 1 and i + 1 < greater to enter insertion sort branch
			if i > less + 1 {
				triple_pivot_quicksort(&mut arr[less..=i - 1]);
			}
			if i + 1 < greater {
				triple_pivot_quicksort(&mut arr[i + 1..=greater]);
			}
		}

	}
}
