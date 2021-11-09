function sortedSquares(nums: number[]): number[] {
  let firstPositiveIndex = nums.findIndex((x) => x > 0);
  if (firstPositiveIndex < 0) firstPositiveIndex = nums.length;
  let positiveIndex = firstPositiveIndex,
    negativeIndex = firstPositiveIndex - 1;
  const sorted = new Array(nums.length);
  let i = 0;
  while (negativeIndex >= 0 && positiveIndex < nums.length) {
    sorted[i++] =
      Math.abs(nums[negativeIndex]) > Math.abs(nums[positiveIndex])
        ? nums[positiveIndex++] ** 2
        : nums[negativeIndex--] ** 2;
  }
  while (negativeIndex >= 0) {
    typeof nums[negativeIndex] === "number" &&
      (sorted[i++] = nums[negativeIndex] ** 2);
    negativeIndex--;
  }
  while (positiveIndex < nums.length) {
    typeof nums[positiveIndex] === "number" &&
      (sorted[i++] = nums[positiveIndex] ** 2);
    positiveIndex++;
  }

  return sorted;
}
