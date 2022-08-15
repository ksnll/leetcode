function removeDuplicates(nums: number[]): number {
  if (nums.length <= 1) {
    return nums.length;
  }
  let g = 1;
  for (let i = 1; i < nums.length; i++) {
    if (nums[i] !== nums[i - 1]) {
      nums[g++] = nums[i];
    }
  }
  return g;
}
