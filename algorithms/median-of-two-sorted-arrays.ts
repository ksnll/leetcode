function findMedianSortedArrays(nums1: number[], nums2: number[]): number {
  let left = 0,
    right = 0;
  const mergedArray = [];
  let mergedArrayIndex = 0;
  while (nums1[left] != null && nums2[right] != null) {
    if (nums1[left] > nums2[right]) {
      mergedArray[mergedArrayIndex++] = nums2[right++];
    } else {
      mergedArray[mergedArrayIndex++] = nums1[left++];
    }
  }
  while (nums1[left] != null) {
    mergedArray[mergedArrayIndex++] = nums1[left++];
  }
  while (nums2[right] != null) {
    mergedArray[mergedArrayIndex++] = nums2[right++];
  }
  if (mergedArray.length % 2 !== 0) {
    return mergedArray[Math.floor(mergedArray.length / 2)];
  } else {
    return (
      (mergedArray[Math.floor(mergedArray.length / 2) - 1] +
        mergedArray[Math.floor(mergedArray.length / 2)]) /
      2
    );
  }
}
