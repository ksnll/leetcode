/**
 * The knows API is defined in the parent class Relation.
 * isBadVersion(version: number): boolean {
 *     ...
 * };
 */

var solution = function (isBadVersion: any) {
  return function (n: number): number {
    let pivot;
    let goodVersion = 0;
    let badVersion = n;
    while (goodVersion < badVersion) {
      pivot = Math.floor((badVersion + goodVersion) / 2);
      if (isBadVersion(pivot)) {
        badVersion = pivot;
      } else {
        goodVersion = pivot;
      }
      if (goodVersion + 1 === badVersion) return badVersion;
    }
  };
};
