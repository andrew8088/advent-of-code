const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

// we could walk down the tree at each later to get the counts, but easier to cache them.
const COUNT_IDX = 2;
// at each layer, [array of zero branches, array of one branches, number of branches]
const indices = [];

rl.on("line", (line) => {
  let pointer = indices;

  for (const ch of line.split("")) {
    if (!pointer[ch]) pointer[ch] = [];
    if (!pointer[COUNT_IDX]) pointer[COUNT_IDX] = 0;
    pointer[COUNT_IDX]++;
    pointer = pointer[ch];
  }
});

rl.on("close", () => {
  function walk(tree, takeZero, str) {
    const [zero, one] = tree;
    if (!zero && !one) return str;

    if (takeZero(zero, one)) {
      return walk(zero, takeZero, str + "0");
    } else {
      return walk(one, takeZero, str + "1");
    }
  }

  const oxygen = walk(indices, (z, o) => z && z[COUNT_IDX] > o[COUNT_IDX], "");
  const co2 = walk(
    indices,
    (z, o) => (z && o ? z[COUNT_IDX] <= o[COUNT_IDX] : z),
    ""
  );

  console.log(parseInt(oxygen, 2) * parseInt(co2, 2));
});
