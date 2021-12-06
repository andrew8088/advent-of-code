const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let fish = [];

rl.on("line", (line) => {
  fish = fish.concat(line.split(",").map((x) => parseInt(x, 10)));
});

rl.on("close", () => {
  for (let i = 1; i <= 80; i++) {
    fish = tick(fish);
  }
  console.log(fish.length);
});

function tick(fish) {
  for (let i = 0, len = fish.length; i < len; i++) {
    fish[i]--;

    if (fish[i] === -1) {
      fish[i] = 6;
      fish.push(8);
    }
  }

  return fish;
}
