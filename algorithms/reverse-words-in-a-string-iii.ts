function reverseWords(s: string): string {
  return s
    .split(" ")
    .map((input) => input.split("").reverse().join(""))
    .join(" ");
}
