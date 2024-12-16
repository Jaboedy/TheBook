fn main() {
    let num_vec: Vec<i32> = Vec::from([1, 3, 5, 2, 8, 6]);
    let answer = two_sum(num_vec, 8);
    println!("Answer: {}, {}", answer[0], answer[1]);
}

fn two_sum(nums: Vec<i32>, target: i32) -> [i32; 2] {
    for (index, val) in nums[..].iter().enumerate() {
        for val2 in nums[index..].iter() {
            let x = val + val2;
            println!("{val} + {val2} = {x}");
            if x == target {
                return [*val, *val2];
            }
        }
    }
    [0, 0]
}
