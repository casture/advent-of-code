const START = 50;
const MAX = 100;

async function main() {
  const path = `../input/${Bun.argv[2] ?? ""}.txt`;
  const text = await Bun.file(path).text();
  let sum = 0,
    cur = START;
  for (const line of text.split("\n")) {
    const dir = line[0] === "R" ? 1 : -1;
    let step = +line.substring(1);

    // part 1
    // const modStep = step % MAX;
    // cur = (cur + dir * modStep) % MAX;
    // if (cur < 0) cur = MAX + cur;
    // if (cur === 0 && step % MAX !== 0) sum++;

    // part 2
    for (let i = 1; i <= step; i++) {
      cur = (cur + dir + 100) % MAX;
      if (cur === 0) sum++;
    }
  }
  console.log(sum);
}

await main();
