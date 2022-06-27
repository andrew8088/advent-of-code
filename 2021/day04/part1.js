/*
 * DATA STRUCTURES
 *
 * I don't want want to loop through every completed row possibility on every
 * time board every I "call" a number. To avoid that, I'm using:
 *
 * NumberBoardMap: a map from the number to an array of spaces on the boards.
 * The index in the array represents the board number, the value represents
 * the number's position on that board.
 *
 * Board: function that takes and stores a position and determines whether it
 * has completed a row.
 *
 */

const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
});

let INPUT = [];

rl.on("line", (line) => {
  INPUT.push(line);
});

rl.on("close", () => {
  const { numbers, boards, numberBoardMap } = parseInput(INPUT);

  for (let i = 0; i < numbers.length; i++) {
    const num = numbers[i];
    const locations = numberBoardMap[num];

    for (let i = 0; i < locations.length; i++) {
      if (locations[i]) {
        const [row, col] = locations[i];
        boards[i].sumOfRemaining -= num;
        boards[i].completedPerRow[row]++;
        boards[i].completedPerCol[col]++;
        if (
          boards[i].completedPerRow[row] === 5 ||
          boards[i].completedPerCol[col] === 5
        ) {
          console.log(boards[i].sumOfRemaining * num);
          return;
        }
      }
    }
  }
});

function parseInput(input) {
  const numbers = toNums(input.shift());
  const numberBoardMap = {};

  let boardIndex = -1; // start with -1 b/c there's an empty row at the beginning.
  let rowIndex = 0;
  const boards = [];

  for (const line of input) {
    if (!line.trim()) {
      // push in the next board
      boards.push({
        sumOfRemaining: 0,
        completedPerRow: [0, 0, 0, 0, 0],
        completedPerCol: [0, 0, 0, 0, 0],
      });

      // reset for next board
      currBoardSum = 0;
      boardIndex++;
      rowIndex = 0;
      continue;
    }

    // process board rows
    toNums(line).forEach((val, colIndex) => {
      if (!numberBoardMap[val]) numberBoardMap[val] = [];
      numberBoardMap[val][boardIndex] = [rowIndex, colIndex];
      boards[boardIndex].sumOfRemaining += val;
    });

    rowIndex++;
  }

  return {
    numbers,
    boards,
    numberBoardMap,
  };
}

function toNums(line) {
  return line
    .trim()
    .replace(/ +/g, ",")
    .split(",")
    .map((n) => parseInt(n, 10));
}
