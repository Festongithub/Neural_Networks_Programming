#!/usr/bin/env python3


# single neurons
inputs = [1, 2, 3, 2.5]
weights = [0.2, 0.8, -0.5, 0.4]

bias = 2

output = inputs[0] * weights[0] + inputs[1] * weights[1] + inputs[2] * weights[2] + inputs[3] * weights[3]+ bias
print(output)

# single inputs, 4 weights 

inputs = [1, 2, 3, 2.5]
weights1 = [0.2, 0.8, -0.5, 0.4]
weights2 = [0.5, 7.5, 9.3, 2.1]
weights3 = [3.4, 6.7, 8.4, 2.4]


bias1 = 2
bias2 = 3
bias3 = 4


t_output = [
        inputs[0] * weights1[0] + inputs[1] * weights1[1] + inputs[2] * weights1[2] + inputs[3] * weights1[3]+ bias1,
        inputs[0] * weights2[0] + inputs[1] * weights2[1] + inputs[2] * weights2[2] + inputs[3] * weights2[3]+ bias2,
        inputs[0] * weights3[0] + inputs[1] * weights3[1] + inputs[2] * weights3[2] + inputs[3] * weights3[3]+ bias3
        ]


print(t_output)



# matrix product 


m_weights = [[0.2, 0.8, -0.5, 0.4],
             [0.5, 7.5, 9.3, 2.1],
             [3.4, 6.7, 8.4, 2.4]]


