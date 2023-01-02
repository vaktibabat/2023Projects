from sklearn.datasets import fetch_openml
from sklearn.linear_model import SGDClassifier
import matplotlib as mpl
import matplotlib.pyplot as plt
import numpy as np
 
mnist = fetch_openml('mnist_784', version=1)

X, y = mnist["data"], mnist["target"]

y = y.astype(np.uint8)

X_train, X_test, y_train, y_test = X[:60000], X[60000:], y[:60000], y[60000:]

def show_image():
    some_digit = X[1]
    some_digit_image = some_digit.reshape(28, 28)
    plt.imshow(some_digit_image, cmap = mpl.cm.binary, interpolation="nearest")
    plt.axis("off")
    plt.show()

def binary_classifier():
    y_train_7 = (y_train == 7)
    y_test_7 = (y_test == 7)

    sgd_clsf = SGDClassifier(random_state=42)
    sgd_clsf.fit(X_train, y_train_7)

def main():
    sgd_clsf = SGDClassifier(random_state=42)
    sgd_clsf.fit(X_train, y_train)

    print(sgd_clsf.predict([X[1]]))

main()
show_image()
    
