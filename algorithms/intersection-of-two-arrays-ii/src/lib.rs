struct Solution;
impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort();
        nums2.sort();
        let mut nums1_iter = nums1.into_iter();
        let mut nums2_iter = nums2.into_iter();
        let mut shared_items: Vec<i32> = Vec::new();
        let mut num1 = nums1_iter.next();
        let mut num2 = nums2_iter.next();

        while let (Some(val1), Some(val2)) = (num1, num2) {
            if val1 == val2 {
                shared_items.push(val1.clone());
                num1 = nums1_iter.next();
                num2 = nums2_iter.next();
            } else {
                if num1 > num2 {
                    num2 = nums2_iter.next();
                } else {
                    num1 = nums1_iter.next();
                }
            }
        }

        shared_items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]);
        assert!(result.into_iter().all(|item| vec![2, 2].contains(&item)));

        let result = Solution::intersect(vec![4, 5, 9], vec![9, 4, 9, 8, 4]);
        assert!(result.into_iter().all(|item| vec![9, 4].contains(&item)));
    }
}
