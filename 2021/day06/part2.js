const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});
// days:    0, 1, 2, 3, 4, 5, 6, 7, 8
let fish = [0, 0, 0, 0, 0, 0, 0, 0, 0];

rl.on("line", (line) => {
  line.split(",").forEach((x) => {
    fish[x]++;
  });
});

rl.on("close", () => {
  for (let i = 1; i <= 256; i++) {
    fish = tick(fish);
  }

  console.log(fish.reduce((acc, next) => acc + next));
});

function tick(fish) {
  const [zero, ...rest] = fish;

  rest[6] += zero; // reset fish to day 6
  rest.push(zero); // create new fish

  return rest;
}
