struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut ixm = m - 1;
        let mut ixn = n - 1;
        let mut current_index = m + n - 1;
        'outer: loop {
            if ixm < 0 && ixn < 0 {
                break;
            }
            while ixm < 0 {
                nums1[current_index as usize] = nums2[ixn as usize];
                current_index = current_index - 1;
                ixn = ixn - 1;
                if ixn < 0 {
                    break 'outer;
                }
            }
            while ixn < 0 {
                nums1[current_index as usize] = nums1[ixm as usize];
                current_index -= 1;
                ixm = ixm - 1;
                if ixm < 0 {
                    break 'outer;
                }
            }
            if nums2[ixn as usize] > nums1[ixm as usize] {
                nums1[current_index as usize] = nums2[ixn as usize];
                current_index -= 1;
                ixn = ixn - 1;
                continue;
            }
            if nums2[ixn as usize] <= nums1[ixm as usize] {
                nums1[current_index as usize] = nums1[ixm as usize];
                current_index -= 1;
                ixm = ixm - 1;
                continue;
            }
        }
    }
}

fn main() {
    // let nums1 = &mut vec![1, 2, 3, 0, 0, 0];
    // Solution::merge(nums1, 3, &mut vec![2, 5, 6], 3);
    let nums1 = &mut vec![1];
    Solution::merge(nums1, 1, &mut vec![], 0);
    println!("{:?}", nums1);
}
