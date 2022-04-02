function mergeTwoLists(
  left: ListNode | null,
  right: ListNode | null
): ListNode | null {
  const result = new ListNode(-1);
  let cur = result;
  while (left && right) {
    if (left.val < right.val) {
      cur.next = new ListNode(left.val);
      left = left.next;
    } else {
      cur.next = new ListNode(right.val);
      right = right.next;
    }
    cur = cur.next;
  }
  cur.next = left || right;
  return result.next;
}
