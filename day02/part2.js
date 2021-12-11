const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let a = 0;
let x = 0;
let y = 0;

rl.on("line", (line) => {
  let [command, value] = line.split(" ");
  value = parseInt(value, 10);

  switch (command) {
    case "forward":
      x += value;
      y += a * value;
      return;
    case "up":
      a -= value;
      return;
    case "down":
      a += value;
      return;
  }
});

rl.on("close", () => console.log(x * y));
