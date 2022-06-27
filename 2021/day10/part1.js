const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

const SCORE_MAP = {
  ")": 3,
  "]": 57,
  "}": 1197,
  ">": 25137,
};

const CLOSE_TO_OPEN = {
  ")": "(",
  "]": "[",
  "}": "{",
  ">": "<",
};

const OPENING_CHARS = Object.values(CLOSE_TO_OPEN);
const CLOSING_CHARS = Object.keys(CLOSE_TO_OPEN);

const lines = [];

rl.on("line", (line) => lines.push(line.split("")));

rl.on("close", () => {
  let score = 0;
  for (const line of lines) {
    const maybeIllegalChar = maybeFindFirstIllegalChar(line);

    if (maybeIllegalChar) {
      score += SCORE_MAP[maybeIllegalChar];
    }
  }

  console.log(score);
});

function maybeFindFirstIllegalChar(line) {
  const stack = [];
  for (const char of line) {
    if (OPENING_CHARS.includes(char)) {
      stack.push(char);
    } else if (CLOSING_CHARS.includes(char)) {
      if (stack.pop() !== CLOSE_TO_OPEN[char]) {
        return char; // illegal char
      }
    } else {
      throw new Error("unreachable");
    }
  }

  return null;
}
