const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let lineNumber = 0;
const octoMap = {};

rl.on("line", (line) => {
  line.split("").forEach((item, idx) => {
    octoMap[`${lineNumber},${idx}`] = parseInt(item, 10);
  });
  lineNumber++;
});

rl.on("close", () => {
  let flashCount = 0;
  let steps = 100;
  while (steps--) {
    flashCount += step(octoMap);
    console.log(`step ${10 - steps}: ${flashCount} flashes`);
    print(octoMap);
  }
});

function step(octoMap) {
  const flashers = [];

  for (const key of Object.keys(octoMap)) {
    flashers.push(increment(octoMap, key));
  }

  return flash(octoMap, flashers.filter(Boolean));
}

// returns key if the cell must flash
function increment(octoMap, key, skipZero = false) {
  if (skipZero && octoMap[key] === 0) return;

  octoMap[key]++;

  if (octoMap[key] > 9) {
    octoMap[key] = 0;
    return key;
  }
}

function flash(octoMap, keys) {
  let flashCount = keys.length;
  let flashers = [];
  for (const key of keys) {
    const [row, col] = key.split(",").map((x) => parseInt(x, 10));
    const adjs = findAdjacents(row, col, octoMap).map((x) => x.join(","));

    for (const adj of adjs) {
      flashers.push(increment(octoMap, adj, true));
    }
  }

  flashers = flashers.filter(Boolean);

  if (flashers.length > 0) {
    flashCount += flash(octoMap, flashers);
  }

  return flashCount;
}

function findAdjacents(x, y) {
  return [
    [x, y - 1], // N
    [x + 1, y - 1], // NE
    [x + 1, y], // E
    [x + 1, y + 1], // SE
    [x, y + 1], // S
    [x - 1, y + 1], // SW
    [x - 1, y], // W
    [x - 1, y - 1], // NW
  ].filter(([x, y]) => inBounds(x, 0, 9) && inBounds(y, 0, 9));
}

const inBounds = (val, min, max) => val >= min && val <= max;

function print(octoMap) {
  const out = [];

  for (const key of Object.keys(octoMap)) {
    const [row, col] = key.split(",").map((x) => parseInt(x, 10));
    if (!out[row]) out[row] = [];
    out[row][col] = octoMap[key];
  }

  console.log(out.map((row) => row.join("")).join("\n"));
  console.log();
}
