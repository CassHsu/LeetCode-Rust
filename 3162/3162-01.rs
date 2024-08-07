impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        let n = nums1.len();
        let m = nums2.len();

        for i in 0..n {
            let n1 = nums1[i];
            for j in 0..m {
                let n2 = nums2[j];
                if n1 % (n2 * k) == 0 {
                    count += 1;
                }
            }
        }

        count
    }
}
