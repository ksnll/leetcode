function sortedSquares(nums: number[]): number[] {
  let firstPositiveIndex;
  for (
    firstPositiveIndex = 0;
    firstPositiveIndex < nums.length;
    firstPositiveIndex++
  ) {
    if (nums[firstPositiveIndex] >= 0) {
      break;
    }
  }
  console.log(firstPositiveIndex);
  const negatives: number[] = nums
    .slice(0, firstPositiveIndex)
    .reverse()
    .map((x) => x ** 2);
  const positives: number[] = nums
    .slice(firstPositiveIndex, nums.length)
    .map((x) => x ** 2);
  const sorted: number[] = [];
  while (negatives.length && positives.length) {
    sorted.push(
      negatives[0] > positives[0] ? positives.shift()! : negatives.shift()!
    );
  }
  while (negatives.length) {
    sorted.push(negatives.shift()!);
  }
  while (positives.length) {
    sorted.push(positives.shift()!);
  }

  return sorted;
}

console.log(sortedSquares([-4, -1, 0, 3, 10]));
