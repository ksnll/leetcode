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

function getIntersectionNode(headA: ListNode | null, headB: ListNode | null): ListNode | null {
  while (headA) {
    (headA as any).visitedByA = true;
    headA = headA.next;
  }
  while (headB) {
    if ((headB as any).visitedByA) return headB;
    headB = headB.next;
  }
  return null;
};
