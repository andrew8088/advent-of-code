const test = `target area: x=20..30, y=-10..-5`;
const input = `target area: x=206..250, y=-105..-57`;

function part1(input) {
  const target = parse(input);

  let maxY = Number.MIN_VALUE;

  let MAX = 1000;

  for (let x = 0; x < MAX; x++) {
    for (let y = 0; y < MAX; y++) {
      if (x === 0 && y === 0) continue;
      const positions = findPositions(target, [x, y]);

      if (positions) {
        const localMax = findMaxY(positions);

        if (localMax > maxY) {
          maxY = localMax;
          console.log(`new max: ${maxY} (${x}, ${y})`);
        }
      }
    }
  }
}

function part2(input) {
  const target = parse(input);

  let hitCount = 0;

  let MAX = 3000;

  for (let x = 1; x < MAX; x++) {
    for (let y = -1 * MAX; y < MAX; y++) {
      if (x === 0 && y === 0) continue;
      const positions = findPositions(target, [x, y]);

      if (positions) {
        hitCount++;
        console.log(`${hitCount} (${x}, ${y})`);
      }
    }
  }
}

function findPositions(target, velocity = [7, 2], pos = [0, 0]) {
  const positions = [pos];
  let tracker;

  do {
    [pos, velocity] = step(pos, velocity);
    positions.push(pos);
    tracker = compareToTarget(pos, target);
    // console.log(tracker, positions);
  } while (tracker === 1);

  return tracker === 0 ? positions : null;
}

function findMaxY(positions) {
  return positions.reduce((max, [_, y]) => Math.max(max, y), 0);
}

function parse(str) {
  return str
    .replace("target area: ", "")
    .split(", ")
    .map((x) => x.split("="))
    .reduce((acc, [key, range]) => {
      acc[key] = range
        .split("..")
        .map((x) => parseInt(x, 10))
        .sort((a, b) => a - b);
      return acc;
    }, {});
}

function step([xPos, yPos], [xVelocity, yVelocity]) {
  xPos += xVelocity;
  yPos += yVelocity;

  if (xVelocity) xVelocity > 0 ? xVelocity-- : xVelocity++;
  yVelocity--;

  return [
    [xPos, yPos],
    [xVelocity, yVelocity],
  ];
}

function compareToTarget([x, y], target) {
  const [minX, maxX] = target.x;
  const [minY, maxY] = target.y;

  // if y is after the minY, we are after the target
  // if x is after the maxX, we are after the target
  if (y < minY || x > maxX) return -1;

  // inside target
  if (x >= minX && x <= maxX && y >= minY && y <= maxY) return 0;

  return 1; // before target;
}

// part1(input);
part2(input);
