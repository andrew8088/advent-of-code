const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

const pathMap = {};

const addToMap = (a, b) => {
  if (!pathMap[a]) pathMap[a] = [];
  pathMap[a].push(b);
};

const maybeAddToVisited = (arr, ...nodes) => {
  const newArr = [...arr];

  for (const node of nodes) {
    if (node.toLowerCase() === node) {
      newArr.push(node);
    }
  }
  return newArr;
};

const getSmallCaves = (map) =>
  Object.keys(map).filter(
    (a) => a.toLowerCase() === a && a !== "start" && a !== "end"
  );

rl.on("line", (line) => {
  const [a, b] = line.split("-");
  addToMap(a, b);
  addToMap(b, a);
});

const FINAL_PATHS = {};

rl.on("close", () => {
  const smallCaves = getSmallCaves(pathMap);

  for (const smallCave of smallCaves) {
    walk(FINAL_PATHS, pathMap, smallCave);
  }

  console.log(Object.keys(FINAL_PATHS).length);
});

function walk(
  aggregator,
  map,
  smallCaveTwiceVisitable,
  smallCaveTwiceVisitableCount = 0,
  node = "start",
  path = [],
  alreadyVisited = []
) {
  if (!map[node]) return;
  if (node === "end") {
    const p = path.concat([node]).join(",");
    aggregator[p] = 1;
    return;
  }

  const nextNodes = map[node].filter((x) => !alreadyVisited.includes(x));

  if (node == smallCaveTwiceVisitable) {
    smallCaveTwiceVisitableCount++;
    if (smallCaveTwiceVisitableCount >= 2) {
      alreadyVisited = maybeAddToVisited(alreadyVisited, node);
    }
  } else {
    alreadyVisited = maybeAddToVisited(alreadyVisited, node);
  }

  for (const nextNode of nextNodes) {
    walk(
      aggregator,
      map,
      smallCaveTwiceVisitable,
      smallCaveTwiceVisitableCount,
      nextNode,
      path.concat([node]),
      alreadyVisited
    );
  }
}
