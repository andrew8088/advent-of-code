const { test, input } = require("./input");

function run(input) {
  const crabs = input
    .split(",")
    .map(Number)
    .sort((a, b) => a - b);

  const totals = [];

  for (let i = crabs[0]; i <= crabs[crabs.length - 1]; i++) {
    totals.push(
      crabs.reduce((fuel, crab) => fuel + triangleNumber(Math.abs(crab - i)), 0)
    );
  }

  totals.sort((a, b) => a - b);

  console.log(totals[0]);
}

// https://en.wikipedia.org/wiki/Triangular_number
function triangleNumber(n) {
  return (n * (n + 1)) / 2;
}

run(input);
