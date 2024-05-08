# Multilayer Perceptron (MLP)

This model consists of an input layer, one hidden layer, and an output layer:

- The input layer has a number of neurons equal to the number of features in the input data (X.shape[1]), and uses the ReLU (Rectified Linear Unit) activation function.
- The hidden layer has 64 neurons and also uses the ReLU activation function.
- The output layer has one neuron, as this is a binary classification problem, and uses the sigmoid activation function to output a probability that the input belongs to one class or the other.

## Config

To run, first install necessary dependencies:
```py
pip install -r requirements.txt
```
and then start the script:
```py
python mlffnn.py
```

---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](mailto:carlobortolan@gmail.com)
