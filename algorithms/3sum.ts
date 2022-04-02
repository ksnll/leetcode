function threeSum(nums: number[]): number[][] {
  const numbers = [];
  nums = nums.sort((a, b) => a - b);
  for (let i = 0; i < nums.length; i++) {
    const target = 0 - nums[i];
    let left = i + 1,
      right = nums.length - 1;
    if (i > 0 && nums[i] === nums[i - 1]) continue;
    while (left < right) {
      if (nums[i] + nums[left] + nums[right] === 0) {
        numbers.push([nums[i], nums[left], nums[right]]);

        while (right > i && nums[right] === nums[right - 1]) {
          right--;
        }
        while (left < nums.length && nums[left] === nums[left + 1]) {
          left++;
        }
        left++;
        right--;
      } else if (nums[left] + nums[right] > target) {
        while (right > i && nums[right] === nums[right - 1]) {
          right--;
        }
        right--;
      } else if (nums[left] + nums[right] < target) {
        while (left < nums.length && nums[left] === nums[left + 1]) {
          left++;
        }
        left++;
      }
    }
  }
  return numbers;
}
