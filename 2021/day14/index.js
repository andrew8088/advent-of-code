const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let TEMPLATE = {};
let LAST_CHAR = "";
const RULES = {};

rl.on("line", (line) => {
  const [a, b] = line.split(" -> ");
  if (b) {
    RULES[a] = b;
  } else if (a) {
    LAST_CHAR = a[a.length - 1];
    TEMPLATE = toPairObj(a);
  }
});

rl.on("close", () => {
  console.log(getPolymer(TEMPLATE, RULES, 10)); // part 1
  console.log(getPolymer(TEMPLATE, RULES, 40)); // part 2
});

function getPolymer(template, rules, stepCount) {
  template = { ...template };

  let i = 0;
  while (i++ < stepCount) {
    template = step(template, rules);
  }

  return score(template, LAST_CHAR);
}

function step(template, rules) {
  const newTemplate = {};
  const keys = Object.keys(template);
  for (const key of keys) {
    if (rules[key] && template[key] > 0) {
      const count = template[key];
      const k1 = key[0] + rules[key];
      const k2 = rules[key] + key[1];
      incr(newTemplate, k1, count);
      incr(newTemplate, k2, count);
    }
  }
  return newTemplate;
}

function incr(obj, key, val = 1) {
  if (!obj[key]) obj[key] = 0;
  obj[key] += val;
}

/*
 * each pair is two letters, but if we count only the first letter in each pair,
 * then we'll have the right count for ever character in the string ... except
 * we'll be missing the very last character in the string. That never changes,
 * so we capture that when parsing and "manually" add it here.
 *
 */
function score(template, lastChar) {
  const charToCount = {};

  for (const key of Object.keys(template)) {
    incr(charToCount, key[0], template[key]);
  }

  charToCount[lastChar]++;

  const tuples = Object.entries(charToCount).sort((a, b) => a[1] - b[1]);

  return tuples[tuples.length - 1][1] - tuples[0][1];
}

function toPairObj(str) {
  const o = {};
  for (let i = 0; i < str.length - 1; i++) {
    const pair = str.slice(i, i + 2);
    incr(o, pair);
  }
  return o;
}
