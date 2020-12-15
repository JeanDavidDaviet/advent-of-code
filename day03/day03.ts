import { getLinesFromFile } from "../modules/input.ts";
const lines = await getLinesFromFile('./input.txt');

const cols = lines[0].length;
const rows = lines.length;
let trees = 0;
let row = 1;

interface Position {
  x: number;
  y: number;
}

const position : Position = { x: 0, y: 0 }

enum TileType {
  OPEN = 'open',
  TREE = 'tree'
}

while(row <= rows - 1){
  position.x = (position.x + 3) % cols;
  position.y += 1;
  
  if(lines[position.y][position.x] === '#'){
    trees++;
  }
  console.log(row, position.y, lines[position.y][position.x]);
  row++;
}

console.log(trees);

