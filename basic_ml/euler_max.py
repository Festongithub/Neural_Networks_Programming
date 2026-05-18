#!/usr/bin/env python3

numbers_output = [4.8, 1.21, 2.385]

e = 2.71828182846

numbers = []

for i in numbers_output:
    numbers.append(i * e)
print(numbers)

# print sum of numbers

norm_base  = sum(numbers)
norm_values = []

for v in numbers:
    norm_values.append(v / norm_base)
print(norm_values)
print("sum is: {}".format(sum(norm_values)))


# Test 2

n = []

for i in list(range(10)):
    n.append(e * i)
    print(n)

n_exp = sum(n)
n_values = []

for j in n:
    n_values.append(j/ n_exp)
    print(n_values)