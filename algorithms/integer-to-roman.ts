const romanToInt = [
  { int: 1000, str: "M" },
  { int: 900, str: "CM" },
  { int: 500, str: "D" },
  { int: 400, str: "CD" },
  { int: 100, str: "C" },
  { int: 90, str: "XC" },
  { int: 50, str: "L" },
  { int: 40, str: "XL" },
  { int: 10, str: "X" },
  { int: 9, str: "IX" },
  { int: 5, str: "V" },
  { int: 4, str: "IV" },
  { int: 1, str: "I" },
];

function intToRoman(num: number): string {
  let output = "";
  while (num > 0) {
    romanToInt.forEach(({ int, str }) => {
      while (int <= num && num > 0) {
        num = num - int;
        output += str;
      }
    });
  }
  return output;
}
