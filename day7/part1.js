const { test, input } = require("./input");

function run(input) {
  const crabs = input
    .split(",")
    .map(Number)
    .sort((a, b) => a - b);

  const med = Math.ceil(median(crabs));
  const fuel = crabs.reduce((fuel, crab) => fuel + Math.abs(crab - med), 0);

  console.log(fuel);
}

// expects sorted input
function median(nums) {
  if (nums.length % 2 === 0) {
    const left = nums[nums.length / 2 - 1];
    const right = nums[nums.length / 2];

    return (left + right) / 2;
  } else {
    return nums[(nums.length + 1) / 2];
  }
}

run(input);
