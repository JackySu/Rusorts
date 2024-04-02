#[cfg(test)]
mod test {

    use crate::qsort::*;
    use crate::ty::FloatOrd;
    use crate::util::*;

    use crumsort::ParCrumSort;

    use rayon::prelude::*;

    #[test]
    fn test_u32_and_f32_performance_on_10m_array() {
        let arr: Vec<u32> = default_vec(10_000_000);
        let mut copy = arr.clone();
        let dur = time_it(|| copy.par_sort_unstable());
        println!("pdq sort 10m u32 array cost: {:?}ns", dur);
        assert_eq!(is_sorted(&copy), true);

        let arr: Vec<FloatOrd> = default_vec(10_000_000);
        let mut copy = arr.clone();
        let dur = time_it(|| copy.par_sort_unstable());
        println!("pdq sort 10m f32 array cost: {:?}ns", dur);
        assert_eq!(is_sorted(&copy), true);
    }

    // TODO: Run more tests on more types
    #[test]
    fn test_10m_array() {
        fn type_name_of_val<T>() -> &'static str {
            std::any::type_name::<T>()
        }
        macro_rules! test_with_type {
            ($typ: ty) => {
                println!("{}", type_name_of_val::<$typ>());
                let arr: Vec<$typ> = default_vec(10_000_000);
                let mut copy = arr.clone();
                let dur = time_it(|| copy.sort_unstable());
                println!("std sort 10m array cost: {:?}ns", dur);
                assert_eq!(is_sorted(&copy), true);

                let mut copy = arr.clone();
                let dur = time_it(|| quick_sort_hoare_partition(&mut copy));
                println!(
                    "quick sort 1-pivot (hoare partition) 10m array cost: {:?}ns",
                    dur
                );
                assert_eq!(is_sorted(&copy), true);

                let mut copy = arr.clone();
                let dur = time_it(|| quick_sort_lomuto_partition(&mut copy));
                println!(
                    "quick sort 1-pivot (lomuto_partition) 10m array cost: {:?}ns",
                    dur
                );
                assert_eq!(is_sorted(&copy), true);

                let mut copy = arr.clone();
                let dur = time_it(|| double_pivot_quicksort(&mut copy));
                println!("quick sort 2-pivot 10m array cost: {:?}ns", dur);
                assert_eq!(is_sorted(&copy), true);

                let mut copy = arr.clone();
                let dur = time_it(|| triple_pivot_quicksort(&mut copy));
                println!("quick sort 3-pivot 10m array cost: {:?}ns", dur);
                assert_eq!(is_sorted(&copy), true);

                let mut copy = arr.clone();
                // transmute copy from Vec<FloatOrd> to Vec<f32>
                let dur = time_it(|| quad_pivot_quicksort(&mut copy));
                println!(
                    "<!> New impl quick sort 4-pivot 10m array cost: {:?}ns",
                    dur
                );
                assert_eq!(is_sorted(&copy), true);

                let mut copy = arr.clone();
                let dur = time_it(|| quick_sort_hoare_partition_block(&mut copy));
                //dbg!(&copy);
                assert_eq!(is_sorted(&copy), true);
                println!(
                    "quick sort 1-pivot (hoare partition block) 10m array cost: {:?}ns",
                    dur
                );

                let mut copy = arr.clone();
                let dur = time_it(|| quick_sort_lomuto_partition_block(&mut copy));
                assert_eq!(is_sorted(&copy), true);
                println!(
                    "quick sort 1-pivot (lomuto partition block) 10m array cost: {:?}ns",
                    dur
                );

                let mut copy = arr.clone();
                let dur = time_it(|| double_pivot_quicksort_lomuto_partition_block(&mut copy));
                assert_eq!(is_sorted(&copy), true);
                println!(
                    "quick sort 2-pivot (block partition) 10m array cost: {:?}ns",
                    dur
                );

                let mut copy = arr.clone();
                let dur = time_it(|| double_pivot_quicksort_new_partition_block(&mut copy));
                assert_eq!(is_sorted(&copy), true);
                println!(
                    "quick sort 2-pivot (new block partition) 10m array cost: {:?}ns",
                    dur
                );
                println!();
            };
        }
        test_with_type!(u32);
        test_with_type!(FloatOrd);
    }

    #[test]
    fn test_crum_sort_vs_pdq_sort() {
        let mut arr: Vec<FloatOrd> = default_vec(1_000);
        let dur = time_it(|| arr.par_crumsort());
        assert_eq!(is_sorted(&arr), true);
        println!("crum sort on 1k floats cost: {:?}ns", dur);

        let mut arr: Vec<FloatOrd> = default_vec(1_000_000);
        let dur = time_it(|| arr.par_crumsort());
        assert_eq!(is_sorted(&arr), true);
        println!("crum sort on 1m floats cost: {:?}ns", dur);

        let mut arr: Vec<FloatOrd> = default_vec(1_000);
        let dur = time_it(|| arr.par_sort_unstable());
        assert_eq!(is_sorted(&arr), true);
        println!("pdq sort on 1k floats cost: {:?}ns", dur);

        let mut arr: Vec<FloatOrd> = default_vec(1_000_000);
        let dur = time_it(|| arr.par_sort_unstable());
        assert_eq!(is_sorted(&arr), true);
        println!("pdq sort on 1m floats cost: {:?}ns", dur);

        let mut arr: Vec<u32> = default_vec(1_000);
        let dur = time_it(|| arr.par_sort_unstable());
        assert_eq!(is_sorted(&arr), true);
        println!("pdq sort on 1k u32 cost: {:?}ns", dur);

        let mut arr: Vec<u32> = default_vec(1_000_000);
        let dur = time_it(|| arr.par_sort_unstable());
        assert_eq!(is_sorted(&arr), true);
        println!("pdq sort on 1m u32 cost: {:?}ns", dur);

        let mut arr: Vec<u32> = default_vec(1_000);
        let dur = time_it(|| arr.par_crumsort());
        assert_eq!(is_sorted(&arr), true);
        println!("crumsort on 1k u32 cost: {:?}ns", dur);

        let mut arr: Vec<u32> = default_vec(1_000_000);
        let dur = time_it(|| arr.par_crumsort());
        assert_eq!(is_sorted(&arr), true);
        println!("crumsort on 1m u32 cost: {:?}ns", dur);
    }

    #[test]
    fn test_double_pivot_quicksort_new_partition_block() {
        let arr: Vec<FloatOrd> = default_vec(10_000_000);

        let mut copy = arr.clone();
        let dur = time_it(|| double_pivot_quicksort(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!("quick sort 2-pivot 10m array cost: {:?}ns", dur);

        let mut copy = arr.clone();
        let dur = time_it(|| double_pivot_quicksort_lomuto_partition_block(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 2-pivot (lomuto block partition) 10m array cost: {:?}ns",
            dur
        );

        let mut copy = arr.clone();
        let dur = time_it(|| double_pivot_quicksort_new_partition_block(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 2-pivot (new block partition) 10m array cost: {:?}ns",
            dur
        );
    }

    #[test]
    fn test_double_pivot_quicksort_lomuto_partition_block() {
        let arr: Vec<FloatOrd> = default_vec(1_000);

        let mut copy = arr.clone();
        let dur = time_it(|| double_pivot_quicksort(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!("quick sort 2-pivot 1k array cost: {:?}ns", dur);

        let mut copy = arr.clone();
        let dur = time_it(|| double_pivot_quicksort_lomuto_partition_block(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 2-pivot (block partition) 1k array cost: {:?}ns",
            dur
        );

        let arr: Vec<FloatOrd> = default_vec(1_000_000);

        let mut copy = arr.clone();
        let dur = time_it(|| double_pivot_quicksort(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!("quick sort 2-pivot 1m array cost: {:?}ns", dur);

        let mut copy = arr.clone();
        let dur = time_it(|| double_pivot_quicksort_lomuto_partition_block(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 2-pivot (block partition) 1m array cost: {:?}ns",
            dur
        );
    }

    #[test]
    fn test_lomuto_partition_and_lomuto_partition_block() {
        let arr: Vec<FloatOrd> = default_vec(10_000_000);
        let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_lomuto_partition(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 1-pivot (lomuto partition) 1m array cost: {:?}ns",
            dur
        );
        let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_lomuto_partition_block(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 1-pivot (lomuto partition block) 1m array cost: {:?}ns",
            dur
        );
    }

    #[test]
    fn test_hoare_partition_and_hoare_partition_block() {
        let arr: Vec<FloatOrd> = default_vec(10_000_000);
        let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_hoare_partition(&mut copy));
        // assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 1-pivot (hoare partition) 1m array cost: {:?}ns",
            dur
        );
        let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_hoare_partition_block(&mut copy));
        //dbg!(&copy);
        assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 1-pivot (hoare partition block) 1m array cost: {:?}ns",
            dur
        );
    }

    #[test]
    fn test_unstable_sort() {
        let mut arr: Vec<f32> = default_vec(1_000);
        let dur = time_it(|| arr.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()));
        assert_eq!(is_sorted(&arr), true);
        println!("unstable sort on 1k floats cost: {:?}ns", dur);

        let mut arr: Vec<f32> = default_vec(1_000_000);
        let dur = time_it(|| arr.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()));
        assert_eq!(is_sorted(&arr), true);
        println!("unstable sort on 1m floats cost: {:?}ns", dur);
    }

    #[test]
    fn test_1k_array() {
        let arr: Vec<FloatOrd> = default_vec(1_000);

        let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_hoare_partition(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 1-pivot (hoare partition) already sorted array cost: {:?}ns",
            dur
        );

        let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_lomuto_partition(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!(
            "quick sort 1-pivot (lomuto_partition) already sorted array cost: {:?}ns",
            dur
        );

        let mut copy = arr.clone();
        let dur = time_it(|| double_pivot_quicksort(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!("quick sort 2-pivot already sorted array cost: {:?}ns", dur);

        let mut copy = arr.clone();
        let dur = time_it(|| triple_pivot_quicksort(&mut copy));
        assert_eq!(is_sorted(&copy), true);
        println!("quick sort 3-pivot already sorted array cost: {:?}ns", dur);
    }

    #[test]
    fn test_hoare_partition() {
        let mut arr: Vec<FloatOrd> = default_vec(1_000);
        let dur = time_it(|| quick_sort_hoare_partition(&mut arr));
        assert_eq!(is_sorted(&arr), true);
        println!(
            "quick sort 1-pivot (hoare partition) 1k array cost: {:?}ns",
            dur
        );

        let mut arr: Vec<FloatOrd> = default_vec(1_000_000);
        let dur = time_it(|| quick_sort_hoare_partition(&mut arr));
        assert_eq!(is_sorted(&arr), true);
        println!(
            "quick sort 1-pivot (hoare partition) 1m array cost: {:?}ns",
            dur
        );
    }

    // #[test]
    // fn test_sort_by_ptrs() {
    //     let v = vec![5, 7, 1, 3, 2, 4, 6];
    //     unsafe {
    //         // test with contiguous vector
    //         let raw_ptr: *const i32 = v.as_ptr();
    //         let len = v.len();
    //         let v_ptrs: Vec<*mut i32> = (0..len).map(|i| raw_ptr.add(i) as *mut i32).collect::<Vec<_>>();
    //         // sort_ptrs(&v_ptrs);
    //         assert_eq!(v_ptrs.iter().map(|&x| *x).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6, 7]);
    //     }
    // }

    // TODO: test with custom FloatOrd type
    #[test]
    fn test_real_quad_pivot_qsort() {
        for _ in 0..100 {
            let mut some_vec: Vec<FloatOrd> = default_vec(5000);
            // dbg!(&some_vec);
            quad_pivot_quicksort(&mut some_vec);
            assert_eq!(is_sorted(&some_vec), true);
        }
    }
}
