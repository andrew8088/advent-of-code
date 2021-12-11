const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let map = [];

rl.on("line", (line) => {
  map.push(line.split("").map((x) => parseInt(x)));
});

rl.on("close", () => {
  let riskSum = 0;

  for (let y = 0; y < map.length; y++) {
    for (let x = 0; x < map[y].length; x++) {
      const risk = maybeGetLowValue(x, y, map);

      if (typeof risk === "number") {
        riskSum += risk + 1;
      }
    }
  }

  console.log(riskSum);
});

function maybeGetLowValue(x, y, map) {
  const adjacentCoords = [
    [x, y - 1], // up
    [x, y + 1], // down
    [x - 1, y], // left
    [x + 1, y], // right
  ].filter(
    ([x, y]) =>
      inBounds(x, 0, map[0].length - 1) && inBounds(y, 0, map.length - 1)
  );

  const currVal = map[y][x];
  for ([adjX, adjY] of adjacentCoords) {
    const adjVal = map[adjY][adjX];

    if (currVal >= adjVal) return null;
  }

  return currVal;
}

function inBounds(val, min, max) {
  return val >= min && val <= max;
}
