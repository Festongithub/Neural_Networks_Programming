import numpy as np
np.random.seed(0)
import nnfs
from nnfs.datasets import spiral_data, vertical_data

nnfs.init()


class Layer_Dense:
    def __init__(self, n_inputs, n_neurons):

        """
        initialize weights as smaller random values
        between -1 to 1
        biases are zero
        """
        self.weights = 0.10 * np.random.randn(n_inputs, n_neurons)
        self.biases = np.zeros((1, n_neurons))

        
    def forward(self, inputs):
        self.output = np.dot(inputs, self.weights) + self.biases
        return self.output

class Activation_ReLU:
    def forward(self, inputs):
        self.output = np.maximum(0, inputs)

class Activation_Softmax:
    def forward(self, inputs):
        exp_values = np.exp(inputs - np.max(inputs, axis=1, keepdims=True))
        probabilities = exp_values / np.sum(exp_values, axis=1, keepdims=True)
        self.output = probabilities
class Loss:
    def calculate(self, output, y):
        sample_losses = self.forward(output, y)
        data_loss = np.mean(sample_losses)

        return data_loss

class Loss_CategoricalCrossentropy(Loss):
    def forward(self, y_pred, y_true):
        samples = len(y_pred)
        y_pred_clipped = np.clip(y_pred, 1e-7, 1 - 1e-7)

        if len(y_true.shape) == 1:
            correct_confidences = y_pred_clipped[range(samples), y_true]

        elif len(y_true.shape) == 2:
            correct_confidences = np.sum(y_pred_clipped * y_true, axis=1)

        negative_log_likelihoods = np.log(correct_confidences)

        return negative_log_likelihoods
    
if __name__ =='__main__':
    X, y = spiral_data(samples=100, classes=3)
    dense1 = Layer_Dense(2, 3)
    activation1 = Activation_ReLU()

    dense2 = Layer_Dense(3, 3)
    activation2 = Activation_Softmax()

    dense1.forward(X)
    activation1.forward(dense1.output)

    dense2.forward(activation1.output)
    activation2.forward(dense2.output)

    print(activation2.output[:5])

    loss_function = Loss_CategoricalCrossentropy()
    loss = loss_function.calculate(activation2.output, y)

    print("Loss: ", loss)



    A, b = vertical_data(samples=100, classes=3)

    denseA = Layer_Dense(2, 3)
    activationA = Activation_ReLU()

    denseB = Layer_Dense(3, 3)
    activationB = Activation_Softmax()

    loss_function = Loss_CategoricalCrossentropy()


    lowest_loss = 9999999
    best_denseA_weights = denseA.weights.copy()
    best_denseA_biases = denseA.biases.copy()

    best_denseB_weights = denseB.weights.copy()
    best_denseB_biases = denseB.biases.copy()


    for iteration in range(100000):
        denseA.weights = 0.05 * np.random.randn(2, 3)
        denseA.biases = 0.05 * np.random.randn(1, 3)

        denseB.weights = 0.05 * np.random.randn(3,3)
        denseB.biases = 0.05 * np.random.randn(1, 3)

        denseA.forward(A)
        activationA.forward(denseA.output)

        denseB.forward(activationA.output)
        activationB.forward(denseB.output)


        loss = loss_function.calculate(activationB.output, b)

        predictions = np.argmax(activationB.output, axis=1)
        accuracy = np.mean(predictions==y)


        if loss < lowest_loss:
            print("New set of weights found, iteration: ", iteration, "loss: ", loss, "acc: ", accuracy)
            best_denseA_weights = denseA.weights.copy()
            best_denseA_biases = denseA.biases.copy()
            best_denseB_weights = denseB.weights.copy()
            best_denseB_biases = denseB.biases.copy()

            lowest_loss = loss


