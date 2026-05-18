import numpy as np

inputs = np.array(
    [[4.8, 1.21, 2.385],
     [8.9, -1.81, 0.2],
     [1.41, 1.051, 0.026]]
)

for i in inputs:
    print(sum(i))



print('Sum without axis')
print(np.sum(inputs))

print(np.sum(inputs, axis=0))