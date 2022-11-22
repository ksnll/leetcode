struct Solution;
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        if nums.len() == 1 {
            if nums[0] == target {
                return vec![0, 0];
            } else {
                return vec![-1, -1];
            }
        }
        let mut low = 0;
        let mut high = nums.len() - 1;
        let mut half = (low + high) / 2;
        let mut pivot = nums[half];

        let mut start = 0;
        let mut stop = high;
        let mut found = false;

        while low <= high {
            dbg!(low);
            dbg!(high);
            dbg!(pivot);
            dbg!(target);
            match pivot.cmp(&target) {
                std::cmp::Ordering::Equal => {
                    start = half;
                    stop = half;
                    found = true;
                    break;
                }
                std::cmp::Ordering::Less => low = half as usize + 1,
                std::cmp::Ordering::Greater => {
                    if half <= 0 {
                        break;
                    };
                    high = half as usize - 1
                }
            }
            half = (low + high) / 2;
            pivot = nums[half];
        }
        if !found {
            return vec![-1, -1];
        }
        while stop < nums.len() && stop + 1 < nums.len() && nums[stop + 1] == target {
            stop = stop + 1;
        }
        while start > 0 && start - 1 >= 0 && nums[start - 1] == target {
            start = start - 1;
        }
        return vec![start as i32, stop as i32];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8);
        // assert_eq!(result, vec![3, 4]);
        //
        // let result = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6);
        // assert_eq!(result, vec![-1, -1]);
        //
        // let result = Solution::search_range(vec![], 0);
        // assert_eq!(result, vec![-1, -1]);
        //
        // let result = Solution::search_range(vec![1], 0);
        // assert_eq!(result, vec![-1, -1]);
        //
        // let result = Solution::search_range(vec![2, 2], 1);
        // assert_eq!(result, vec![-1, -1]);

        let result = Solution::search_range(vec![1, 1, 2], 1);
        assert_eq!(result, vec![0, 1]);
    }
}
