const { getTestInput2, getTestInput, getInput } = require("./input");

function part1(input) {
  const [scanner0, ...otherScanners] = parse(input);

  for (let i = 0; i < otherScanners.length; i++) {
    const scanner = otherScanners[i];
    console.log(translateBeacons(scanner));

    // const offsets = findPossibleOffsets(scanner0, scanner);

    // console.log("matchingScanner", scanner0);
    // console.log("originalScanner", scanner);
    // for (const offset of offsets) {
    //   const offsetScanner = offsetBeacons(offset, scanner);
    //   const overlap = intersection(scanner0, offsetScanner);

    //   if (overlap.length) {
    //     console.log(
    //       `scanner0 and scanner${i + 1} overlap with ${overlap.length} beacons`
    //     );
    //   }
    // }
  }
}

part1(getTestInput2());

function translateBeacons(scanner) {
  return scanner.map((beacon) => getTranslations().map((fn) => fn(beacon)));
}

function findPossibleOffsets(scannerA, scannerB) {
  const offsets = {};

  for (const beacon1 of scannerA) {
    for (let beacon2 of scannerB) {
      for (let fn of getTranslations()) {
        beacon2 = fn(beacon2);
        const deltaX = beacon1[0] - beacon2[0];
        const deltaY = beacon1[1] - beacon2[1];
        const deltaZ = beacon1[2] - beacon2[2];
        const delta = [deltaX, deltaY, deltaZ];

        offsets[delta.toString(0)] = delta;
      }
    }
  }

  return Object.values(offsets);
}

function offsetBeacons(offset, beacons) {
  const [offsetX, offsetY, offsetZ] = offset;
  return beacons.map(([x, y, z]) => [offsetX + x, offsetY + y, offsetZ + z]);
}

function intersection(arrA, arrB) {
  arrA = arrA.map((x) => x.join(","));
  arrB = arrB.map((x) => x.join(","));
  return arrA
    .filter((value) => arrB.includes(value))
    .map((x) => x.split(",").map((x) => parseInt(x)));
}

function parse(input) {
  return input
    .trim()
    .split("\n\n")
    .map((x) =>
      x
        .trim()
        .split("\n")
        .map((x) =>
          x.includes("---") ? null : x.split(",").map((x) => parseInt(x, 10))
        )
    )
    .map(([name, ...beacons]) => beacons);
}

function getTranslations() {
  const n = (x) => -1 * x;
  return [
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
}
