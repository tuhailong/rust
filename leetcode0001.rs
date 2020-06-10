/**
给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
示例:
给定 nums = [2, 7, 11, 15], target = 9
因为 nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]
链接：https://leetcode-cn.com/problems/two-sum
**/

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::BTreeMap;
        let mut map: BTreeMap<i32, usize> = BTreeMap::new();
        let mut ret: Vec<i32> = vec![-1; 2];
        for (idx, &num) in nums.iter().enumerate() {
            if let Some(&pos) = map.get(&(target - num)) {
                ret[0] = pos as i32;
                ret[1] = idx as i32;
                break;
            }
            map.insert(num, idx);
        }
        return ret;
    }
}
