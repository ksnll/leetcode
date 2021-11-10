/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */
type ListNode = {
  val: number;
  next: ListNode | null;
};

function middleNode(head: ListNode | null): ListNode | null {
  let cur = head;
  let length = 0;
  while (cur) {
    length++;
    cur = cur?.next;
  }
  let pivot = Math.ceil((length - 1) / 2);
  cur = head;

  while (pivot > 0 && cur) {
    cur = cur?.next;
    pivot--;
  }
  return cur;
}
