function containsDuplicate(nums: number[]): boolean {
  const charMap = new Set();
  for (let i = 0; i < nums.length; i++) {
    if (charMap.has(nums[i])) return true;
    charMap.add(nums[i]);
  }
  return false;
}
