const v1 = [];
const v2 = [];
const input = await Deno.readTextFile(
  new URL(import.meta.resolve("./input.txt")),
).then((lines) =>
  lines.split("\n").forEach((line) => {
    if (line) {
      let [i1, i2] = line.split(/\s+/);
      v1.push(Number(i1));
      v2.push(Number(i2));
    }
  }),
);
v1.sort();
v2.sort();
let sum = 0;
for (let i = 0; i < v1.length; i++) {
  sum += Math.abs(v1[i] - v2[i]);
}
console.log(sum);
