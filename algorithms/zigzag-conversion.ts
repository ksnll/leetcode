function convert(s: string, numRows: number): string {
  const rows: string[][] = [];
  let reverse = false;
  let rowNum = 0;
  if (numRows === 1 || s.length <= 1) return s;
  for (let i = 0; i < numRows; i++) {
    rows[i] = [] as string[];
  }
  for (let i = 0; i < s.length; i++) {
    rows[rowNum].push(s[i]);
    if (rowNum + 1 === numRows) {
      reverse = true;
    } else if (rowNum === 0) {
      reverse = false;
    }
    reverse ? rowNum-- : rowNum++;
  }
  return rows.map((row) => row.join("")).join("");
}
