function strStr(haystack: string, needle: string): number {
  if (needle.length < 1) return 0;
  let g = 0;
  for (let i = 0; i < haystack.length; i++) {
    let h = i;
    while (haystack[h++] === needle[g++]) {
      if (g >= needle.length) return i;
    }
    console.log("note the same", haystack[i], needle[g]);
    g = 0;
  }
  return -1;
}
