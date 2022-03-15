class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}
function addTwoNumbers(
  l1: ListNode | null,
  l2: ListNode | null
): ListNode | null {
  return add(l1, l2, 0);

  function add(
    l1: ListNode | null,
    l2: ListNode | null,
    carried: number
  ): ListNode | null {
    const value1 = (l1 && l1.val) || 0;
    const value2 = (l2 && l2.val) || 0;
    const sum = value1 + value2 + carried;
    const carry = Math.floor(sum / 10);

    if (l1 || l2 || carried) {
      return {
        val: sum % 10,
        next: add(l1 && l1.next, l2 && l2.next, carry),
      };
    }
    return null;
  }
}
