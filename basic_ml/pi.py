#!/usr/bin/env python3

import numpy as np


#inputs can be features from a heat-detection centers
inputs = [[1, 2, 3, 2.5],
          [2.0, 5.8, -1.0, 2.0],
          [-1.5, 2.7, 3.3, -0.8]] 

weights = [[0.2, 0.8, -0.5, 1.0],
           [0.5, -0.91, 0.26, -0.5],
           [-0.26, -0.27, 01.7, 0.87]]

biases = [2, 3, 0.5]

weights2 = [[0.1, -0.14, 0.5],
           [0.5, 0.12, -0.33],
           [0.44, 0.73, -0.13]]

biases2 = [1, 2, -0.5]




layer1_outputs = np.dot(inputs, np.array(weights).T) + biases
layer2_outputs = np.dot(layer1_outputs, np.array(weights2).T) + biases2

print(layer2_outputs)

#u1 = np.dot(weights, np.array(inputs).T) + biases
#print(u1)


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
