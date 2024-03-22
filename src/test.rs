#[cfg(test)]
mod test {

    use crate::qsort::*;
    use crate::ty::FloatOrd;
    use crate::util::*;

    use introsort::sort_floats;
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
    fn test_introsort() {
        let mut arr: Vec<f32> = default_vec(1_000);
        let dur = time_it(|| sort_floats(&mut arr));
        assert_eq!(is_sorted(&arr), true);
        println!("introsort cost: {:?}ns", dur);

        let mut arr: Vec<f32> = default_vec(1_000_000);
        let dur = time_it(|| sort_floats(&mut arr));
        assert_eq!(is_sorted(&arr), true);
        println!("introsort on 1m floats cost: {:?}ns", dur);

        // TODO: test introsort on custom FloatOrd type
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
    fn test_10m_array() {
        let arr: Vec<FloatOrd> = default_vec(10_000_000);
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
        // let mut arr = default_vec(1_000_000);
        // let mut copy = arr.clone();
        // // transmute copy from Vec<FloatOrd> to Vec<f32>
        // let dur = time_it(|| quadro_pivot_quicksort(&mut copy));
        // println!("quick sort 4-pivot 10m array cost: {:?}ns", dur);
        // assert_eq!(is_sorted(&copy), true);

        // let mut copy = arr.clone();
        // let dur = time_it(|| quadro_pivot_quicksort_2(&mut copy));
        // println!("quick sort 4-pivot (2nd) 10m array cost: {:?}ns", dur);
        // assert_eq!(is_sorted(&copy), true);

        // let mut copy = arr.clone();
        // let dur = time_it(|| penta_pivot_quicksort(&mut copy));
        // println!("quick sort 5-pivot 10m array cost: {:?}ns", dur);
        // assert_eq!(is_sorted(&copy), true);

        // let mut copy = arr.clone();
        // let dur = time_it(|| hexa_pivot_quicksort(&mut copy));
        // println!("quick sort 6-pivot 10m array cost: {:?}ns", dur);
        // assert_eq!(is_sorted(&copy), true);

        // let mut copy = arr.clone();
        // let dur = time_it(|| hepta_pivot_quicksort(&mut copy));
        // println!("quick sort 7-pivot 10m array cost: {:?}ns", dur);
        // assert_eq!(is_sorted(&copy), true);

        // let mut copy = arr.clone();
        // let dur = time_it(|| octal_pivot_quicksort(&mut copy));
        // println!("quick sort 8-pivot 10m array cost: {:?}ns", dur);
        // assert_eq!(is_sorted(&copy), true);
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

    #[test]
    fn test_4_pivots_qsort() {
        let _arr: Vec<f32> = default_vec(20);
        let mut copy = [
            0.0846004486,
            0.324027538,
            0.247496307,
            0.346324563,
            0.32713002,
            0.524065554,
            0.0999410152,
            0.448016822,
            0.157732904,
            0.249729276,
            0.360087872,
            0.937479197,
        ];
        let dur = time_it(|| quadro_pivot_quicksort(&mut copy));
        println!("{:#?}", copy);
        println!("std sort 10m array cost: {:?}ns", dur);
        assert_eq!(is_sorted(&copy), true);
    }
}
