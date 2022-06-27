const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let x = 0;
let y = 0;

rl.on("line", (line) => {
  let [command, value] = line.split(" ");
  value = parseInt(value, 10);

  switch (command) {
    case "forward":
      x += value;
      return;
    case "up":
      y -= value;
      return;
    case "down":
      y += value;
      return;
  }
});

rl.on("close", () => console.log(x * y));
