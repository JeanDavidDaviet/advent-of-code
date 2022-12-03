accumulator = {}
counter = 0
max = 0

with open("./input.txt") as f:
  lines = f.readlines()

for line in lines:
  if not (counter in accumulator):
    accumulator[counter] = []
  
  line = line.rstrip()
  if line == '':
    counter += 1
  else:
    accumulator[counter].append(int(line))

for acc in accumulator:
  accumulator[acc] = sum(accumulator[acc])
  if accumulator[acc] >= max:
    max = accumulator[acc]

print(max)
    