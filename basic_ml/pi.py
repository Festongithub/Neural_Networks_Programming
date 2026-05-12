#!/usr/bin/env python3

import numpy as np



inputs = [1, 2, 3, 2.5]

weights = [[0.2, 0.8, -0.5, 1.0],
           [0.5, -0.91, 0.26, -0.5],
           [-0.26, -0.27, 01.7, 0.87]]

biases = [2, 3, 0.5]

output = np.dot(weights, inputs) + biases
print(output)


some_value = 0.5
weight = -0.7
bias = 0.7

print(some_value*weight)
print(some_value+bias)


a = [1, 2, 3]
b = [4, 5, 6]
b1 = 3
c = np.dot(b, a) + b1
print(c)
