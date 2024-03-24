// #![allow(dead_code)]
#![allow(clippy::uninit_assumed_init)]
use std::simd::{cmp::SimdPartialOrd, *};
use std::cmp::Ordering;
use std::{ptr, mem, cmp};

use crate::util::*;

#[cfg(debug_assertions)]
const DEBUG_INSERTION_SORT_THRESHOLD: usize = 9;
#[cfg(not(debug_assertions))]
const RELEASE_INSERTION_SORT_THRESHOLD: usize = 27;


#[inline]
fn insertion_sort<T: Ord>(arr: &mut [T], left: usize, right: usize) {
	for i in (left + 1)..(right + 1) {
		let mut j = i;
		while j > left && arr[j].cmp(&arr[j - 1]) == Ordering::Less {
			arr.swap(j, j - 1);
			j = j - 1;
		}
	}
}

#[inline]
fn partial_insertion_sort<T: PartialOrd>(arr: &mut [T], left: usize, right: usize) {
    for i in (left + 1)..(right + 1) {
        let mut j = i;
        while j > left && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j = j - 1;
        }
    }
}

macro_rules! conditional_sort {
    (debug, $arr: expr) => {
        #[cfg(debug_assertions)]
        if $arr.len() < DEBUG_INSERTION_SORT_THRESHOLD {
            return insertion_sort($arr, 0, $arr.len() - 1);
        }
    };
    (release, $arr: expr) => {
        #[cfg(not(debug_assertions))]
        if $arr.len() < RELEASE_INSERTION_SORT_THRESHOLD {
            return insertion_sort($arr, 0, $arr.len() - 1);
        }
    };
}

macro_rules! conditional_partial_sort {
    (debug, $arr: expr) => {
        #[cfg(debug_assertions)]
        if $arr.len() < DEBUG_INSERTION_SORT_THRESHOLD {
            return partial_insertion_sort($arr, 0, $arr.len() - 1);
        }
    };
    (release, $arr: expr) => {
        #[cfg(not(debug_assertions))]
        if $arr.len() < RELEASE_INSERTION_SORT_THRESHOLD {
            return partial_insertion_sort($arr, 0, $arr.len() - 1);
        }
    };
}

pub fn quick_sort_lomuto_partition<T: Ord>(mut arr: &mut [T]) {
    loop {
        unsafe {
            conditional_sort!(debug, arr);
            conditional_sort!(release, arr);
            let (left, right) = (0, arr.len() - 1);
            let pivot = ptr::read(arr.get_unchecked(right));
            let mut i = left;
            for j in left..right {
                if arr[j].cmp(&pivot) == Ordering::Less || arr[j].cmp(&pivot) == Ordering::Equal {
                    arr.swap_unchecked(i, j);
                    i += 1;
                }
            }
            arr.swap_unchecked(i, right);
            let (left, right) = arr.split_at_mut(i);
            if left.len() > right.len() {
                quick_sort_lomuto_partition(right);
                arr = left;
            } else {
                quick_sort_lomuto_partition(left);
                arr = right;
            }
        }
    }
}

pub fn quick_sort_lomuto_partition_block<T: Ord>(mut arr: &mut [T]) {
    const BLOCK: usize = 128;
    let mut block_t = BLOCK;
    let mut offsets_t: [u8; BLOCK] = unsafe { mem::MaybeUninit::uninit().assume_init() };
    loop {
        unsafe {
            conditional_sort!(debug, arr);
            conditional_sort!(release, arr);
            let (left, right) = (0, arr.len() - 1);
            let pivot = ptr::read(arr.get_unchecked(right));
            let (mut i, mut j) = (left, left);
            let mut num = 0;
            while j < right {
                block_t = cmp::min(block_t, right - j);
                for k in 0..block_t {
                    offsets_t[num] = k as u8;
                    num += (arr[j + k].cmp(&pivot) == Ordering::Less) as usize;
                }
                for k in 0..num {
                    arr.swap_unchecked(i, j + offsets_t[k] as usize);
                    i += 1;
                }
                num = 0;
                j += block_t;
            }
            arr.swap_unchecked(i, right);
            let (left, right) = arr.split_at_mut(i);
            let (pivot, right) = right.split_at_mut(1);
            let pivot = &pivot[0];
            debug_assert!(left.iter().all(|x| x <= pivot) && right.iter().all(|x| x >= pivot));
            if left.len() < right.len() {
                quick_sort_lomuto_partition_block(left);
                arr = right;
            } else {
                quick_sort_lomuto_partition_block(right);
                arr = left;
            }
        }
    }
}

pub fn double_pivot_quicksort_lomuto_partition_block<T: Ord>(mut arr: &mut [T]) {
    const BLOCK: usize = 128;
    let mut block_t = BLOCK;
    let mut offsets_t: [u8; BLOCK] = unsafe { mem::MaybeUninit::uninit().assume_init() };
    loop {
        unsafe {
            conditional_sort!(debug, arr);
            conditional_sort!(release, arr);
            let (left, right) = (0, arr.len() - 1);
            if arr.get_unchecked(left).cmp(arr.get_unchecked(right)) == Ordering::Greater {
                arr.swap_unchecked(left, right);
            }
            let (pivot1, pivot2) = (ptr::read(arr.get_unchecked(left)), ptr::read(arr.get_unchecked(right)));
        
            let (mut i, mut j, mut k) = (left + 1, left + 1, left + 1);
            let (mut num_p1, mut num_p2) = (0, 0);
            while k < right {
                block_t = cmp::min(block_t, right - k);
                for l in 0..block_t {
                    offsets_t[num_p2] = l as u8;
                    num_p2 += (arr[k + l].cmp(&pivot2) == Ordering::Less) as usize;
                }
                for l in 0..num_p2 {
                    arr.swap_unchecked(j + l, k + offsets_t[l] as usize);
                }
                k += block_t;
                for l in 0..num_p2 {
                    offsets_t[num_p1] = l as u8;
                    num_p1 += (arr[j + l].cmp(&pivot1) == Ordering::Less) as usize;
                }
                for l in 0..num_p1 {
                    arr.swap_unchecked(i, j + offsets_t[l] as usize);
                    i += 1;
                }
                j += num_p2;
                num_p1 = 0;
                num_p2 = 0;
            }
            arr.swap_unchecked(i - 1, left);
            arr.swap_unchecked(j, right);
            let (left, right) = arr.split_at_mut(i - 1);
            let (pivot1, right) = right.split_at_mut(1);
            let _pivot1 = &pivot1[0];
            let (mid, right) = right.split_at_mut(j - i);
            let (pivot2, right) = right.split_at_mut(1);
            let _pivot2 = &pivot2[0];

            if left.len() < mid.len() {
                double_pivot_quicksort_lomuto_partition_block(left);
                double_pivot_quicksort_lomuto_partition_block(right);
                arr = mid;
            } else if mid.len() > right.len() {
                double_pivot_quicksort_lomuto_partition_block(right);
                double_pivot_quicksort_lomuto_partition_block(mid);
                arr = left;
            } else {
                double_pivot_quicksort_lomuto_partition_block(left);
                double_pivot_quicksort_lomuto_partition_block(mid);
                arr = right;
            }
        }
    }
}

pub fn double_pivot_quicksort_new_partition_block<T: Ord>(mut arr: &mut [T]) {
    const BLOCK: usize = 128;
    let mut block_t = BLOCK;
    // offsets of elements <P1 will be stored from left to right, and elements P1<=x<P2 will be stored from right to left
    let mut offsets: [u8; BLOCK] = unsafe { mem::MaybeUninit::uninit().assume_init() };
    loop {
        unsafe {
            conditional_sort!(debug, arr);
            conditional_sort!(release, arr);
            let (left, right) = (0, arr.len() - 1);
            if arr.get_unchecked(left).cmp(arr.get_unchecked(right)) == Ordering::Greater {
                arr.swap_unchecked(left, right);
            }
            let (pivot1, pivot2) = (ptr::read(arr.get_unchecked(left)), ptr::read(arr.get_unchecked(right)));
            // dbg!(&pivot1, &pivot2);
            let (mut i, mut j, mut k) = (left + 1, left + 1, left + 1);
            let (mut num_p1, mut num_p2) = (0, 0);
            while k < right {
                block_t = cmp::min(block_t, right - k);
                for l in 0..block_t {
                    offsets[num_p1] = l as u8;
                    num_p1 += (arr[k + l].cmp(&pivot1) == Ordering::Less) as usize;
                    offsets[block_t - 1 - num_p2] = l as u8;
                    num_p2 += (arr[k + l].cmp(&pivot2) == Ordering::Less && 
                        (arr[k + l].cmp(&pivot1) == Ordering::Greater || arr[k + l].cmp(&pivot1) == Ordering::Equal)) as usize;
                }
 
                let (mut idx_p1, mut idx_p2) = (0, 0);
                let mut l = 0;
                while l < num_p1 + num_p2 {
                    // if there is nothing to swap in either one of the blocks, break
                    if (idx_p1 == num_p1) || (idx_p2 == num_p2) {
                        break;
                    }
                    let idx_off = cmp::min(offsets[idx_p1], offsets[block_t - 1 - idx_p2]);                    
                    if idx_off == offsets[idx_p1] {
                        rotate4(arr, [k + idx_off as usize, k + l, j, i]);
                        i += 1;
                        idx_p1 += 1;
                    } else {
                        arr.swap_unchecked(k + idx_off as usize, j);
                        idx_p2 += 1;
                    }
                    j += 1;
                    l += 1;
                }
                if idx_p1 < num_p1 {
                    for idx_off in idx_p1..num_p1 {
                        rotate3(arr, [k + offsets[idx_off] as usize, j, i]);
                        i += 1;
                        j += 1;
                    }
                }
                if idx_p2 < num_p2 {
                    for idx_off in idx_p2..num_p2 {
                        arr.swap_unchecked(k + offsets[block_t - 1 - idx_off] as usize, j);
                        j += 1;
                    }
                }
                k += block_t;
                num_p1 = 0;
                num_p2 = 0;
            }
            arr.swap_unchecked(i - 1, left);
            arr.swap_unchecked(j, right);
            let (left, right) = arr.split_at_mut(i - 1);
            let (pivot1, right) = right.split_at_mut(1);
            let _pivot1 = &pivot1[0];
            let (mid, right) = right.split_at_mut(j - i);
            let (pivot2, right) = right.split_at_mut(1);
            let _pivot2 = &pivot2[0];
            
            if left.len() < mid.len() {
                double_pivot_quicksort_new_partition_block(left);
                double_pivot_quicksort_new_partition_block(right);
                arr = mid;
            } else if mid.len() > right.len() {
                double_pivot_quicksort_new_partition_block(right);
                double_pivot_quicksort_new_partition_block(mid);
                arr = left;
            } else {
                double_pivot_quicksort_new_partition_block(left);
                double_pivot_quicksort_new_partition_block(mid);
                arr = right;
            }
        }
    }
}


pub fn quick_sort_hoare_partition<T: Ord>(mut arr: &mut [T]) {
    loop {
        unsafe {
            conditional_sort!(debug, arr);
            conditional_sort!(release, arr);
            let mut i = -1;
            let mut j = arr.len() as isize;
            let pivot = ptr::read(arr.get_unchecked(0));
            loop {
                i += 1;
                while arr[i as usize].cmp(&pivot) == Ordering::Less {
                    i += 1;
                }
        
                j -= 1;
                while arr[j as usize].cmp(&pivot) == Ordering::Greater {
                    j -= 1;
                }
        
                if i >= j {
                    break;
                }
                arr.swap_unchecked(i as usize, j as usize);
            }

            // arr.swap_unchecked(0, j as usize);
            let (left, right) = arr.split_at_mut((j + 1) as usize);
            if left.len() > right.len() {
                if right.len() > 1 {
                    quick_sort_hoare_partition(right);
                }
                arr = left;
            } else {
                if left.len() > 1 {
                    quick_sort_hoare_partition(left);
                }
                arr = right;
            }
        }
    }
    
}

// Refer to PDQSort implementation in std::slice::unstable_sort
fn partition_in_blocks<T, F>(v: &mut [T], pivot: &T, is_less: &mut F) -> usize
    where F: FnMut(&T, &T) -> bool
{
    const BLOCK: usize = 128;

    let mut l = v.as_mut_ptr();
    let mut block_l = BLOCK;
    let mut start_l = ptr::null_mut();
    let mut end_l = ptr::null_mut();
    let mut offsets_l: [u8; BLOCK] = unsafe { mem::MaybeUninit::uninit().assume_init() };

    // The current block on the right side (from `r.offset(-block_r)` to `r`).
    let mut r = unsafe { l.offset(v.len() as isize) };
    let mut block_r = BLOCK;
    let mut start_r = ptr::null_mut();
    let mut end_r = ptr::null_mut();
    let mut offsets_r: [u8; BLOCK] = unsafe { mem::MaybeUninit::uninit().assume_init() };

    fn width<T>(l: *mut T, r: *mut T) -> usize {
        (r as usize - l as usize) / mem::size_of::<T>()
    }

    loop {
        let is_done = width(l, r) <= 2 * BLOCK;

        if is_done {
            let mut rem = width(l, r);
            if start_l < end_l || start_r < end_r {
                rem -= BLOCK;
            }

            if start_l < end_l {
                block_r = rem;
            } else if start_r < end_r {
                block_l = rem;
            } else {
                block_l = rem / 2;
                block_r = rem - block_l;
            }
            debug_assert!(block_l <= BLOCK && block_r <= BLOCK);
            debug_assert!(width(l, r) == block_l + block_r);
        }

        if start_l == end_l {
            start_l = offsets_l.as_mut_ptr();
            end_l = offsets_l.as_mut_ptr();
            let mut elem = l;

            for i in 0..block_l {
                unsafe {
                    *end_l = i as u8;
                    end_l = end_l.offset(!is_less(&*elem, pivot) as isize);
                    elem = elem.offset(1);
                }
            }
        }

        if start_r == end_r {
            start_r = offsets_r.as_mut_ptr();
            end_r = offsets_r.as_mut_ptr();
            let mut elem = r;

            for i in 0..block_r {
                unsafe {
                    elem = elem.offset(-1);
                    *end_r = i as u8;
                    end_r = end_r.offset(is_less(&*elem, pivot) as isize);
                }
            }
        }

        let count = cmp::min(width(start_l, end_l), width(start_r, end_r));

        if count > 0 {
            macro_rules! left { () => { l.offset(*start_l as isize) } }
            macro_rules! right { () => { r.offset(-(*start_r as isize) - 1) } }

            // cyclic swap
            unsafe {
                let tmp = ptr::read(left!());
                ptr::copy_nonoverlapping(right!(), left!(), 1);

                for _ in 1..count {
                    start_l = start_l.offset(1);
                    ptr::copy_nonoverlapping(left!(), right!(), 1);
                    start_r = start_r.offset(1);
                    ptr::copy_nonoverlapping(right!(), left!(), 1);
                }

                ptr::copy_nonoverlapping(&tmp, right!(), 1);
                mem::forget(tmp);
                start_l = start_l.offset(1);
                start_r = start_r.offset(1);
            }
        }

        if start_l == end_l {
            l = unsafe { l.offset(block_l as isize) };
        }

        if start_r == end_r {
            r = unsafe { r.offset(-(block_r as isize)) };
        }

        if is_done {
            break;
        }
    }

    if start_l < end_l {
        debug_assert_eq!(width(l, r), block_l);
        while start_l < end_l {
            unsafe {
                end_l = end_l.offset(-1);
                ptr::swap(l.offset(*end_l as isize), r.offset(-1));
                r = r.offset(-1);
            }
        }
        width(v.as_mut_ptr(), r)
    } else if start_r < end_r {
        debug_assert_eq!(width(l, r), block_r);
        while start_r < end_r {
            unsafe {
                end_r = end_r.offset(-1);
                ptr::swap(l, r.offset(-(*end_r as isize) - 1));
                l = l.offset(1);
            }
        }
        width(v.as_mut_ptr(), l)
    } else {
        width(v.as_mut_ptr(), l)
    }
}


pub fn quick_sort_hoare_partition_block<T: Ord>(mut arr: &mut [T]) {

    let is_less = &mut |a: &T, b: &T| a.cmp(b) == Ordering::Less;

    unsafe {
        loop {
            conditional_sort!(debug, arr);
            conditional_sort!(release, arr);
            let (mid, _was_partitioned) = {
                let (pivot, arr) = arr.split_at_mut(1);
                let pivot = &mut pivot[0];
                let mut l = 0;
                let mut r = arr.len();
                while l < r && is_less(arr.get_unchecked(l), pivot) {
                    l += 1;
                }
        
                while l < r && is_less(pivot, arr.get_unchecked(r - 1)) {
                    r -= 1;
                }
    
                (l + partition_in_blocks(&mut arr[l..r], pivot, is_less), l >= r)
            };
            arr.swap_unchecked(0, mid);
    
            let (left, right) = arr.split_at_mut(mid);
            let (_pivot, right) = right.split_at_mut(1);
    
            if left.len() < right.len() {
                quick_sort_hoare_partition_block(left);
                arr = right;
            } else {
                quick_sort_hoare_partition_block(right);
                arr = left;
            }
        }
    }

}

// In respect to 
// - https://github.com/veddan/rust-introsort/blob/master/src/sort.rs
// - https://github.com/rosacris/rust-doublepivot-quicksort/blob/master/src/lib.rs
pub fn double_pivot_quicksort<T: Ord>(arr: &mut [T]) {
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);
	let (left, right) = (0, arr.len() - 1);

	unsafe {
		// pivots
		let pivot1 : *mut T = &mut arr[left];
		let pivot2 : *mut T = &mut arr[right];
		
		// swap pivots if p1 > p2
		if (&*pivot1).cmp(&*pivot2) == Ordering::Greater {
			arr.swap_unchecked(left, right);
		}

		// partition indexes
		let mut less = left + 1;
		let mut greater = right - 1;

		// sorting
		let mut k = less;
		while k <= greater {
			if arr[k].cmp(&*pivot1) == Ordering::Less {
                arr.swap_unchecked(k, less);
                less = less + 1;
            }
			else {
                if arr[k].cmp(&*pivot2) == Ordering::Greater {
                    // find the rightmost element less than pivot2
                    while k < greater && arr[greater].cmp(&*pivot2) == Ordering::Greater {
                        greater = greater - 1;
                    }
                    // swap it with arr[k]
                    arr.swap_unchecked(k, greater);
                    greater = greater - 1;

                    // if the swapped element is less than pivot1
                    // then swap it with arr[less]
                    if arr[k].cmp(&*pivot1) == Ordering::Less {
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
			double_pivot_quicksort(&mut arr[left..less - 1]);
		}

		if greater + 2 < right {
			double_pivot_quicksort(&mut arr[greater + 2..=right]);
		}

		if less < greater && (&*pivot1).cmp(&*pivot2) == Ordering::Less { // some elements are equal to pivot1 or pivot2
			double_pivot_quicksort(&mut arr[less..=greater]);
		}
	}
}

pub fn triple_pivot_quicksort<T: Ord>(arr: &mut [T]) {
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);

	let (left, right) = (0, arr.len() - 1);
	
	unsafe {
		let p1: *mut T = &mut arr[left];
		let p2: *mut T = &mut arr[left + 1];
		let p3: *mut T = &mut arr[right];
	
		if (&*p1).cmp(&*p2) == Ordering::Greater {
			arr.swap_unchecked(left, left + 1);
		}
		if (&*p2).cmp(&*p3) == Ordering::Greater {
			arr.swap_unchecked(left + 1, right);
		}
		if (&*p1).cmp(&*p2) == Ordering::Greater { 
			arr.swap_unchecked(left, left + 1);
		}

		let (mut i, mut j, mut k, mut l) = (left + 2, left + 2, right - 1, right - 1);
		let (p1, p2, p3) = (ptr::read(p1), ptr::read(p2), ptr::read(p3));
		while j <= k {
			// j moves right until arr[j] >= p2
			while arr[j].cmp(&p2) == Ordering::Less {
				// arr[<i] -> elements that are less than p1, arr[i] is not less than p1
				if arr[j].cmp(&p1) == Ordering::Less {
					arr.swap_unchecked(i, j);
					i += 1;
				}
				j += 1;
			}
			// k moves left until arr[k] <= p2
			while arr[k].cmp(&p2) == Ordering::Greater {
				// arr[>l] -> elements that are greater than p3, arr[l] is not greater than p3
				if arr[k].cmp(&p3) == Ordering::Greater {
					arr.swap_unchecked(k, l);
					l -= 1;
				}
				k -= 1;
			}
			// if j is still less than k
			if j <= k {
				if arr[j].cmp(&p3) == Ordering::Greater {
					if arr[k].cmp(&p1) == Ordering::Less {
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
					if arr[k].cmp(&p1) == Ordering::Less {
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
		// k += 1;  // k is never used, so it's not necessary to increment it
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

pub fn quad_pivot_quicksort<T: Ord>(arr: &mut [T]) {
    conditional_sort!(debug, arr);
    conditional_sort!(release, arr);

    let (left, right) = (0, arr.len() - 1);
	
	unsafe {
		let p1: *mut T = &mut arr[left];
		let p2: *mut T = &mut arr[left + 1];
		let p3: *mut T = &mut arr[right - 1];
        let p4: *mut T = &mut arr[right];

        if (&*p1).cmp(&*p2) == Ordering::Greater {
            arr.swap_unchecked(left, left + 1);
        }
        if (&*p2).cmp(&*p3) == Ordering::Greater {
            arr.swap_unchecked(left + 1, right - 1);
        }
        if (&*p3).cmp(&*p4) == Ordering::Greater {
            arr.swap_unchecked(right - 1, right);
        }
        if (&*p1).cmp(&*p2) == Ordering::Greater {
            arr.swap_unchecked(left, left + 1);
        }
        if (&*p2).cmp(&*p3) == Ordering::Greater {
            arr.swap_unchecked(left + 1, right - 1);
        }
        if (&*p1).cmp(&*p2) == Ordering::Greater {
            arr.swap_unchecked(left, left + 1);
        }

        let (mut i, mut j, mut k, mut l, mut m) = (left + 2, left + 2, left + 2, right - 2, right - 2);
		let (p1, p2, p3, p4) = (ptr::read(p1), ptr::read(p2), ptr::read(p3), ptr::read(p4));
        
        while k <= l {
            //        | i              | j              | k
            // | < p1 | >= p1 and < p2 | >= p2 and < p3 | unknown
            while arr[k].cmp(&p3) == Ordering::Less {
                if arr[k].cmp(&p1) == Ordering::Less {
                    rotate3(arr, [k, j, i]);
                    i += 1;
                    j += 1;
                } else if arr[k].cmp(&p2) == Ordering::Less {
                    arr.swap_unchecked(k, j);
                    j += 1;
                }
                k += 1;
            }

            //       l |              m |      |               
            // unknown | >= p3 and < p4 | > p4 |
            while arr[l].cmp(&p3) == Ordering::Greater {
                if arr[l].cmp(&p4) == Ordering::Greater {
                    arr.swap_unchecked(l, m);
                    m -= 1;
                }
                l -= 1;
            }

            if k <= l {
                if arr[k].cmp(&p4) == Ordering::Less {
                    // arr[k] > p3, arr[l] < p3
                    if arr[l].cmp(&p1) == Ordering::Less {
                        rotate4(arr, [k, j, i, l]);
                        i += 1;
                        j += 1;
                    } else if arr[l].cmp(&p2) == Ordering::Less {
                        rotate3(arr, [k, j, l]);
                        j += 1;
                    } else {
                        arr.swap_unchecked(k, l);
                    }
                } else {
                    // arr[k] > p4, arr[l] < p3
                    if arr[l].cmp(&p2) == Ordering::Greater { // arr[l] goes to (p2, p3), increase k
                        rotate3(arr, [k, l, m]);
                    } else if arr[l].cmp(&p1) == Ordering::Greater { // arr[l] goes to (p1, p2), increase j and k
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
        // k -= 1;
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
            conditional_partial_sort!(debug, arr);
            conditional_partial_sort!(release, arr);

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

// impl_4n_pivot_qsort!(4, quadro_pivot_quicksort, f32, f32x4);
impl_4n_pivot_qsort!(8, octal_pivot_quicksort, f32, f32x8);

pub fn quadro_pivot_quicksort_2(arr: &mut [f32]) {
    conditional_partial_sort!(debug, arr);
    conditional_partial_sort!(release, arr);
    // let n = arr.len();
    
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
            conditional_partial_sort!(debug, arr);
            conditional_partial_sort!(release, arr);
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