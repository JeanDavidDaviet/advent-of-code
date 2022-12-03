max = 0

def choice(hand):
  match hand:
    case 'X':
      return 1
    case 'Y':
      return 2
    case 'Z':
      return 3
  return 0

def fight(one, two):
  if (one == 'A' and two == 'X') or (one == 'B' and two == 'Y') or (one == 'C' and two == 'Z'):
    return 3
  elif (one == 'A' and two == 'Y') or (one == 'B' and two == 'Z') or (one == 'C' and two == 'X'):
    return 6
  elif (one == 'A' and two == 'Z') or (one == 'B' and two == 'X') or (one == 'C' and two == 'Y'):
    return 0


with open("./input.txt") as f:
  lines = f.readlines()

for line in lines:
  chars = line.rstrip().split(' ')
  point = choice(chars[1])
  result = fight(chars[0], chars[1])
  print (chars, point, result)
  max = max + result + point

print(max)