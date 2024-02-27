#[cfg(test)]
mod test {

    use crate::util::*;
    use crate::qsort::*;
    use crate::ty::FloatOrd;

    use introsort::sort_floats;

    #[test]
    fn test_pdq_sort() {
        let mut arr: Vec<FloatOrd> = default_vec(1_000);
        let dur = time_it(|| pdqsort::sort(&mut arr));
        assert_eq!(is_sorted(&arr), true);
        println!("pdq sort on 1k floats cost: {:?}ns", dur);

        let mut arr: Vec<FloatOrd> = default_vec(1_000_000);
        let dur = time_it(|| pdqsort::sort(&mut arr));
        assert_eq!(is_sorted(&arr), true);
        println!("pdq sort on 1k floats cost: {:?}ns", dur);
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
	fn test_triple_pivot_quicksort() {
		let mut arr = [0.5835883f32, 0.71631217, 0.8832437, 0.3880486, 0.021315217, 0.53300536, 0.20367897, 0.27174407, 0.96776694, 0.25351256];
		let dur = time_it(|| triple_pivot_quicksort(&mut arr));
        #[cfg(debug_assertions)]
        println!("arr: {:?}", arr);
		assert_eq!(is_sorted(&arr), true);
		println!("triple pivot quick sort cost: {:?}ns", dur);

        let mut arr = [0.96378565f32, 0.3529173, 0.547055, 0.9667923, 0.31933308, 0.8284124, 0.14586675, 0.48742378, 0.5244448, 0.28129673];
        let dur = time_it(|| triple_pivot_quicksort(&mut arr));
        #[cfg(debug_assertions)]
        println!("arr: {:?}", arr);
        assert_eq!(is_sorted(&arr), true);
        println!("triple pivot quick sort cost: {:?}ns", dur);

        let mut arr: Vec<f32> = default_vec(10);
        #[cfg(debug_assertions)]
        println!("arr before: {:?}", arr);
        let dur = time_it(|| triple_pivot_quicksort(&mut arr));
        #[cfg(debug_assertions)]
        println!("arr after: {:?}", arr);
        assert_eq!(is_sorted(&arr), true);
        println!("triple pivot quick sort on 10 floats cost: {:?}ns", dur);

        let mut arr: Vec<f32> = default_vec(1_00);
        let dur = time_it(|| triple_pivot_quicksort(&mut arr));
        #[cfg(debug_assertions)]
        println!("arr: {:?}", arr);
        assert_eq!(is_sorted(&arr), true);
        println!("triple pivot quick sort on 100 floats cost: {:?}ns", dur);

        let mut arr: Vec<f32> = default_vec(1_000_000);
        let dur = time_it(|| triple_pivot_quicksort(&mut arr));
        assert_eq!(is_sorted(&arr), true);
        println!("triple pivot quick sort on 1m floats cost: {:?}ns", dur);
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
        let arr: Vec<f32> = default_vec(1_000);

		let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_hoare_partition(&mut copy));
        assert_eq!(is_sorted(&copy), true);
		println!("quick sort 1-pivot (hoare partition) already sorted array cost: {:?}ns", dur);

        let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_lomuto_partition(&mut copy));
        assert_eq!(is_sorted(&copy), true);
		println!("quick sort 1-pivot (lomuto_partition) already sorted array cost: {:?}ns", dur);
        
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
    fn test_1m_array() {
        let arr: Vec<f32> = default_vec(1_000_000);
		let mut copy = arr.clone();
        let dur = time_it(|| copy.sort_by(|a, b| a.partial_cmp(b).unwrap()));
        println!("std sort 10m array cost: {:?}ns", dur);
        assert_eq!(is_sorted(&copy), true);

		let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_hoare_partition(&mut copy));
        println!("quick sort 1-pivot (hoare partition) 10m array cost: {:?}ns", dur);
        assert_eq!(is_sorted(&copy), true);

        let mut copy = arr.clone();
        let dur = time_it(|| quick_sort_lomuto_partition(&mut copy));
        println!("quick sort 1-pivot (lomuto_partition) 10m array cost: {:?}ns", dur);
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
		let dur = time_it(|| quadro_pivot_quicksort(&mut copy));
		println!("quick sort 4-pivot 10m array cost: {:?}ns", dur);
		assert_eq!(is_sorted(&copy), true);

        let mut copy = arr.clone();
		let dur = time_it(|| quadro_pivot_quicksort_2(&mut copy));
		println!("quick sort 4-pivot 10m array cost: {:?}ns", dur);
		assert_eq!(is_sorted(&copy), true);

    }

    #[test]
    fn test_sort_by_ptrs() {
        let v = vec![5, 7, 1, 3, 2, 4, 6];
        unsafe {
            // test with contiguous vector
            let raw_ptr: *const i32 = v.as_ptr();
            let len = v.len();
            let v_ptrs: Vec<*mut i32> = (0..len).map(|i| raw_ptr.add(i) as *mut i32).collect::<Vec<_>>();
            sort_ptrs(&v_ptrs);
            assert_eq!(v_ptrs.iter().map(|&x| *x).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5, 6, 7]);
        }
    }

    #[test]
    fn test_4_pivots_qsort() {
        let arr: Vec<f32> = default_vec(20);
		let mut copy = [0.0846004486, 0.324027538, 0.247496307, 0.346324563, 0.32713002, 0.524065554, 0.0999410152, 0.448016822, 0.157732904, 0.249729276, 0.360087872, 0.937479197];
        let dur = time_it(|| quadro_pivot_quicksort(&mut copy));
        println!("{:#?}", copy);
        println!("std sort 10m array cost: {:?}ns", dur);
        assert_eq!(is_sorted(&copy), true);
    }
}