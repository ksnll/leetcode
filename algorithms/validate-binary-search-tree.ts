class TreeNode {
  val: number;
  left: TreeNode | null;
  right: TreeNode | null;
  constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
  }
}
const visit = (
  node: TreeNode,
  state: { isValid: boolean; prev: number }
): any => {
  if (node.left) {
    visit(node.left, state);
  }
  state.isValid = state.isValid && node.val > state.prev;
  state.prev = node.val;
  if (node.right) {
    visit(node.right, state);
  }
};
function isValidBST(root: TreeNode | null): boolean {
  if (!root) return false;
  const state = { isValid: true, prev: Number.MIN_SAFE_INTEGER };
  visit(root, state);
  return state.isValid;
}
