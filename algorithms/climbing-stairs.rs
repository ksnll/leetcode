struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }

        let mut stairs_solutions = std::collections::HashMap::new();
        stairs_solutions.insert(1, 1);
        stairs_solutions.insert(2, 2);
        stairs_solutions.insert(3, 3);
        for i in 3..=n {
            let prev = stairs_solutions.get(&(i - 1)).unwrap();
            let prev_prev = stairs_solutions.get(&(i - 2)).unwrap();

            stairs_solutions.insert(i, prev + prev_prev);
        }
        *stairs_solutions.get(&n).unwrap()
    }
}

fn main() {
    let target: Vec<String> = std::env::args().collect();
    println!("{:?}", Solution::climb_stairs(target[1].parse().unwrap()));
}
