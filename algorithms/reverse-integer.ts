function reverse(x: number): number {
  const positive = x >= 0;
  x = Math.abs(x);
  let ret = 0;
  while (x > 0) {
    let digit = x % 10;
    x = Math.floor(x / 10);
    if (ret > Math.ceil(2 ** 31 / 10 - digit)) return 0;
    ret = ret * 10 + digit;
  }
  return positive ? ret : -ret;
}
