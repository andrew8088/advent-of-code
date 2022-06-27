const s1 = [
  [0, 2],
  [4, 1],
  [3, 3],
];
const s2 = [
  [-1, -1],
  [-5, 0],
  [-2, 1],
];

const offsets = [];

for (const b1 of s1) {
  for (const b2 of s2) {
    const deltaX = b1[0] - b2[0];
    const deltaY = b1[1] - b2[1];
    offsets.push([deltaX, deltaY]);
  }
}

for (const [offsetX, offsetY] of offsets) {
  const s2Offset = s2.map(([x, y]) => [x + offsetX, y + offsetY]);
  const overlap = intersection(s1, s2Offset);
  console.log(offsetX, offsetY, overlap);
}

function intersection(arrA, arrB) {
  const intersection = [];

  for (let i = 0; i < arrA.length; i++) {
    for (let j = 0; j < arrB.length; j++) {
      if (arrA[i][0] === arrB[j][0] && arrA[i][1] === arrB[j][1]) {
        intersection.push([...arrA[i]]);
      }
    }
  }

  return intersection;
}
