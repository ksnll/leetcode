function longestCommonPrefix(strs: string[]): string {
  let commonPrefix = "";
  let i = 0;
  while (true) {
    for (let g = 0; g < strs.length; g++) {
      if (!strs[g][i] || strs[g][i] !== strs[0][i]) return commonPrefix;
    }
    commonPrefix += strs[0][i];
    i++;
  }
}
