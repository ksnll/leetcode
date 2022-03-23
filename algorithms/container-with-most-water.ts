function maxArea(height: number[]): number {
  let left = 0,
    right = height.length - 1;
  let maxLiters = 0;
  while (left < right) {
    const liters = (right - left) * Math.min(height[left], height[right]);
    if (liters > maxLiters) maxLiters = liters;
    if (height[right] > height[left]) left++;
    else right--;
  }
  return maxLiters;
}
