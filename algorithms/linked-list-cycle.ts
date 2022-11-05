function hasCycle(head: ListNode | null): boolean {
  let next = head;
  if (!head || !head.next) return false;
  while (true) {
    (next as any).visited = true;
    next = next.next;

    if (!next) return false;
    if ((next as any).visited) return true;
  }
};
