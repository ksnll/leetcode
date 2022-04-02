function fourSum(nums: number[], target: number): number[][] {
  const solution = new Set<string>();
  nums = nums.sort((a, b) => a - b);
  for (let i = 0; i < nums.length - 3; i++) {
    for (let g = i + 1; g < nums.length - 2; g++) {
      let left = g + 1;
      let right = nums.length - 1;

      while (left < right) {
        if (nums[i] + nums[g] + nums[left] + nums[right] === target) {
          solution.add(`${nums[i]},${nums[g]},${nums[left]},${nums[right]}`);
          left++;
          right--;
        } else if (nums[i] + nums[g] + nums[left] + nums[right] < target) {
          left++;
        } else if (nums[i] + nums[g] + nums[left] + nums[right] > target) {
          right--;
        }
      }
    }
  }
  return [...solution].map((val) =>
    val.split(",").map((digit) => parseInt(digit, 10))
  );
}
