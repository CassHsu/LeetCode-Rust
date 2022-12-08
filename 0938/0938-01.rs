use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(node) = root {
            let mut n = node.borrow();
            let mut v = 0;
            if low <= n.val && n.val <= high {
                v = n.val;
            }

            let l = Solution::range_sum_bst(n.left.clone(), low, high);
            let r = Solution::range_sum_bst(n.right.clone(), low, high);

            return l + r + v;
        }

        0
    }
}
