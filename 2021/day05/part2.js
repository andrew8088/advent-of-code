const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

const lines = [];

rl.on("line", (line) => {
  lines.push(parseLine(line));
});

rl.on("close", () => {
  const points = lines.flatMap(getPoints);

  const pointCounts = {};
  let overlappingPointsCount = 0;

  for (const point of points) {
    const p = point.join(",");
    if (!pointCounts[p]) {
      pointCounts[p] = 0;
    }

    pointCounts[p]++;

    // this point is now overlapping, but only count it once.
    if (pointCounts[p] === 2) {
      overlappingPointsCount++;
    }
  }

  console.log(overlappingPointsCount);
});

function onlyStraightLines([[x1, y1], [x2, y2]]) {
  return x1 === x2 || y1 === y2;
}

function parseLine(line) {
  return line
    .split(" -> ")
    .map((coord) => coord.split(",").map((x) => parseInt(x, 10)));
}

function getPoints([[x1, y1], [x2, y2]]) {
  const points = [];
  if (x1 === x2) {
    let [yMin, yMax] = [y1, y2].sort((a, b) => a - b);

    while (yMin <= yMax) {
      points.push([x1, yMin++]);
    }
  } else if (y1 === y2) {
    let [xMin, xMax] = [x1, x2].sort((a, b) => a - b);

    while (xMin <= xMax) {
      points.push([xMin++, y1]);
    }
  } else {
    let [xTmp, yTmp] = [x1, y1];

    while (xTmp !== x2) {
      points.push([xTmp, yTmp]);

      if (xTmp < x2) xTmp++;
      else xTmp--;

      if (yTmp < y2) yTmp++;
      else yTmp--;
    }
    points.push([xTmp, yTmp]);
  }
  return points;
}
