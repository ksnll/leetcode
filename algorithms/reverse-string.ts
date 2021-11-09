function reverseString(s: string[]): void {
  let left = 0,
    right = s.length - 1;
  while (left < right) {
    const toSwap = s[right];
    s[right] = s[left];
    s[left] = toSwap;
    right--;
    left++;
  }
}
