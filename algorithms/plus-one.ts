function plusOne(digits: number[]): number[] {
  let g = digits.length - 1;
  let carry = true;
  while (g >= 0 && carry) {
    if (digits[g] + 1 > 9) {
      digits[g--] = 0;
      carry = true;
      continue;
    }
    digits[g] = digits[g] + 1;
    g--;
    carry = false;
  }
  if (carry) return [1, ...digits];
  return digits;
}
