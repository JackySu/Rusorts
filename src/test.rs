#[cfg(test)]
mod test {

    use crate::util::{default_vec, is_sorted, time_it};
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
		assert_eq!(is_sorted(&arr), true);
		println!("triple pivot quick sort cost: {:?}ns", dur);

        let mut arr: Vec<f32> = default_vec(1_000);
        let dur = time_it(|| triple_pivot_quicksort(&mut arr));
        assert_eq!(is_sorted(&arr), true);
        println!("triple pivot quick sort on 1k floats cost: {:?}ns", dur);

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
        // copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let dur = time_it(|| quick_sort(&mut copy));
        assert_eq!(is_sorted(&copy), true);
		println!("quick sort 1-pivot already sorted array cost: {:?}ns", dur);
        
        let mut copy = arr.clone();
        // copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let dur = time_it(|| double_pivot_quicksort(&mut copy));
        assert_eq!(is_sorted(&copy), true);
		println!("quick sort 2-pivot already sorted array cost: {:?}ns", dur);
        
		let mut copy = arr.clone();
		// copy.sort_by(|a, b| a.partial_cmp(b).unwrap());
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
        let dur = time_it(|| quick_sort(&mut copy));
        println!("quick sort 1-pivot 10m array cost: {:?}ns", dur);
        assert_eq!(is_sorted(&copy), true);

		let mut copy = arr.clone();
        let dur = time_it(|| double_pivot_quicksort(&mut copy));
        println!("quick sort 2-pivot 10m array cost: {:?}ns", dur);
        assert_eq!(is_sorted(&copy), true);

		let mut copy = arr.clone();
		let dur = time_it(|| triple_pivot_quicksort(&mut copy));
		println!("quick sort 3-pivot 10m array cost: {:?}ns", dur);
		assert_eq!(is_sorted(&copy), true);
    }
}