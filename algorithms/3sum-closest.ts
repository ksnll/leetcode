function threeSumClosest(nums: number[], target: number): number {
  let closest;
  nums = nums.sort((a, b) => a - b);
  for (let i = 0; i < nums.length - 2; i++) {
    let left = i + 1;
    let right = nums.length - 1;
    while (left < right) {
      const sumTry = nums[i] + nums[left] + nums[right];
      if (closest === undefined) closest = sumTry;
      if (Math.abs(sumTry - target) < Math.abs(closest - target))
        closest = sumTry;
      if (sumTry < target) left++;
      else if (sumTry > target) right--;
      else if (sumTry === target) return sumTry;
    }
  }
  return closest;
}
