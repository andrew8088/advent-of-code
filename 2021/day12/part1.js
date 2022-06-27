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

rl.on("line", (line) => {
  const [a, b] = line.split("-");
  addToMap(a, b);
  addToMap(b, a);
});

rl.on("close", () => {
  console.log(pathMap);
  console.log(walk(pathMap));
});

function walk(map, node = "start", path = [], alreadyVisited = []) {
  if (node === "end") {
    console.log(path.concat([node]));
    return 1;
  }
  if (!map[node]) return 0;

  let sum = 0;

  const nextNodes = map[node].filter((x) => !alreadyVisited.includes(x));

  for (const nextNode of nextNodes) {
    sum += walk(
      map,
      nextNode,
      path.concat([node]),
      maybeAddToVisited(alreadyVisited, node)
    );
  }

  return sum;
}
