const findLongest = (s: string, centre: number, offset = false) => {
  let left = centre,
    right = centre;
  if (s[centre + 1] && s[centre + 1] === s[centre] && offset)
    right = centre + 1;
  while (
    s[left - 1] !== undefined &&
    s[right + 1] !== undefined &&
    s[left - 1] === s[right + 1]
  ) {
    left = left - 1;
    right = right + 1;
  }
  return s.slice(left, right + 1);
};
function longestPalindrome(s: string): string {
  let max = "";
  for (let i = 0; i < s.length; i++) {
    const longest = findLongest(s, i, false);
    const longestWithOffset = findLongest(s, i, true);
    if (longest.length > max.length) max = longest;
    if (longestWithOffset.length > max.length) max = longestWithOffset;
  }
  return max;
}
