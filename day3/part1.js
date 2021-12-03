const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

const count = [];

rl.on("line", (line) => {
  const chars = line.split("");

  for (let i = 0; i < chars.length; i++) {
    if (!count[i]) count[i] = 0;

    count[i] += chars[i] === "1" ? 1 : -1;
  }
});

rl.on("close", () => {
  const g = parseInt(count.map((n) => (n > 0 ? "1" : "0")).join(""), 2);
  const e = parseInt(count.map((n) => (n < 0 ? "1" : "0")).join(""), 2);

  console.log(g * e);
});
