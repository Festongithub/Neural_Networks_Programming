#!/usr/bin/env python3
import numpy as np


E = 2.71828182846

exp_values = []

for i in list(range(10)):
    exp_values.append(E * i)

print(exp_values, end=',')

norm_base = sum(exp_values)
norm_values = []

for j in exp_values:
    norm_values.append(j / norm_base)

print(norm_values)
print(sum(norm_values))

layer_outputs = [4.8, 1.21, 2.385]

e_values = np.exp(layer_outputs)
print(e_values)

n_values = e_values / np.sum(e_values)
print(n_values)
print(np.sum(n_values))
