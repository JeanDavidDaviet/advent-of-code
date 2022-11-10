import { getLinesFromFile } from "../modules/input.ts";
const lines = await getLinesFromFile('./input.txt');

const cols = lines[0].length;
const rows = lines.length;

type Position = Slope | Square;

interface Slope {
  x: number,
  y: number
}

interface Square {
  x: number;
  y: number;
}

enum TileType {
  OPEN = '.',
  TREE = '#'
}

const runThrough = (slope: Position) : number => {
  let row = 1;
  let trees = 0;
  const position : Position = { x: 0, y: 0}

  while(row <= rows - 1){
    position.x = (position.x + slope.x) % cols;
    position.y += slope.y;
    
    if(lines[position.y] !== undefined && lines[position.y][position.x] === TileType.TREE){
      trees++;
    }
    row++;
  }

  return trees;
}

const ex1 = runThrough({x: 1, y: 1});
const ex2 = runThrough({x: 3, y: 1});
const ex3 = runThrough({x: 5, y: 1});
const ex4 = runThrough({x: 7, y: 1});
const ex5 = runThrough({x: 1, y: 2});

console.log(ex1 * ex2 * ex3 * ex4 * ex5);
