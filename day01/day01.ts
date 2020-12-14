const decoder = new TextDecoder("utf-8");
const input = await Deno.readFile("./input.txt");
const lines = decoder.decode(input).trim().split("\n");
const numbers = lines.map((x) => parseInt(x, 10));

const getPart1 = (resolve: (value: [number, number]) => void) => {
  let result: [number, number];
  for (const number1 of numbers) {
    for (const number2 of numbers) {
      const sum = number1 + number2;
      if (sum === 2020) {
        const product = number1 * number2;
        result = [sum, product];
        resolve(result);
      }
    }
  }
};

const getPart2 = (resolve: (value: [number, number]) => void) => {
  let result: [number, number];
  for (const number1 of numbers) {
    for (const number2 of numbers) {
      for (const number3 of numbers) {
        const sum = number1 + number2 + number3;
        if (sum === 2020) {
          const product = number1 * number2 * number3;
          result = [sum, product];
          resolve(result);
        }
      }
    }
  }
};

// part 1
try {
  const sumAndProduct = await new Promise(((resolve) => getPart1(resolve)));
  console.log(sumAndProduct);
} catch (e) {
  console.error(e);
}

// part 2
try {
  const sumAndProduct = await new Promise(((resolve) => getPart2(resolve)));
  console.log(sumAndProduct);
} catch (e) {
  console.error(e);
}
