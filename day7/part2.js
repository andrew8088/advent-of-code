const { test, input } = require("./input");

function run(input) {
  const crabs = input
    .split(",")
    .map(Number)
    .sort((a, b) => a - b);

  const mn = Math.floor(mean(crabs));
  const fuel = crabs.reduce(
    (fuel, crab) => fuel + triangleNumber(Math.abs(crab - mn)),
    0
  );

  console.log(fuel);
}

function mean(nums) {
  return nums.reduce((acc, next) => acc + next, 0) / nums.length;
}

// https://en.wikipedia.org/wiki/Triangular_number
function triangleNumber(n) {
  return (n * (n + 1)) / 2;
}

run(input);
