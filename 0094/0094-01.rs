use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(n) = node {
                dfs(n.borrow().left.clone(), res);
                res.push(n.borrow().val);
                dfs(n.borrow().right.clone(), res);
            }
        }

        dfs(root, &mut res);
        return res;
    }
}
