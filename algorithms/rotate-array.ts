/**
 Do not return anything, modify nums in-place instead.
 */
function rotate(nums: number[], k: number): void {
  const reversePartial = (left: number, right: number) => {
    for (let i = 0; left + i < right - i; i++) {
      const swap = nums[left + i];
      nums[left + i] = nums[right - i];
      nums[right - i] = swap;
    }
  };
  reversePartial(nums.length - (k % nums.length), nums.length - 1);
  reversePartial(0, nums.length - (k % nums.length) - 1);
  reversePartial(0, nums.length - 1);
}
