function checkInclusion(s1: string, s2: string): boolean {
  let leftIndex = 0,
    rightIndex = s1.length;
  const sortedString = s1.split("").sort().join("");
  let stringToCompare;
  while (rightIndex <= s2.length) {
    if (!stringToCompare) {
      stringToCompare = s2
        .slice(leftIndex, rightIndex)
        .split("")
        .sort()
        .join("");
    } else {
      stringToCompare = stringToCompare.slice(1, stringToCompare.length);
      const charToInsert = s2[rightIndex];
      let inserted = false;
      for (let i = 0; i < stringToCompare.length - 1; i++) {
        if (
          stringToCompare[i] < charToInsert &&
          stringToCompare[i + 1] > charToInsert
        ) {
          inserted = true;
        }
      }
      if (!inserted) {
        stringToCompare = stringToCompare + charToInsert;
      }
    }
    console.log({ stringToCompare, sortedString });
    if (stringToCompare === sortedString) return true;
    leftIndex++;
    rightIndex++;
  }
  return false;
}
