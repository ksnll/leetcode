function lengthOfLongestSubstring(s: string): number {
  let firstIndex = 0;
  let secondIndex = 1;
  let biggestString: string = "";
  if (s.length <= 1) return s.length;

  const hasDuplicates = (input: string): boolean => {
    for (let i = 0; i < input.length - 1; i++) {
      for (let g = i + 1; g < input.length; g++) {
        if (input[g] === input[i]) return true;
      }
    }
    return false;
  };

  while (secondIndex <= s.length && firstIndex < s.length) {
    if (!hasDuplicates(s.slice(firstIndex, secondIndex))) {
      biggestString = s.slice(firstIndex, secondIndex);
      secondIndex++;
      continue;
    }
    firstIndex++;
    secondIndex++;
  }
  return biggestString.length;
}
