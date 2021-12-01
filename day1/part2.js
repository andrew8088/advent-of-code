process.stdin.resume();
process.stdin.setEncoding("utf8");

let measurements = [];

process.stdin.on("data", (data) => {
  measurements = measurements.concat(
    data.split("\n").map((line) => parseInt(line.trim(), 10))
  );
});

process.stdin.on("end", () => {
  let count = 0;
  let [prev1, prev2, prev3, ...rest] = measurements;
  const sum = () => prev1 + prev2 + prev3;

  let prevSum = sum();

  for (const next of rest) {
    prev1 = prev2;
    prev2 = prev3;
    prev3 = next;

    if (sum() > prevSum) count++;
    prevSum = sum();
  }

  console.log(count);
});
