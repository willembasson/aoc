v1 = []
v2 = []
with open("input.txt") as file:
    for line in file:
        numbers = line.strip().split()
        v1.append(int(numbers[0]))
        v2.append(int(numbers[-1]))
v1.sort()
v2.sort()
sum = 0
for num1, num2 in zip(v1, v2):
    sum += abs(num1 - num2)

print(sum)
