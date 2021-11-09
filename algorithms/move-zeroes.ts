/**
 Do not return anything, modify nums in-place instead.
 */
function moveZeroes(nums: number[]): void {
  for (let left = 0, right = 0; left < nums.length; left++) {
    while (nums[right] === 0 && right < nums.length) {
      right++;
    }
    nums[left] = nums[right] || 0;
    right++;
  }
}
