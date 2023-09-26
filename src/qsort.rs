// #![allow(dead_code)]
#![allow(unused)]

const DEBUG_INSERTION_SORT_THRESHOLD: usize = 5;
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

		let mut mid = (left + right) / 2;
		let pivot1: *mut T = &mut arr[mid - 1];
		let pivot2: *mut T = &mut arr[mid];
		let pivot3: *mut T = &mut arr[right];

		if *pivot1 > *pivot2 { arr.swap(mid - 1, mid); }
		if *pivot1 > *pivot3 { arr.swap(mid - 1, right); }
		if *pivot2 > *pivot3 { arr.swap(mid, right); }

		let pivot1_val = *pivot1.clone();
		let pivot2_val = *pivot2.clone();
		let pivot3_val = *pivot3.clone();
		
		// partition indexes

		// |            left part          | mid |          right part          |
		// | partition 0 | ... | partition 1 | partition 2 | ... | partition 3 |

		// TODO: optimize the following code (turn them into loops or function calls)
		// ==========================================================================
		// put left part elements less than pivot1 into partition 0
		let mut pivot1_index = left;
		for i in left..mid - 1 {
			if arr[i] <= pivot1_val {
				arr.swap(i, pivot1_index);
				pivot1_index += 1;
			}
		}
		arr.swap(pivot1_index, mid - 1);

		// put right part elements greater than pivot3 on the rightmost side
		arr.swap(mid + 1, right);
		let mut pivot3_index = right;
		for i in (mid + 2..=right).rev() {
			if arr[i] >= pivot3_val {
				arr.swap(i, pivot3_index);
				pivot3_index -= 1;
			}
		}
		arr.swap(pivot3_index, mid + 1);
		// ==========================================================================
		
		let mut pivot2_index = mid;
		if pivot1_index + 2 < pivot3_index {
			arr.swap(pivot3_index - 1, mid);

			let mut l = pivot1_index + 1;
			for k in pivot1_index + 1..pivot3_index - 1 {
				if arr[k] <= pivot2_val {
					arr.swap(l, k);
					l += 1;
				}
			}
			arr.swap(l, pivot3_index - 1);
			pivot2_index = l;
		}

		// new mid as pivot2_index
		mid = pivot2_index;

		// partition arr[left..mid] and arr[mid + 1..right]
		// if mid == 0, then arr[left..mid] is empty
		// if mid == right - 1, then arr[mid + 1..right] is empty

		// partition arr[left..pivot2_index]
		// move pivot1 to the end of | partition 0 | ... | partition 1 |
		
		// DO NOT remove equal sign, otherwise it will cause error
		if pivot2_index >= left {
			arr.swap(mid - 1, pivot1_index);

			let mut i = left;
			for j in left..mid - 1 {
				if arr[j] <= pivot1_val {
					arr.swap(i, j);
					i += 1;
				}
			}
			arr.swap(i, mid - 1);
			pivot1_index = i;
		}
		// partition arr[pivot2_index + 1..right]

		// move pivot3 to the end of | partition 2 | ... | partition 3 |
		// DO NOT remove equal sign, otherwise it will cause error
		if pivot3_index <= right {
			arr.swap(right, pivot3_index);

			let mut i = mid + 1;
			for j in mid + 1..right {
				if arr[j] <= pivot3_val {	
					arr.swap(i, j);
					i += 1;
				}
			}
			arr.swap(i, right);
			pivot3_index = i;
		}

		// all the followings have to be xx + C < yy, C = 1, C > 1 will cause error 
		if pivot1_index > left + 1 {
			triple_pivot_quicksort(&mut arr[left..pivot1_index]);
		}

		if pivot1_index + 1 < pivot2_index {
			triple_pivot_quicksort(&mut arr[pivot1_index + 1..pivot2_index]);
		}

		if pivot2_index + 1 < pivot3_index {
			triple_pivot_quicksort(&mut arr[pivot2_index + 1..pivot3_index]);
		}

		if pivot3_index + 1 < right {
			triple_pivot_quicksort(&mut arr[pivot3_index + 1..=right]);
		}

	}
}
