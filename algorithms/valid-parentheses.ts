const invertParentheses = {
  "(": ")",
  "[": "]",
  "{": "}",
};
function isValid(s: string): boolean {
  const stack = [];
  for (let i = 0; i < s.length; i++) {
    if (["(", "{", "["].includes(s[i])) stack.push(s[i]);
    else {
      const char = stack.pop();
      if (s[i] !== invertParentheses[char]) return false;
    }
  }
  return stack.length === 0;
}
