max = 0

def tool(choice):
  match choice:
    case "Rock":
      return 1
    case "Paper":
      return 2
    case "Scissor":
      return 3
  return 0

def choice(hand):
  match hand:
    case 'X': # loose
      return 0
    case 'Y': # draw
      return 3
    case 'Z': # win
      return 6
  return 0

def fight(point, char):
    print (point, char)
    if (point == 0 and char == 'A') or (point == 3 and char == 'C') or (point == 6 and char == 'B'):
        return tool("Scissor")
    elif (point == 0 and char == 'B') or (point == 3 and char == 'A') or (point == 6 and char == 'C'):
        return tool("Rock")
    elif (point == 0 and char == 'C') or (point == 3 and char == 'B') or (point == 6 and char == 'A'):
        return tool("Paper")
    else:
       return 0


with open("./input.txt") as f:
  lines = f.readlines()

for line in lines:
  chars = line.rstrip().split(' ')
  point = choice(chars[1])
  result = fight(point, chars[0])
  print (chars, result + point)
  max = max + result + point

print(max)