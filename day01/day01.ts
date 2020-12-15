const decoder = new TextDecoder("utf-8");
const input = await Deno.readFile("./input.txt");
const lines = decoder.decode(input).trim().split("\n");
const numbers = lines.map((x) => parseInt(x, 10));

type ResolveArg = (value: [number, number]) => void;
type RejectArg = (reason: Error) => void;

enum Error {
  NOT_FOUND = 'no item have a 2020 sum'
}

const getPart1 = (resolve: ResolveArg, reject: RejectArg) => {
  let result: [number, number];
  for (const number1 of numbers) {
    for (const number2 of numbers) {
      const sum = number1 + number2;
      if (sum === 2020) {
        console.log(number1, number2);
        
        const product = number1 * number2;
        result = [sum, product];
        resolve(result);
      }
    }
  }
  reject(Error.NOT_FOUND);
};

const getPart2 = (resolve: ResolveArg, reject: RejectArg) => {
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
  reject(Error.NOT_FOUND);
};

// part 1
try {
  const sumAndProduct = await new Promise((resolve: ResolveArg, reject: RejectArg) => getPart1(resolve, reject));
  console.log(sumAndProduct);
} catch (e) {
  console.error(e);
}

// part 2
try {
  const sumAndProduct = await new Promise((resolve: ResolveArg, reject: RejectArg) => getPart2(resolve, reject));
  console.log(sumAndProduct);
} catch (e) {
  console.error(e);
}

export {}
