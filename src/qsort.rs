// #![allow(dead_code)]
#![allow(unused)]

use std::{fmt::Debug, mem::swap};
use std::simd::{cmp::SimdPartialOrd, *};

const DEBUG_INSERTION_SORT_THRESHOLD: usize = 5;
const RELEASE_INSERTION_SORT_THRESHOLD: usize = 27;
const QUICKSORT_STACK_SIZE: usize = 64;

macro_rules! conditional_sort {
    (debug, $arr: expr) => {
        #[cfg(debug_assertions)]
        if $arr.len() < DEBUG_INSERTION_SORT_THRESHOLD {
            return $arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
    };
    (release, $arr: expr) => {
        #[cfg(not(debug_assertions))]
        if $arr.len() < RELEASE_INSERTION_SORT_THRESHOLD {
            return $arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        }
    };
}

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
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);

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
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);

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
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);
	let (left, right) = (0, arr.len() - 1);

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
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);

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

#[inline]
pub unsafe fn sort_ptrs<T: PartialOrd + Copy>(ptrs: &[*mut T]) {
	// better not use this for pivots < 4
	let mut data = ptrs.iter().map(|&p| *p).collect::<Vec<T>>();
	data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
	for (i, p) in ptrs.iter().zip(data.iter()) {
		**i = *p;
	}
}

use once_cell::unsync::Lazy;
static mut ARENA: Lazy<Vec<Vec<f32>>> = Lazy::new(||
    vec![Vec::with_capacity(100_000_000); 10]
);


#[inline]
fn thread_local_arena_reset() {
    unsafe {
        ARENA.iter_mut().for_each(|bucket| bucket.clear());
    }
}

#[inline]
fn thread_local_arena_push(i: usize, n: f32) {
    unsafe {
        ARENA[i].push(n);
    };
}

#[inline]
fn thread_local_arena_clear_lane(i: usize) {
    unsafe {
        ARENA[i].clear();
    }
}

pub fn quadro_pivot_quicksort(arr: &mut [f32]) {
	conditional_sort!(debug, arr);
    conditional_sort!(release, arr);
    let n = arr.len();
    if n < 27 {
        arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
        return;
    }
    let mut pivots = [
        arr[arr.len() / 4].clone() as f32,
        arr[arr.len() / 2].clone(),
        arr[arr.len() * 3 / 4].clone(),
        arr[arr.len() - 1].clone(),
    ];
    pivots.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let pivot_vec = f32x4::from_array(pivots);
    // compare with array elements, put elements less than pivot0 into the first bucket of arena, put elements less than pivot1 into the second bucket of arena, and so on
    let mut bucket_sizes = [0; 5];
    // the size of arr is guaranteed to be a multiple of 8
    
    unsafe {
    thread_local_arena_reset();
    for &mut x in arr.into_iter() {
        // don't worry, the slice is always aligned to 16 bytes, won't be performance penalty
        let x_vec = f32x4::splat(x);
        let mask = x_vec.simd_ge(pivot_vec);
        let mask = mask.to_bitmask() as usize;
        match mask {
            0b0000 => {
                thread_local_arena_push(0, x);
                bucket_sizes[0] += 1;
            }
            0b0001 => {
                thread_local_arena_push(1, x);
                bucket_sizes[1] += 1;
            }
            0b0011 => {
                thread_local_arena_push(2, x);
                bucket_sizes[2] += 1;
            }
            0b0111 => {
                thread_local_arena_push(3, x);
                bucket_sizes[3] += 1;
            }
            0b1111 => {
                thread_local_arena_push(4, x);
                bucket_sizes[4] += 1;
            }
            _ => unreachable!(),
        }
        // let mask = mask.to_bitmask() as usize;

        // let bucket = unsafe { arena.assume_init_mut().get_unchecked_mut(mask) };
        // push elements greater than or equal to pivot0 into the first bucket
    }
        // copy elements from buckets to arr
    
        // memcpy from buckets back to arr
        let mut arr_ptr = arr.as_mut_ptr();
        for bucket in ARENA.iter() {
            // assert!(arr_ptr.align_offset(16) == 0);
            let bucket_ptr = bucket.as_ptr();
            // assert!(bucket_ptr.align_offset(16) == 0);
            let bucket_len = bucket.len();
            std::ptr::copy_nonoverlapping(bucket_ptr, arr_ptr, bucket_len);
            arr_ptr = arr_ptr.add(bucket_len);
        };
    }

    quadro_pivot_quicksort(&mut arr[0..bucket_sizes[0]]);
    quadro_pivot_quicksort(&mut arr[bucket_sizes[0]..bucket_sizes[0] + bucket_sizes[1]]);
    quadro_pivot_quicksort(&mut arr[bucket_sizes[0] + bucket_sizes[1]..bucket_sizes[0] + bucket_sizes[1] + bucket_sizes[2]]);
    quadro_pivot_quicksort(&mut arr[bucket_sizes[0] + bucket_sizes[1] + bucket_sizes[2]..bucket_sizes[0] + bucket_sizes[1] + bucket_sizes[2] + bucket_sizes[3]]);
    quadro_pivot_quicksort(&mut arr[bucket_sizes[0] + bucket_sizes[1] + bucket_sizes[2] + bucket_sizes[3]..]);
    
}

pub fn quadro_pivot_quicksort_2(arr: &mut [f32]) {
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);
    let n = arr.len();
    
    let n_pivots = 4;
    
    let mut pivots = [
        arr[arr.len() / 4].clone(),
        arr[arr.len() / 2].clone(),
        arr[arr.len() * 3 / 4].clone(),
        arr[arr.len() - 1].clone(),
    ];
    pivots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    // compare with array elements, put elements less than pivot0 into the first bucket of arena, put elements less than pivot1 into the second bucket of arena, and so on
    let mut bucket_sizes = [0; 5];
    // the size of arr is guaranteed to be a multiple of 8 in the first step, but not for the remainings
    
    let mut a = arr.to_vec();
    thread_local_arena_reset();
    for i in 0..n_pivots {
        if i > 0 { a = unsafe { ARENA[i].clone() }; }
        // dbg!(&a.len());
        thread_local_arena_clear_lane(i);
        let pivot_vec = f32x8::splat(pivots[i]);
        let chunks = a.chunks_exact(8);
        for x in chunks.clone() {
            let x_vec = f32x8::from_slice(x);
            let mask = x_vec.simd_le(pivot_vec);
            let mut mask = mask.to_bitmask() as usize;
            for j in 0..8 {
                if mask & 1 == 1 {
                    thread_local_arena_push(i, x[j]);
                    bucket_sizes[i] += 1;
                } else {
                    thread_local_arena_push(i + 1, x[j]);
                }
                mask >>= 1;
            }
        }
        let r = chunks.remainder();
        for &x in r {
            if x <= pivots[i] {
                thread_local_arena_push(i, x);
                bucket_sizes[i] += 1;
            } else {
                thread_local_arena_push(i + 1, x);
            }
        }
        if i < n_pivots - 1 { a.clear(); }
        // dbg!(&a.len(), bucket_sizes);
    }
    bucket_sizes[n_pivots] = unsafe { ARENA[n_pivots].len() };
    // memcpy from buckets back to arr
    // dbg!(bucket_sizes);
    unsafe {
        let mut arr_ptr = arr.as_mut_ptr();
        for i in 0..=n_pivots {
            // assert!(arr_ptr.align_offset(16) == 0);
            let bucket_ptr = ARENA[i].as_ptr();
            // assert!(bucket_ptr.align_offset(16) == 0);
            let bucket_len = bucket_sizes[i];
            // dbg!(bucket_len);
            std::ptr::copy_nonoverlapping(bucket_ptr, arr_ptr, bucket_len);
            arr_ptr = arr_ptr.add(bucket_len);
        };
    }
    // bg!(n, bucket_sizes);
    quadro_pivot_quicksort_2(&mut arr[0..bucket_sizes[0]]);
    quadro_pivot_quicksort_2(&mut arr[bucket_sizes[0]..bucket_sizes[0] + bucket_sizes[1]]);
    quadro_pivot_quicksort_2(&mut arr[bucket_sizes[0] + bucket_sizes[1]..bucket_sizes[0] + bucket_sizes[1] + bucket_sizes[2]]);
    quadro_pivot_quicksort_2(&mut arr[bucket_sizes[0] + bucket_sizes[1] + bucket_sizes[2]..bucket_sizes[0] + bucket_sizes[1] + bucket_sizes[2] + bucket_sizes[3]]);
    quadro_pivot_quicksort_2(&mut arr[bucket_sizes[0] + bucket_sizes[1] + bucket_sizes[2] + bucket_sizes[3]..]);
}
