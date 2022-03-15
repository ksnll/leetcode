class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

const listToInt = (listValue: ListNode | null) => {
  let cur = listValue;
  let arrayValue = [];
  if (listValue === null) return 0;
  do {
    typeof cur!.val === "number" && arrayValue.push(cur!.val);
  } while (cur && cur.next && (cur = cur.next));
  return parseInt(arrayValue.reverse().join(""));
};
const intToList = (integerValue: number) => {
  const arrayValue = integerValue
    .toLocaleString("fullwide", { useGrouping: false })
    .split("")
    .reverse()
    .map((num) => parseInt(num, 10));
  const head = new ListNode(arrayValue[0]);
  let cur = head;
  for (let i = 1; i <= arrayValue.length; i++) {
    cur.next = new ListNode(arrayValue[i]);
    cur = cur.next;
  }
  return head;
};
function addTwoNumbers(
  l1: ListNode | null,
  l2: ListNode | null
): ListNode | null {
  const result = listToInt(l1) + listToInt(l2);
  return intToList(result);
}
