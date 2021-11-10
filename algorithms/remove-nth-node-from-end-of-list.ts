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

function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
  let cur = head;
  let length = 0;
  while (cur) {
    cur = cur?.next;
    length++;
  }
  cur = head;
  let indexToRemove = length - n;

  if (indexToRemove === 0 && cur) {
    head = cur.next || null;
  }

  while (cur && indexToRemove-- > 1) {
    cur = cur?.next;
  }

  if (cur?.next?.next) {
    cur.next = cur.next?.next || null;
  } else if (cur && cur.next) {
    cur.next = null;
  }

  return head;
}
