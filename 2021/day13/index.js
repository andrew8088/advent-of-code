const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

const coords = [];
const folds = [];

rl.on("line", (line) => {
  if (line.includes(",")) {
    coords.push(splitInts(line));
  } else if (line.includes("fold")) {
    folds.push(
      line
        .replace("fold along ", "")
        .split("=")
        .map((x) => (isNaN(parseInt(x, 10)) ? x : parseInt(x, 10)))
    );
  }
});

rl.on("close", () => {
  // part1(coords, folds);
  part2(coords, folds);
});

function part1(coords, folds) {
  fold(coords, ...folds[0]);
  const uniq = uniquePoints(coords);
  console.log(uniq.length);
}

function part2(coords, folds) {
  for (const f of folds) fold(coords, ...f);
  print(coords);
}

function uniquePoints(coords) {
  return Object.keys(
    coords.reduce((acc, next) => {
      acc[next] = 1;
      return acc;
    }, {})
  ).map(splitInts);
}

// if the fold axis is `x`, the x value changes
// if the fold axis is `y`, the y value changes
function fold(coords, foldAxis, foldIndex) {
  const changingIndex = foldAxis === "x" ? 0 : 1;

  for (let i = 0; i < coords.length; i++) {
    const delta = foldIndex - coords[i][changingIndex];

    if (delta < 0) {
      coords[i][changingIndex] = foldIndex + delta;
    }
  }
}

function print(coords) {
  const paper = [];

  let maxX = -1;
  let maxY = -1;

  for (const [x, y] of coords) {
    if (!paper[y]) paper[y] = [];
    paper[y][x] = "#";

    maxX = Math.max(maxX, x);
    maxY = Math.max(maxY, y);
  }

  let str = "";

  for (let i = 0; i <= maxY; i++) {
    if (!paper[i]) {
      str += Array(maxX + 1)
        .fill(".")
        .join("");
      str += "\n";
      continue;
    }

    for (let j = 0; j <= maxX; j++) {
      str += paper[i][j] ? "#" : ".";
    }
    str += "\n";
  }

  console.log(str);
}

function splitInts(line) {
  return line.split(",").map((x) => parseInt(x, 10));
}
