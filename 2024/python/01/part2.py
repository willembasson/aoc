v1 = []
v2 = []
with open("input.txt") as file:
    for line in file:
        numbers = line.strip().split()
        v1.append(int(numbers[0]))
        v2.append(int(numbers[-1]))
sum = 0
for i in v1:
    sum += i * v2.count(i)

print(sum)
