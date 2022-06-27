const { getInput, getTestInput } = require("./input");

function part1(input) {
  input = parse(input);

  let sum = input.shift();

  for (const num of input) {
    sum = add(sum, num);
  }

  console.log(magnitude(sum));
}

function part2(input) {
  input = parse(input);
  let maxMag = -1;

  for (let i = 0; i < input.length; i++) {
    for (let j = 0; j < input.length; j++) {
      if (i === j) continue;
      const sum1 = add(input[i], input[j]);
      const mag1 = magnitude(sum1);
      if (mag1 > maxMag) maxMag = mag1;

      const sum2 = add(input[j], input[i]);
      const mag2 = magnitude(sum2);
      if (mag2 > maxMag) maxMag = mag2;
    }
  }
  console.log(maxMag);
}

// part1(getTestInput());
part2(getInput());

function add(a, b) {
  a = JSON.parse(JSON.stringify(a));
  b = JSON.parse(JSON.stringify(b));
  return reduce([a, b]);
}

function reduce(num) {
  const ePath = getExplodePath(num);

  if (ePath) {
    num = explode(num, ePath);
    return reduce(num);
  }

  const sPath = getSplitPath(num);
  if (sPath) {
    num = split(num, sPath);
    return reduce(num);
  }

  return num;
}

function magnitude(num) {
  if (typeof num === "number") return num;

  return magnitude(num[0]) * 3 + magnitude(num[1]) * 2;
}

module.exports = {
  add,
  reduce,
  magnitude,
};

function getExplodePath(num, path = []) {
  if (!Array.isArray(num)) {
    if (path.length > 4) {
      path.pop();
      return path;
    }
    return null;
  }
  return (
    getExplodePath(num[0], path.concat([0])) ||
    getExplodePath(num[1], path.concat([1]))
  );
}

function explode(num, path) {
  const [left, right] = path.reduce((acc, next) => acc[next], num);

  addLeftOfPath(num, path, left);
  addRightOfPath(num, path, right);
  replacePathWithZero(num, path);

  return num;
}

function replacePathWithZero(num, path) {
  const lastPos = path.pop();
  let pointer = path.reduce((acc, next) => acc[next], num);
  pointer[lastPos] = 0;
}

function addLeftOfPath(num, path, left) {
  // find the location where this path last branched right; we need to go left at that junction
  for (let i = path.length - 1; i >= 0; i--) {
    if (path[i] === 1) {
      path = path.slice(0, i);
      break;
    }
  }
  let pointer = path.reduce((acc, next) => acc[next], num);

  // go left
  if (Array.isArray(pointer[0])) {
    pointer = pointer[0];
    path.push(0);
  } else {
    pointer[0] = pointer[0] + left;
    return;
  }

  // now go right until you reach a normal number
  while (Array.isArray(pointer[1])) {
    pointer = pointer[1];
    path.push(1);
  }

  pointer[1] = pointer[1] + left;
}

function addRightOfPath(num, path, right) {
  // find the location where this path last branched left; we need to go right at that junction
  for (let i = path.length - 1; i >= 0; i--) {
    if (path[i] === 0) {
      path = path.slice(0, i);
      break;
    }
  }

  let pointer = path.reduce((acc, next) => acc[next], num);

  // go right
  if (Array.isArray(pointer[1])) {
    pointer = pointer[1];
    path.push(1);
  } else {
    pointer[1] = pointer[1] + right;
    return;
  }

  // now go left until you reach a normal number
  while (Array.isArray(pointer[0])) {
    pointer = pointer[0];
    path.push(0);
  }

  pointer[0] = pointer[0] + right;
}

function getSplitPath(num, path = []) {
  if (!Array.isArray(num)) {
    return num >= 10 ? path : null;
  }
  return (
    getSplitPath(num[0], path.concat([0])) ||
    getSplitPath(num[1], path.concat([1]))
  );
}

function split(num, path) {
  const lastPos = path.pop();
  let pointer = path.reduce((acc, next) => acc[next], num);

  const val = pointer[lastPos];
  pointer[lastPos] = [Math.floor(val / 2), Math.ceil(val / 2)];

  return num;
}

function parse(input) {
  return input.trim().split("\n").map(eval);
}
