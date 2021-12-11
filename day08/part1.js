const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let INPUT = [];

rl.on("line", (line) => INPUT.push(line.split("|")[1].trim().split(" ")));

rl.on("close", () => {
  const is1or4or7or8 = (item) => item.length <= 4 || item.length === 7;

  const count = INPUT.reduce(
    (total, values) => total + values.filter(is1or4or7or8).length,
    0
  );
  console.log(count);
});
