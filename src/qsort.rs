// #![allow(dead_code)]
#![allow(unused)]

const DEBUG_INSERTION_SORT_THRESHOLD: usize = 5;
const RELEASE_INSERTION_SORT_THRESHOLD: usize = 27;
const QUICKSORT_STACK_SIZE: usize = 64;


#[inline(always)]
unsafe fn rotate3<T: Copy>(arr: &mut [T], a: usize, b: usize, c: usize) {
	let pa: *mut T = arr.get_unchecked_mut(a);
	let pb: *mut T = arr.get_unchecked_mut(b);
	let pc: *mut T = arr.get_unchecked_mut(c);
	let tmp = *pa;
	*pa = *pb;
	*pb = *pc;
	*pc = tmp;
}

pub fn quick_sort_lomuto_partition<T: PartialOrd + Copy>(arr: &mut [T]) {
	#[cfg(debug_assertions)]
	if arr.len() < DEBUG_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	#[cfg(not(debug_assertions))]
	if arr.len() < RELEASE_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	let (left, right) = (0, arr.len() - 1);
	let pivot = arr[right];
	let mut i = left;
	for j in left..right {
		if arr[j] <= pivot {
			arr.swap(i, j);
			i += 1;
		}
	}
	arr.swap(i, right);
	// if left part has more than 1 element
	if i > 1 {
		quick_sort_lomuto_partition(&mut arr[..=i - 1]);
	}
	// if right part has more than 1 element
	if i + 2 < arr.len() {
		quick_sort_lomuto_partition(&mut arr[i + 1..]);
	}
}

pub fn quick_sort_hoare_partition<T: PartialOrd + Copy>(arr: &mut [T]) {
	#[cfg(debug_assertions)]
	if arr.len() < DEBUG_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	#[cfg(not(debug_assertions))]
	if arr.len() < RELEASE_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	let pivot = arr[0];
	let mut i = -1;
	let mut j = arr.len() as i32;
	loop {
		i += 1;
		while arr[i as usize] < pivot {
			i += 1;
		}

		j -= 1;
		while arr[j as usize] > pivot {
			j -= 1;
		}

		if i >= j {
			break;
		}
		arr.swap(i as usize, j as usize);
	}

	if j > 1 {
		quick_sort_lomuto_partition(&mut arr[..=j as usize]);
	}
	if j + 2 < arr.len() as i32 {
		quick_sort_hoare_partition(&mut arr[(j + 1) as usize..]);
	}
}

// In respect to 
// - https://github.com/veddan/rust-introsort/blob/master/src/sort.rs
// - https://github.com/rosacris/rust-doublepivot-quicksort/blob/master/src/lib.rs
pub fn double_pivot_quicksort<T: PartialOrd + Clone>(arr: &mut [T]) {
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

pub fn triple_pivot_quicksort<T: PartialOrd + Clone + Copy>(arr: &mut [T]) {
	#[cfg(debug_assertions)]
	if arr.len() < DEBUG_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	#[cfg(not(debug_assertions))]
	if arr.len() < RELEASE_INSERTION_SORT_THRESHOLD {
		return arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
	}

	let (left, right) = (0, arr.len() - 1);
	
	unsafe {
		let p1: *mut T = &mut arr[left];
		let p2: *mut T = &mut arr[left + 1];
		let p3: *mut T = &mut arr[right];
	
		if *p1 > *p2 {
			arr.swap(left, left + 1);
		}
		if *p2 > *p3 {
			arr.swap(left + 1, right);
		}
		if *p1 > *p2 {
			arr.swap(left, left + 1);
		}

		let (mut i, mut j, mut k, mut l) = (left + 2, left + 2, right - 1, right - 1);
		let (p1, p2, p3) = (*p1, *p2, *p3);
		while j <= k {
			// j moves right until arr[j] >= p2
			while arr[j] < p2 {
				// arr[<i] -> elements that are less than p1, arr[i] is not less than p1
				if arr[j] < p1 {
					arr.swap(i, j);
					i += 1;
				}
				j += 1;
			}
			// k moves left until arr[k] <= p2
			while arr[k] > p2 {
				// arr[>l] -> elements that are greater than p3, arr[l] is not greater than p3
				if arr[k] > p3 {
					arr.swap(k, l);
					l -= 1;
				}
				k -= 1;
			}
			// if j is still less than k
			if j <= k {
				if arr[j] > p3 {
					if arr[k] < p1 {
						// if arr[j] > p3 and arr[k] < p1, 
						// rotate arr[j] to k and arr[k] to i because arr[<i] < p1
						rotate3(arr, j, i, k);
						i += 1;
					} else {
						// if arr[j] > p3 and arr[k] >= p1,
						// simply swap arr[j] and arr[k]
						arr.swap(j, k);
					}
					// at this moment arr[k] must be greater than p3
					// swap it with arr[l] to move it to the right
					arr.swap(k, l);
					l -= 1;
				} else { 
					// if arr[j] <= p3, we do the same logic as above
					if arr[k] < p1 {
						rotate3(arr, j, i, k);
						i += 1;
					} else {
						arr.swap(j, k);
					}
					// at this moment arr[j] must be less than or equal p1
					// arr[k] must be less than or equal p3
				}
				j += 1;
				k -= 1;
			}
		}
		
		i -= 1;
		j -= 1;
		k += 1;
		l += 1;
		// at this point arr[<=i] < p1, arr[i..=j] >= p1 and <= p2, arr[k..=l] >= p2 and <= p3, arr[>=l] > p3 (j == k)
		// move p2 from arr[left + 1] to vacant position arr[j] (in the middle) 
		rotate3(arr, left + 1, i, j);
		// arr.swap(left + 1, i);
		// arr.swap(i, j);

		i -= 1;

		arr.swap(left, i);
		arr.swap(right, l);
		if left + 1 < i {
			triple_pivot_quicksort(&mut arr[left..i]);
		}
		if i + 1 < j {
			triple_pivot_quicksort(&mut arr[i + 1..j]);
		}
		if j + 1 < l {
			triple_pivot_quicksort(&mut arr[j + 1..l]);
		}
		if l + 1 < right {
			triple_pivot_quicksort(&mut arr[l + 1..=right]);
		}
	}
}
