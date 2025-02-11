use std::collections::HashMap;

// O(n²) Approach
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
      for j in (i + 1)..nums.len() {
        if nums[i] + nums[j] == target {
          let vec_i32 = vec![i, j].iter().map(|f| *f as i32).collect::<Vec<i32>>();
          return vec_i32;
        }
      }
    }
    // Code should never arrives here
    unreachable!()
}

// O(n) Approach
pub fn two_sum_o_zero(nums: Vec<i32>, target: i32) -> Vec<i32> {
  let mut map = HashMap::new();

  for (i, &num) in nums.iter().enumerate() {
      let complement = target - num;
      if let Some(&j) = map.get(&complement) {
          return vec![j as i32, i as i32];
      }
      map.insert(num, i);
  }

  unreachable!();
}

#[cfg(test)]
mod tests {
  use super::*; 

  #[test]
  fn basic() {
    assert_eq!(two_sum(vec![2,7,11,15], 9), vec![0,1]);
    assert_eq!(two_sum(vec![3,2,4], 6), vec![1,2]);
    assert_eq!(two_sum(vec![3,3], 6), vec![0,1]);
  }

  #[test]
  fn basic2() {
    assert_eq!(two_sum_o_zero(vec![2,7,11,15], 9), vec![0,1]);
    assert_eq!(two_sum_o_zero(vec![3,2,4], 6), vec![1,2]);
    assert_eq!(two_sum_o_zero(vec![3,3], 6), vec![0,1]);
  }
}
