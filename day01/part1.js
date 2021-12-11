process.stdin.resume();
process.stdin.setEncoding("utf8");

let measurements = [];

process.stdin.on("data", (data) => {
  measurements = measurements.concat(
    data.split("\n").map((line) => parseInt(line.trim(), 10))
  );
});

process.stdin.on("end", () => {
  let count = -1;
  let prev = 0;

  for (const m of measurements) {
    if (m > prev) count++;
    prev = m;
  }

  console.log(count);
});
