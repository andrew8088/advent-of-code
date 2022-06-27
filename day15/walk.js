const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

const grid = [];

rl.on("line", (line) => grid.push(line.split("").map((x) => parseInt(x, 10))));

rl.on("close", () => {
  part1(grid);
});

function part1(grid) {
  const distances = dijkstra(grid, [0, 0]);
  console.log(distances.pop().pop());
}

function part1Old(grid) {
  const [score, path] = walk(grid);
  console.log(score);

  const board = Array(grid.length)
    .fill(0)
    .map(() => Array(grid.length).fill(" "));
  path.forEach(([x, y]) => (board[y][x] = "X"));
  console.log(board.map((row) => row.join("")).join("\n"));
}

const SCORE_MEMO = {};
const PATH_MEMO = {};

function walk(grid, currCoord = [0, 0]) {
  const [x, y, key] = getVals(currCoord);

  const currScore = isStartState(x, y) ? 0 : grid[y][x];
  if (isEndState(grid, x, y)) return [currScore, [currCoord]];
  if (SCORE_MEMO[key]) return [SCORE_MEMO[key], PATH_MEMO[key]];

  const adjs = findAdjacents(x, y, grid[0].length - 1, grid.length - 1);
  const scores = adjs.map((coords) => walk(grid, coords));
  const [score, path] = scores.sort((a, b) => a[0] - b[0])[0];

  SCORE_MEMO[key] = currScore + score;
  PATH_MEMO[key] = [currCoord, ...path];
  return [SCORE_MEMO[key], PATH_MEMO[key]];
}

const toKey = (coord) => coord.join(",");
const toCoord = (key) => key.split(",").map((x) => parseInt(x, 10));

function dijkstra(grid, start) {
  const distances = [];
  
  for (let i = 0; i < grid.length; i++) {
    distances[i] = [];
    for (let j = 0; j < grid[i].length; j++)  {
      distances[i][j] = Number.MAX_VALUE;
    }
  }
  
  distances[start[1]][start[0]] = 0;
  
  const visited = {};
  
  while (true) {
    let shortestDistance = Number.MAX_VALUE;
    let shortestX = -1;
    let shortestY = -1;
    
    for (let i = 0; i < grid.length; i++) {
      for (let j = 0; j < grid[i].length; j++)  {
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
    
    const adjs = findAdjacents(shortestX, shortestY, grid[0].length - 1, grid.length - 1);
    
    for (const [x, y] of adjs) {
      if (distances[y][x] > distances[shortestY][shortestX] + grid[y][x]) {
        distances[y][x] = distances[shortestY][shortestX] + grid[y][x];
      }
    }
    
    visited[toKey([shortestX, shortestY])] = true;
  }
}

const inBounds = (val, min, max) => val >= min && val <= max;

function findAdjacents(x, y, maxX, maxY) {
  return [
    [x, y - 1], // N
    // [x + 1, y - 1], // NE
    [x + 1, y], // E
    // [x + 1, y + 1], // SE
    [x, y + 1], // S
    // [x - 1, y + 1], // SW
    [x - 1, y], // W
    // [x - 1, y - 1], // NW
  ].filter(([x, y]) => inBounds(x, 0, maxX) && inBounds(y, 0, maxY));
}

function isStartState(x, y) {
  return x === 0 && y === 0;
}

function isEndState(grid, x, y) {
  return y === grid.length - 1 && x === grid[grid.length - 1].length - 1;
}


