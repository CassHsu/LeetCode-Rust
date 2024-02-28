impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let c1 = nums1.iter().filter(|n| nums2.contains(n)).count();
        let c2 = nums2.iter().filter(|n| nums1.contains(n)).count();
        vec![c1 as i32, c2 as i32]
    }
}
