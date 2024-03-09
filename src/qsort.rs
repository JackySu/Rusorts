// #![allow(dead_code)]
#![allow(unused)]

use std::{fmt::Debug, mem::swap};
use std::simd::{cmp::SimdPartialOrd, *};

use crate::util::*;

const DEBUG_INSERTION_SORT_THRESHOLD: usize = 9;
const RELEASE_INSERTION_SORT_THRESHOLD: usize = 27;
const QUICKSORT_STACK_SIZE: usize = 64;


#[macro_export]
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
unsafe fn compare_idxs<T: PartialOrd>(v: &[T], a: usize, b: usize) -> bool {
    let x = v.get_unchecked(a);
    let y = v.get_unchecked(b);
    x > y
}

pub fn insertion_sort<T: PartialOrd>(v: &mut [T]) {
    let mut i = 1;
    let n = v.len();
    while i < n {
        let mut j = i;
        while j > 0 && ! unsafe { compare_idxs(v, j - 1, j) } {
            unsafe { v.swap_unchecked(j, j - 1) };
            j -= 1;
        }
        i += 1;
    }
}


pub fn quick_sort_lomuto_partition<T: PartialOrd + Copy>(arr: &mut [T]) {
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);

    unsafe {
        let (left, right) = (0, arr.len() - 1);
        let pivot = arr[right];
        let mut i = left;
        for j in left..right {
            if arr[j] <= pivot {
                arr.swap_unchecked(i, j);
                i += 1;
            }
        }
        arr.swap_unchecked(i, right);

        // if left part has more than 1 element
        if i > 1 {
            quick_sort_lomuto_partition(&mut arr[..=i - 1]);
        }
        // if right part has more than 1 element
        if i + 2 < arr.len() {
            quick_sort_lomuto_partition(&mut arr[i + 1..]);
        }
    }
}

fn simd_quick_sort_lomuto_partition(arr: &mut [f32]) {
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);
    let pivot_idx = arr.len() - 1;
    let pivot1 = arr[pivot_idx];
    // let arr_chunks = arr.chunks_exact(4);
    // let r = arr_chunks.remainder().clone();
    let pivot_vec = f32x8::splat(pivot1);
    unsafe {
        // let mut partition_0_indexes = Vec::with_capacity(arr.len());
        let mut p = 0;
        let mut i = 0;
        while i < (pivot_idx - 1) / 8 {
            let chunk_vec = f32x8::from_slice(&arr[i..i + 8]);
            let mask = chunk_vec.simd_le(pivot_vec);
            let mut mask_bits = mask.to_bitmask() as usize;
            for j in 0..8 {
                if mask_bits & 1 == 1 {
                    // partition_0_indexes.push(i + j);
                    arr.swap_unchecked(i + j, p);
                    p += 1;
                }
                mask_bits >>= 1;
            }
            i += 8;
        }
        while i < pivot_idx {
            if arr[i] <= pivot1 {
                // partition_0_indexes.push(i);
                arr.swap_unchecked(i, p);
                p += 1;
            }
            i += 1;
        }
        // dbg!(&partition_0_indexes);
        // swaps to shift partition_0_indexes to the leftmost side
        arr.swap_unchecked(p, pivot_idx);
        // arr[i] = pivot1;
        simd_quick_sort_lomuto_partition(&mut arr[..p]);
        simd_quick_sort_lomuto_partition(&mut arr[p..]);
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
		unsafe { arr.swap_unchecked(i as usize, j as usize); }
	}

	if j > 0 {
		quick_sort_hoare_partition(&mut arr[..=j as usize]);
	}
	if j < arr.len() as i32 {
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
			arr.swap_unchecked(left, right);
		}

		// partition indexes
		let mut less = left + 1;
		let mut greater = right - 1;

		// sorting
		let mut k = less;
		while k <= greater {
			if arr[k] <= *pivot1 {
					arr.swap_unchecked(k, less);
					less = less + 1;
			}
			else {
				if arr[k] >= *pivot2 {
					// find the rightmost element less than pivot2
					while k < greater && arr[greater] > *pivot2 {
						greater = greater - 1;
					}
					// swap it with arr[k]
					arr.swap_unchecked(k, greater);
					greater = greater - 1;

					// if the swapped element is less than pivot1
					// then swap it with arr[less]
					if arr[k] <= *pivot1 {
						arr.swap_unchecked(k, less);
						less = less + 1;
					}
				}
			}
			k = k + 1;
		}
		
		arr.swap_unchecked(less - 1, left);
		arr.swap_unchecked(greater + 1, right);

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
			arr.swap_unchecked(left, left + 1);
		}
		if *p2 > *p3 {
			arr.swap_unchecked(left + 1, right);
		}
		if *p1 > *p2 {
			arr.swap_unchecked(left, left + 1);
		}

		let (mut i, mut j, mut k, mut l) = (left + 2, left + 2, right - 1, right - 1);
		let (p1, p2, p3) = (*p1, *p2, *p3);
		while j <= k {
			// j moves right until arr[j] >= p2
			while arr[j] < p2 {
				// arr[<i] -> elements that are less than p1, arr[i] is not less than p1
				if arr[j] < p1 {
					arr.swap_unchecked(i, j);
					i += 1;
				}
				j += 1;
			}
			// k moves left until arr[k] <= p2
			while arr[k] > p2 {
				// arr[>l] -> elements that are greater than p3, arr[l] is not greater than p3
				if arr[k] > p3 {
					arr.swap_unchecked(k, l);
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
						rotate3(arr, [j, i, k]);
						i += 1;
					} else {
						// if arr[j] > p3 and arr[k] >= p1,
						// simply swap arr[j] and arr[k]
						arr.swap_unchecked(j, k);
					}
					// at this moment arr[k] must be greater than p3
					// swap it with arr[l] to move it to the right
					arr.swap_unchecked(k, l);
					l -= 1;
				} else { 
					// if arr[j] <= p3, we do the same logic as above
					if arr[k] < p1 {
						rotate3(arr, [j, i, k]);
						i += 1;
					} else {
						arr.swap_unchecked(j, k);
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
		rotate3(arr, [left + 1, i, j]);
		// arr.swap_unchecked(left + 1, i);
		// arr.swap_unchecked(i, j);

		i -= 1;

		arr.swap_unchecked(left, i);
		arr.swap_unchecked(right, l);
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

pub fn quad_pivot_quicksort<T: PartialOrd + Clone + Copy>(arr: &mut [T]) {
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);

    let (left, right) = (0, arr.len() - 1);
	
	unsafe {
		let p1: *mut T = &mut arr[left];
		let p2: *mut T = &mut arr[left + 1];
		let p3: *mut T = &mut arr[right - 1];
        let p4: *mut T = &mut arr[right];

        if *p1 > *p2 {
            arr.swap_unchecked(left, left + 1);
        }
        if *p2 > *p3 {
            arr.swap_unchecked(left + 1, right - 1);
        }
        if *p3 > *p4 {
            arr.swap_unchecked(right - 1, right);
        }
        if *p1 > *p2 {
            arr.swap_unchecked(left, left + 1);
        }
        if *p2 > *p3 {
            arr.swap_unchecked(left + 1, right - 1);
        }
        if *p1 > *p2 {
            arr.swap_unchecked(left, left + 1);
        }

        let (mut i, mut j, mut k, mut l, mut m) = (left + 2, left + 2, left + 2, right - 2, right - 2);
		let (p1, p2, p3, p4) = (*p1, *p2, *p3, *p4);
        
        while k <= l {
            //        | i              | j              | k
            // | < p1 | >= p1 and < p2 | >= p2 and < p3 | unknown
            while arr[k] < p3 {
                if arr[k] < p1 {
                    rotate3(arr, [k, j, i]);
                    i += 1;
                    j += 1;
                } else if arr[k] < p2 {
                    arr.swap_unchecked(k, j);
                    j += 1;
                }
                k += 1;
            }

            //       l |              m |      |               
            // unknown | >= p3 and < p4 | > p4 |
            while arr[l] > p3 {
                if arr[l] > p4 {
                    arr.swap_unchecked(l, m);
                    m -= 1;
                }
                l -= 1;
            }

            if k <= l {
                if arr[k] < p4 {
                    // arr[k] > p3, arr[l] < p3
                    if arr[l] < p1 {
                        rotate4(arr, [k, j, i, l]);
                        i += 1;
                        j += 1;
                    } else if arr[l] < p2 {
                        rotate3(arr, [k, j, l]);
                        j += 1;
                    } else {
                        arr.swap_unchecked(k, l);
                    }
                } else {
                    // arr[k] > p4, arr[l] < p3
                    if arr[l] > p2 { // arr[l] goes to (p2, p3), increase k
                        rotate3(arr, [k, l, m]);
                    } else if arr[l] > p1 { // arr[l] goes to (p1, p2), increase j and k
                        rotate4(arr, [k, j, l, m]);
                        j += 1;
                    } else { // arr[l] goes to leftmost side
                        rotate5(arr, [k, j, i, l, m]);
                        i += 1;
                        j += 1;
                    }
                    m -= 1;
                }
                k += 1;
                l -= 1;
            }
        }
        i -= 1;
        j -= 1;
        k -= 1;
        l += 1;
        m += 1;
        
        // i, j, l, m are indexes of pivot 1, 2, 3, 4
        // Here is the place rotate3 can't be replaced by arr.rotate_left
        // because the indexes (left + 1 > i ?) are not always ascending
        // so it could lead to a panic
        // anyway, I leave the rotate_n macro for you to try out in `src/util.rs`
        rotate3(arr, [left + 1, i, j]);
        i -= 1;
        arr.swap_unchecked(left, i);
        
        rotate3(arr, [right - 1, m, l]);
        m += 1;
        arr.swap_unchecked(right, m);

        if left + 1 < i {
            quad_pivot_quicksort(&mut arr[left..i]);
        }
        if i + 1 < j {
            quad_pivot_quicksort(&mut arr[i + 1..j]);
        }
        if j + 1 < l {
            quad_pivot_quicksort(&mut arr[j + 1..l]);
        }
        if l + 1 < m {
            quad_pivot_quicksort(&mut arr[l + 1..m]);
        }
        if m + 1 < right {
            quad_pivot_quicksort(&mut arr[m + 1..=right]);
        }
        
    }   
}


use once_cell::unsync::Lazy;
static mut ARENA: Lazy<Vec<Vec<f32>>> = Lazy::new(||
    vec![Vec::with_capacity(10_000_000); 9]
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

#[macro_export]
macro_rules! impl_4n_pivot_qsort {
    ($n:expr, $func_name:ident, $data_type:ty, $simd_type:ty) => {
        pub fn $func_name(arr: &mut [$data_type]) {
            conditional_sort!(debug, arr);
            conditional_sort!(release, arr);

            let mut pivots = [0 as $data_type; $n];
            for i in 0..$n {
                pivots[i] = arr[i];
            }
            pivots.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let pivot_vec = <$simd_type>::from_slice(&pivots);
            let mut bucket_sizes = [0; $n + 1];

            // let mut arena: Vec<Vec<$data_type>> = vec![Vec::with_capacity(arr.len()); $n + 1];
            unsafe {
                thread_local_arena_reset();
                for &mut x in arr.iter_mut() {
                    let x_vec = <$simd_type>::splat(x);
                    let mask = x_vec.simd_ge(pivot_vec);
                    let mask = mask.to_bitmask() as usize;
                    if mask == 0 {
                        // arena[0].push(x);
                        thread_local_arena_push(0, x);
                        // bucket_sizes[0] += 1;
                        continue;
                    }
                    for i in (0..$n).rev() {
                        if mask & (1 << i) > 0 {
                            // arena[i + 1].push(x);
                            thread_local_arena_push(i + 1, x);
                            // bucket_sizes[i + 1] += 1;
                            break;
                        }
                    }
                }

                let mut arr_ptr = arr.as_mut_ptr();
                for bucket in 0..=$n {
                    let bucket_cur = ARENA.get_unchecked(bucket);
                    let bucket_ptr = bucket_cur.as_ptr();
                    let bucket_len = bucket_cur.len();
                    bucket_sizes[bucket] = bucket_len;
                    // $func_name(bucket, pindex);
                    std::ptr::copy_nonoverlapping(bucket_ptr, arr_ptr, bucket_len);
                    arr_ptr = arr_ptr.add(bucket_len);
                };
            }

            $func_name(&mut arr[0..bucket_sizes[0]]);
            for i in 1..=$n {
                $func_name(&mut arr[bucket_sizes[..i].iter().sum::<usize>()..bucket_sizes[..i + 1].iter().sum::<usize>()]);
            }
        }
    }
}

impl_4n_pivot_qsort!(4, quadro_pivot_quicksort, f32, f32x4);
impl_4n_pivot_qsort!(8, octal_pivot_quicksort, f32, f32x8);

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


#[macro_export]
macro_rules! impl_non_4n_pivot_qsort {
    ($n:expr, $pivot_repeat_times:expr, $func_name:ident, $data_type:ty, $simd_len:expr, $simd_type:ty) => {
        pub fn $func_name(arr: &mut [$data_type]) {
            conditional_sort!(debug, arr);
            conditional_sort!(release, arr);
            let mut pivots = [0.0; $n];
            for i in 0..$n {
                pivots[i] = arr[i];
            }
            pivots.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
            let mut bucket_sizes = [0; $n + 1];
        
            let arr_chunks = arr.chunks_exact($pivot_repeat_times);
            let r = arr_chunks.remainder().clone();
            thread_local_arena_reset();

            let filled = pivots.repeat($pivot_repeat_times);
            let pivots_vecs = filled.chunks($simd_len); // construct pivots_vecs
            // for n = 5 it looks like
            // [p0, p1, p2, p3, p4, p0, p1, p2, p3, p4, p0, p1, p2, p3, p4, p0, p1, p2, p3, p4]
            
            let mut arr_repeated = [0 as $data_type; $n * $pivot_repeat_times];
            for chunk in arr_chunks {
                let mut result = 0;
                for i in 0..$pivot_repeat_times {
                    for j in 0..$n {
                        arr_repeated[i * $n + j] = chunk[i];
                    }
                }
                let elem_chunks = arr_repeated.chunks_exact($simd_len);

                for (i, (arr_chunk, pivots_vec)) in (elem_chunks.zip(pivots_vecs.clone())).enumerate() {
                    let arr_vec = <$simd_type>::from_slice(arr_chunk);
                    let mask = arr_vec.simd_le(<$simd_type>::from_slice(pivots_vec));
                    let mask = mask.to_bitmask() as usize;
                    result |= mask << (i * $simd_len);
                }
                for i in 0..$pivot_repeat_times {
                    let mask = 2usize.pow($n) - 1;  // 0b11111
                    let sub_result = result >> (i * $n) & mask;
                    if sub_result == 0 {
                        thread_local_arena_push($n, chunk[i]);
                        continue;
                    }
                    for j in 0..$n {
                        if sub_result & 1 << j > 0 {
                            thread_local_arena_push(j, chunk[i]);
                            break;
                        }
                    }
                }
            }
            for &x in r {
                if x > pivots[$n - 1] { // 5 - 1
                    thread_local_arena_push($n, x);
                    continue;
                }
                for i in 0..$n {
                    if x <= pivots[i] {
                        thread_local_arena_push(i, x);
                        break;
                    }
                }
            }
            
            unsafe {
                let mut arr_ptr = arr.as_mut_ptr();
                for i in 0..=$n {
                    let bucket = ARENA.get_unchecked(i);
                    let bucket_ptr = bucket.as_ptr();
                    let bucket_len = bucket.len();
                    bucket_sizes[i] = bucket_len;
                    std::ptr::copy_nonoverlapping(bucket_ptr, arr_ptr, bucket_len);
                    arr_ptr = arr_ptr.add(bucket_len);
                };
            }
            $func_name(&mut arr[0..bucket_sizes[0]]);
            for i in 1..=$n {
                $func_name(&mut arr[bucket_sizes[..i].iter().sum::<usize>()..bucket_sizes[..i + 1].iter().sum::<usize>()]);
            }
        }
    };
}

// params: $n, $pivot_repeat_times, $func_name, $data_type, $simd_len, $simd_type
impl_non_4n_pivot_qsort!(5, 4, penta_pivot_quicksort, f32, 4, f32x4);
impl_non_4n_pivot_qsort!(6, 2, hexa_pivot_quicksort, f32, 4, f32x4);
impl_non_4n_pivot_qsort!(7, 4, hepta_pivot_quicksort, f32, 4, f32x4);