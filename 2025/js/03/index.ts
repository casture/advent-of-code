const NEEDED_BATTERIES = 12;

async function main() {
  const path = `../input/${Bun.argv[2] ?? ""}.txt`;
  const text = await Bun.file(path).text();
  var sum = 0;
  for (const line of text.split("\n")) {
    sum += +part2(line);
  }
  console.log(sum);
}

function part1(line: string): string {
  const batteries = line.split("");
  let l = "0",
    li = 0;
  for (let i = 0; i < batteries.length - 1; i++) {
    const cur = batteries[i] ?? "0";
    if (cur > l) {
      l = cur;
      li = i;
    }
  }

  let r = "0";
  for (let i = li + 1; i < batteries.length; i++) {
    const cur = batteries[i] ?? "0";
    if (cur > r) {
      r = cur;
    }
  }
  return l + r;
}

function part2(line: string): string {
  const batteries = line.split("");

  function findMaxIndex(low: number, high: number): number {
    let maxI = low;
    for (let i = low; i <= high; i++) {
      if (batteries[maxI]! < batteries[i]!) {
        maxI = i;
      }
    }
    return maxI;
  }

  let lastMaxI = -1;
  return Array(NEEDED_BATTERIES)
    .fill(0)
    .map((i) => {
      const nI = findMaxIndex(
        lastMaxI + 1,
        batteries.length - NEEDED_BATTERIES + i,
      );
      return batteries[nI];
    })
    .join("");

  // const best = [];
  // let lastMaxI = -1;
  // for (let i = 0; i < NEEDED_BATTERIES; i++) {
  //   lastMaxI = findMaxIndex(
  //     lastMaxI + 1,
  //     batteries.length - NEEDED_BATTERIES + i,
  //   );
  //   best.push(lastMaxI);
  // }
  // return best.map((i) => batteries[i]).join("");
}

await main();
