#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    first: i32,  // value
    second: i32, // index
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.first.cmp(&other.first)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}



impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut my_ans: Vec<Pair> = Vec::new();

        for (i, x) in nums.iter().enumerate() {
            my_ans.push(Pair {
                second: i as i32, // index
                first: *x,        // value
            });
        }

    
        my_ans.sort();

        let mut left = 0;
        let mut right = my_ans.len() - 1;

        while left < right {
            let sum = my_ans[left].first + my_ans[right].first;
            if sum == target {
                return vec![my_ans[left].second, my_ans[right].second];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        vec![-1, -1] // Return invalid indices if not found
    }
}

