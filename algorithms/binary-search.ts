function search(nums: number[], target: number): number {
  let left = 0;
  let right = nums.length;
  let pivot;

  while (left < right) {
    pivot = Math.floor((left + right - 1) / 2);
    console.log({ left, right, pivot });
    if (nums[pivot] === target) return pivot;

    if (nums[pivot] > target) right = pivot;

    if (nums[pivot] < target) left = pivot + 1;
  }
  return -1;
}
