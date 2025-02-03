fn three_sum(nums: &mut Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result: Vec<Vec<i32>> = vec![];
    for element in 0..nums.len() {
        let current = nums[element];
        let mut smallest: usize = element + 1;
        let mut largest: usize = nums.len() - 1;

        while smallest < largest {
            let current_sum = current + nums[smallest] + nums[largest];
            if current_sum == target {
                result.push(vec![current, nums[smallest], nums[largest]]);
                smallest += 1;
                largest -= 1;
            } else if current_sum < target {
                smallest += 1;
            } else {
                largest -= 1;
            }
        }
    }
    result
}

fn main() {
    let mut test_cases: Vec<i32> = vec![12, 3, 1, 2, -6, 5, 0, -8, -1, 6];
    let target: i32 = 0;
    let test: Vec<Vec<i32>> = three_sum(&mut test_cases, target);
    println!("{test_cases:?}");
    println!("Unique triplets that sum to zero:");
    println!("{:?}", test);
}
