const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let INPUT = [];

rl.on("line", (line) =>
  INPUT.push(line.split("|").map((part) => part.trim().split(" ")))
);

rl.on("close", () => {
  const result = INPUT.map(([patterns, output]) => {
    const patternMap = mapPatterns(patterns);

    return output
      .map(sortChars)
      .reduce((acc, item) => acc + patternMap[item], "");
  }).reduce((acc, next) => acc + parseInt(next, 10), 0);

  console.log(result);
});

function mapPatterns(patterns) {
  // sort words by letter, and then the list by word length
  patterns = patterns.map(sortChars).sort((a, b) => a.length - b.length);

  const patternMap = {};

  // 1, 4, 7, 8 have unique lengths
  const [one, seven, four, eight] = patterns.filter(len(2, 3, 4, 7));
  patternMap[one] = 1;
  patternMap[four] = 4;
  patternMap[seven] = 7;
  patternMap[eight] = 8;

  let segmentC;

  const lenFives = patterns.filter(len(5));
  const lenSixes = patterns.filter(len(6));

  // lenSixes has the 0, 6, and 9
  for (let lenSix of lenSixes) {
    if (!hasChars(lenSix, seven)) {
      // 6 is the only six-char number that is not a super-set of 7
      patternMap[lenSix] = 6;
      // the only segment not in 6 is c
      segmentC = eight.replace(new RegExp(`[${lenSix}]`, "g"), "");
    } else if (hasChars(lenSix, four)) {
      // 9 is the only six-char number that is a super-set 4
      patternMap[lenSix] = 9;
    } else {
      // 0 is the only six-char number left!
      patternMap[lenSix] = 0;
    }
  }

  // lenFives has the 2, 3, and 5
  for (let lenFive of lenFives) {
    if (hasChars(lenFive, seven)) {
      // 3 is the only five-char number that is a super-set of 7
      patternMap[lenFive] = 3;
    } else if (hasChars(lenFive, segmentC)) {
      // 5 is the only five-char number that has segmentC (after 3 has been eliminated)
      patternMap[lenFive] = 2;
    } else {
      // 5 is the only five-char number left!
      patternMap[lenFive] = 5;
    }
  }

  return patternMap;
}

// returns a predicate that asserts ONE of a set of lengths
function len(...lens) {
  return (item) => {
    for (const l of lens) {
      if (item.length === l) return true;
    }
  };
}

// returns a predicate that asserts ALL of a character subset
function hasChars(str, chars) {
  return !!str.match(new RegExp(chars.split("").join(".*")));
}

function sortChars(str) {
  return str.split("").sort().join("");
}
