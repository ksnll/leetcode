struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut solution = vec![vec![1]];
        for i in 1..num_rows {
            let last_row = solution.get((i - 1) as usize).unwrap().clone();
            let new_row = Solution::build_row_from_previous(&last_row);
            solution.push(new_row);
        }
        solution
    }
    pub fn build_row_from_previous(previous: &Vec<i32>) -> Vec<i32> {
        let mut new_vec = vec![1];
        if previous.len() == 1 {
            vec![1, 1]
        } else {
            for i in 0..previous.len() - 1 {
                new_vec.push(previous.get(i).unwrap() + previous.get(i + 1).unwrap());
            }
            new_vec.push(1);
            new_vec
        }
    }
}

fn main() {
    println!("{:?}\n", Solution::generate(5));
}
