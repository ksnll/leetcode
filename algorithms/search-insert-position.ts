function searchInsert(nums: number[], target: number): number {
  let start = -1,
    end = nums.length,
    pivot: number;
  pivot = Math.floor((start + end) / 2);
  while (start < end) {
    pivot = Math.floor((start + end) / 2);
    if (nums[pivot] === target) return pivot;
    if (nums[pivot] > target) end = pivot;
    if (nums[pivot] < target) start = pivot;
    if (start + 1 === end) return end;
  }
  return pivot;
}

console.log(searchInsert([1, 3, 5, 6], 0));
