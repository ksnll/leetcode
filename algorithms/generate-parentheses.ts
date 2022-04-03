function generateParenthesis(n: number): string[] {
  const tree = "*" + "()".repeat(2 ** (n * 2) - 1);
  const results = [];
  const visit = (index, string = "") => {
    if (!tree[index]) return "";
    const left = index * 2 + 1;
    const right = index * 2 + 2;
    string = string + tree[index];
    if (!tree[left]) results.push(string.slice(1));
    let opened = 0;
    for (let i = 1; i < string.length; i++) {
      if (string[i] === "(") {
        opened++;
      } else {
        opened--;
      }
      if (opened < 0) return "";
    }
    visit(left, string);
    visit(right, string);
  };
  visit(0);
  return results.filter((result) => {
    let opened = 0;
    for (let i = 0; i < result.length; i++) {
      if (result[i] === "(") {
        opened++;
      } else {
        opened--;
      }
    }
    return opened === 0;
  });
}
