import argparse
import json
import os

import numpy as np
import pandas as pd
import tensorflow as tf
from sklearn.model_selection import train_test_split
from tensorflow.keras.layers import (
    Conv2D,
    Dense,
    Dropout,
    GlobalAveragePooling2D,
    MaxPooling2D,
)
from tensorflow.keras.models import Sequential
from tensorflow.keras.utils import to_categorical

data_dir = "../data/"
class_mapping_file = os.path.join(data_dir, "classes.csv")
saved_data_file = os.path.join(data_dir, "data.npz")
model_file = os.path.join(data_dir, "model.h5")


def load_file(file_path):
    with open(file_path, "r") as f:
        data = json.load(f)
        size = data["config"]["size"]
        steps = data["config"]["steps"]
        return np.array(data["rows"]).astype("float32").reshape(1, size, steps, 1)


# Function to predict the class of a single sample
def predict_class(model, sample):
    # sample = np.array(sample).reshape(1, 16, 16, 1)  # Reshape for CNN input
    prediction = model.predict(sample)
    return int(np.argmax(prediction)) + 1


def load_data(data_dir, class_mapping_file, saved_data_file):
    X, y = [], []

    # Load class mappings from CSV
    class_mapping = pd.read_csv(class_mapping_file, index_col=0).to_dict()["class"]
    print(class_mapping)

    # Iterate through JSON files
    for rule_folder in os.listdir(data_dir):
        rule_path = os.path.join(data_dir, rule_folder)
        if os.path.isdir(rule_path):
            print(f"Processing: {rule_path}")
            for file in os.listdir(rule_path)[:100]:
                if file.endswith(".json"):
                    with open(os.path.join(rule_path, file), "r") as f:
                        data = json.load(f)
                    rule = int(data["config"]["rule"].split(" ")[1])
                    rule_class = class_mapping[rule]
                    X.append(np.array(data["rows"]))
                    y.append(rule_class)
    X = np.array(X).astype("float32")
    y = np.array(y) - 1

    # Normalize X (optional, since values are binary 0/1)
    X = X / 1.0

    # Convert labels to categorical (one-hot encoding)
    y = to_categorical(y, num_classes=4)

    # Split into train and test sets
    X_train, X_test, y_train, y_test = train_test_split(
        X, y, test_size=0.2, random_state=42
    )

    # Save preprocessed data to file
    np.savez_compressed(
        saved_data_file, X_train=X_train, X_test=X_test, y_train=y_train, y_test=y_test
    )
    print(f"Data saved to {saved_data_file}")

    return X_train, X_test, y_train, y_test


# The CNN model will:
# Take a 16×16 binary image as input.
# Apply convolutional layers to detect patterns in cellular automata evolution.
# Use pooling layers to reduce dimensionality.
# Include dense layers at the end for classification.
def build_model():
    model = Sequential(
        [
            # First convolutional
            # 32 filters (small pattern detectors).
            # Each filter is 3x3 in size.
            # ReLU activation (removes negative values to introduce non-linearity).
            # Padding='same' (ensures output size is same as input).
            Conv2D(
                32,
                (3, 3),
                activation="relu",
                padding="same",
                input_shape=(None, None, 1),
            ),
            # Pooling layer
            # Reduces image size by 2x2.
            # Helps remove noise and reduces computation.
            MaxPooling2D((2, 2)),
            # Second convolutional
            # Similar to the first convolution, but now with 64 filters.
            # More filters capture more complex patterns.
            Conv2D(64, (3, 3), activation="relu", padding="same"),
            # Pooling layer
            MaxPooling2D((2, 2)),
            # Flatten: Converts the 2D feature maps into a 1D vector
            # Flatten(),
            # GlobalAveragePooling2D() replaces Flatten(), ensuring the model works with variable input sizes.
            # It averages feature maps, keeping a fixed-size representation regardless of input dimensions.
            GlobalAveragePooling2D(),
            # Fully connected layer with 128 neurons.
            Dense(128, activation="relu"),
            # Randomly disables 50% of neurons during training to prevent overfitting.
            Dropout(0.5),
            # Output layer with 4 neurons (one for each Wolfram class)
            # Softmax activation ensures outputs sum to 1 (interpreted as probabilities).
            Dense(4, activation="softmax"),
        ]
    )

    model.compile(
        optimizer="adam",
        loss="categorical_crossentropy",
        metrics=["accuracy"],
    )

    return model


def train_model(saved_data_file, model_file):
    # Load preprocessed dataset
    data = np.load(saved_data_file)
    X_train, X_test, y_train, y_test = (
        data["X_train"],
        data["X_test"],
        data["y_train"],
        data["y_test"],
    )

    # Reshape input data to match CNN expected input shape (16, 16, 1)
    # X_train = X_train.reshape(-1, 16, 16, 1)
    # X_test = X_test.reshape(-1, 16, 16, 1)

    # Build the CNN model
    model = build_model()

    # Train the model
    model.fit(
        X_train, y_train, epochs=10, batch_size=32, validation_data=(X_test, y_test)
    )

    # Save the trained model
    model.save(model_file)
    print(f"Model saved to {model_file}")

    return model


if __name__ == "__main__":
    # Parse command-line arguments
    parser = argparse.ArgumentParser(description="Predict the class of a cellular automaton.")
    # add --reload flag to reload test data
    parser.add_argument("--reload", action="store_true", help="Reload test data.")
    # add --train flag to train the model
    parser.add_argument("--train", action="store_true", help="Train the model.")
    # add the filename
    parser.add_argument('filename')
    # parse the arguments
    args = parser.parse_args()

    if args.reload or not os.path.exists(saved_data_file):
        load_data(data_dir, class_mapping_file, saved_data_file)
    if args.train or not os.path.exists(model_file):
        train_model(saved_data_file, model_file)
    if args.filename:
        model = tf.keras.models.load_model(model_file)
        data = load_file(args.filename)
        print(predict_class(model, data))
