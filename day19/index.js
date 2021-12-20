const { getTestInput2, getInput } = require("./input");

const n = (x) => -1 * x;

const translations = [
  // 1, 2, 3
  ([x, y, z]) => [x, y, z],
  // 1, -2, -3
  ([x, y, z]) => [x, n(y), n(z)],
  // 1, 3, -2
  ([x, y, z]) => [x, z, n(y)],
  // 1, -3, 2
  ([x, y, z]) => [x, n(z), y],
  // -1, 2, -3
  ([x, y, z]) => [n(x), y, n(z)],
  // -1, -2, 3
  ([x, y, z]) => [n(x), n(y), z],
  // -1, 3, 2
  ([x, y, z]) => [n(x), z, y],
  // -1, -3, -2
  ([x, y, z]) => [n(x), n(z), n(y)],
  // 2, -1, 3
  ([x, y, z]) => [y, n(x), z],
  // 2, 1, -3
  ([x, y, z]) => [y, x, n(z)],
  // 2, 3, 1,
  ([x, y, z]) => [y, z, x],
  // 2, -3, -1
  ([x, y, z]) => [y, n(z), n(x)],
  // -2, 1, 3
  ([x, y, z]) => [n(y), x, z],
  // -2, -1, -3
  ([x, y, z]) => [n(y), n(x), n(z)],
  // -2, -3, 1
  ([x, y, z]) => [n(y), n(z), x],
  // -2, 3, -1
  ([x, y, z]) => [n(y), z, n(x)],
  // 3, -1, -2
  ([x, y, z]) => [z, n(x), n(y)],
  // 3, 1, 2
  ([x, y, z]) => [z, x, y],
  // 3, -2, 1
  ([x, y, z]) => [z, n(y), x],
  // 3, 2, -1
  ([x, y, z]) => [z, y, n(x)],
  // -3, 1, 2
  ([x, y, z]) => [n(z), x, y],
  // -3, -1, 2
  ([x, y, z]) => [n(z), n(x), y],
  // -3, -2, -1
  ([x, y, z]) => [n(z), n(y), n(x)],
  // -3, 2, 1
  ([x, y, z]) => [n(z), y, x],
];

function part1(input) {
  const [scanner0, scanner1] = parse(input);

  const diffs = [];

  for (const beacon of scanner0.beaconArrs[0]) {
    for (let i = 0; i < scanner1.beaconArrs.length; i++) {
      const beacons = scanner1.beaconArrs[i];
      for (const beacon2 of beacons) {
        diffs.push(compare(beacon, beacon2));
      }
    }
  }

  console.log(diffs.sort((a, b) => a[0] - b[0]));
}

function compare(b1, b2) {
  const [x1, y1, z1] = b1;
  const [x2, y2, z2] = b2;
  return [x1 - x2, y1 - y2, z1 - z2];
}

part1(getTestInput2());

function parse(input) {
  return input
    .trim()
    .split("\n\n")
    .map((x) =>
      x
        .trim()
        .split("\n")
        .map((x) =>
          x.includes("---")
            ? x.split("---")[1].trim()
            : x.split(",").map((x) => parseInt(x, 10))
        )
    )
    .map(([name, ...originalBeacons]) => {
      return {
        name,
        beaconArrs: translations.map((fn) => originalBeacons.map(fn)),
      };
    });
}
