impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut char_map: Vec<i32> = vec![0; 26];
        
        for (i, (sc, tc)) in s.chars().zip(t.chars()).enumerate() {
            let i = i as i32;
            
            let pos = (sc as u8 - 97) as usize;
            char_map[pos] = char_map[pos].abs_diff(i) as i32;
            
            let pos = (tc as u8 - 97) as usize;
            char_map[pos] = char_map[pos].abs_diff(i) as i32; 
        }
        
        char_map.iter().sum::<i32>() as _
    }
}
