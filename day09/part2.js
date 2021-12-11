const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let map = [];

rl.on("line", (line) => {
  map.push(line.split("").map((x) => parseInt(x)));
});

rl.on("close", () => {
  let lowPoints = [];

  for (let y = 0; y < map.length; y++) {
    for (let x = 0; x < map[y].length; x++) {
      if (isLowPoint(x, y, map)) {
        lowPoints.push([x, y]);
      }
    }
  }

  const score = lowPoints
    .map(([x, y]) => findBasin(x, y, map))
    .sort((a, b) => b - a)
    .slice(0, 3)
    .reduce((acc, next) => acc * next, 1);

  console.log(score);
});

function findAdjacents(x, y, map) {
  return [
    [x, y - 1], // up
    [x, y + 1], // down
    [x - 1, y], // left
    [x + 1, y], // right
  ].filter(
    ([x, y]) =>
      inBounds(x, 0, map[0].length - 1) && inBounds(y, 0, map.length - 1)
  );
}

function isLowPoint(x, y, map) {
  const adjacentCoords = findAdjacents(x, y, map);
  for ([adjX, adjY] of adjacentCoords) {
    if (isLower(x, y, adjX, adjY, map)) return false;
  }
  return true;
}

function isLower(x1, y1, x2, y2, map) {
  return map[y1][x1] >= map[y2][x2];
}

function inBounds(val, min, max) {
  return val >= min && val <= max;
}

function findBasin(x1, y1, map, basin = {}) {
  basin[`${x1},${y1}`] = 1;
  const adjs = findAdjacents(x1, y1, map);

  for (const [x2, y2] of adjs) {
    if (map[y2][x2] !== 9 && !isLower(x1, y1, x2, y2, map)) {
      findBasin(x2, y2, map, basin);
    }
  }

  return Object.keys(basin).length;
}
