import matplotlib.pyplot as plt
import numpy as np
from math import e
def f(x):
    return 2 * x**2


p2_delta = 0.0001

x1 = 1
x2 = x1 + p2_delta

y1 = f(x1)
y2 = f(x2)

approximate_d = (y2 - y1) / (x2 - x1)
print(approximate_d)


x = np.arange(0, 50, 0.001)
Y = f(x)

plt.plot(x, Y)
plt.show()
