const sizes = [100, 1_000, 10_000, 100_000];

const uniq1 = (arr) =>
  Object.keys(arr.reduce((o, v) => ({ ...o, [v]: 1 }), {}));

const uniq2 = (arr) =>
  Object.keys(
    arr.reduce((o, v) => {
      o[v] = 1;
      return o;
    }, {})
  );

const uniq3 = (arr) => [...new Set(arr)];

for (const size of sizes) {
  const list = [];
  let start, end;

  for (let i = 0; i < size; i++) {
    list.push(`key-${i}`);
  }

  start = Date.now();
  uniq1(list);
  end = Date.now();

  console.log(`uniq1, items: ${size}, time: ${(end - start) / 1000}s`);

  start = Date.now();
  uniq2(list);
  end = Date.now();

  console.log(`uniq2, items: ${size}, time: ${(end - start) / 1000}s`);

  start = Date.now();
  uniq3(list);
  end = Date.now();

  console.log(`uniq3, items: ${size}, time: ${(end - start) / 1000}s`);
}
