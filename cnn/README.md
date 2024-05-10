# Classifier

`api_classifier.py`, is a classifier that uses the pre-trained ResNet50 model from TensorFlow to classify images from Pexels. It fetches images from specific URLs, preprocesses them, and uses the model to predict their labels. The script is designed to handle a large number of images by using concurrent futures for parallel processing. It also handles exceptions for failed requests or image classification. Once an image is classified, the script posts the image URL, its label (converted from snake_case to Title Case), and the source to the Rust API. The script also includes a filter to skip posting images with certain labels.

## Config

To run, first install necessary dependencies:
```py
pip install -r requirements.txt
```
and then start the script:
```py
python api_classifier.py
```

# Convolutional neural network (CNN)

> ![NOTE]
> The CNN is not implemented yet

The model consists of an input layer, one hidden layer, and an output layer:

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
