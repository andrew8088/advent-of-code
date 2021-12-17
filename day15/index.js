const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

const grid = [];

rl.on("line", (line) => grid.push(line.split("").map((x) => parseInt(x, 10))));

rl.on("close", () => {
  // part1(grid);
  part2(grid);
});

function part1(grid) {
  const distances = dijkstra(grid, [0, 0]);
  console.log(distances.pop().pop());
}

function part2(grid) {
  const bigGrid = embiggen(grid);
  console.log(bigGrid.length);
  const distances = dijkstra(bigGrid, [0, 0]);
  console.log(distances.pop().pop());
}

const toKey = (coord) => coord.join(",");

function dijkstra(grid, start) {
  const distances = [];

  for (let i = 0; i < grid.length; i++) {
    distances[i] = [];
    for (let j = 0; j < grid[i].length; j++) {
      distances[i][j] = Number.MAX_VALUE;
    }
  }

  distances[start[1]][start[0]] = 0;

  let visitedCount = 0;
  const visited = {};

  while (true) {
    if (visitedCount % 100 === 0) console.log(visitedCount);
    let shortestDistance = Number.MAX_VALUE;
    let shortestX = -1;
    let shortestY = -1;

    for (let i = 0; i < grid.length; i++) {
      for (let j = 0; j < grid[i].length; j++) {
        if (distances[i][j] < shortestDistance && !visited[toKey([j, i])]) {
          shortestDistance = distances[i][j];
          shortestX = j;
          shortestY = i;
        }
      }
    }

    if (shortestX === -1 && shortestY === -1) {
      return distances;
    }

    const adjs = findAdjacents(
      shortestX,
      shortestY,
      grid[0].length - 1,
      grid.length - 1
    );

    for (const [x, y] of adjs) {
      if (distances[y][x] > distances[shortestY][shortestX] + grid[y][x]) {
        distances[y][x] = distances[shortestY][shortestX] + grid[y][x];
      }
    }

    visited[toKey([shortestX, shortestY])] = true;
    visitedCount++;
  }
}

const inBounds = (val, min, max) => val >= min && val <= max;

function findAdjacents(x, y, maxX, maxY) {
  return [
    [x, y - 1], // N
    [x + 1, y], // E
    [x, y + 1], // S
    [x - 1, y], // W
  ].filter(([x, y]) => inBounds(x, 0, maxX) && inBounds(y, 0, maxY));
}

function defaultPopulateCell(x, y, grid) {
  const max = 9;
  const origRowLen = grid[0].length;
  const origColLen = grid.length;

  const gridX = x % origRowLen;
  const gridY = y % origColLen;

  const incrX = Math.floor(x / origRowLen);
  const incrY = Math.floor(y / origColLen);

  const val = (grid[gridY][gridX] + incrX + incrY) % max;
  return val === 0 ? max : val;
}

// assumes a square grid
function embiggen(
  grid,
  populateCell = defaultPopulateCell,
  hCopies = 5,
  vCopies = 5
) {
  return Array(grid.length * vCopies)
    .fill(0)
    .map((_, y) =>
      Array(grid[0].length * hCopies)
        .fill(0)
        .map((_, x) => populateCell(x, y, grid))
    );
}

function print(grid) {
  console.log(grid.map((row) => row.join("")).join("\n"));
}
