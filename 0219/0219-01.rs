use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut m = HashMap::<i32, i32>::new();
        
        for i in 0..nums.len() {
            if m.contains_key(&nums[i]) && i - *m.get(&nums[i]).unwrap() as usize <= k as usize {
                return true;
            }   
            m.insert(nums[i], i as i32);
        }
        
        false
    }
}
