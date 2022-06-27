const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

const SCORE_MAP = {
  ")": 1,
  "]": 2,
  "}": 3,
  ">": 4,
};

const CLOSE_TO_OPEN = {
  ")": "(",
  "]": "[",
  "}": "{",
  ">": "<",
};

const OPEN_TO_CLOSE = {
  "(": ")",
  "[": "]",
  "{": "}",
  "<": ">",
};

const lines = [];

rl.on("line", (line) => lines.push(line.split("")));

rl.on("close", () => {
  let scores = [];
  let lineScore = 0;

  for (const line of lines) {
    const maybeIncompleteChars = maybeFindIncompleteChars(line);

    if (maybeIncompleteChars) {
      lineScore = 0;
      let next;
      while ((next = maybeIncompleteChars.pop())) {
        const charScore = SCORE_MAP[OPEN_TO_CLOSE[next]];
        lineScore = lineScore * 5 + charScore;
      }
      scores.push(lineScore);
    }
  }

  scores.sort((a, b) => a - b);

  console.log(scores[(scores.length - 1) / 2]);
});

function maybeFindIncompleteChars(line) {
  const stack = [];
  for (const char of line) {
    if (OPEN_TO_CLOSE[char]) {
      stack.push(char);
    } else if (CLOSE_TO_OPEN[char]) {
      if (stack.pop() !== CLOSE_TO_OPEN[char]) {
        return null; // illegal line
      }
    } else {
      throw new Error("unreachable");
    }
  }

  return stack;
}
