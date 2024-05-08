import os
from dotenv import load_dotenv
import pymongo
import requests
import numpy as np
from sklearn.model_selection import train_test_split
from tensorflow.keras import layers, models
from PIL import Image
import io
import requests
from sklearn.preprocessing import LabelEncoder
from imblearn.over_sampling import RandomOverSampler
from sklearn.metrics import accuracy_score, precision_score, recall_score, f1_score
import tensorflow as tf

# Load .env file
load_dotenv()
# Retrieve res from MongoDB
client = pymongo.MongoClient(os.getenv("MONGO_URL"))
db = client["spider_web"]
collection = db["scraped_data"]
data = list(collection.find())
# Extract features
image_urls = [entry["image_url"] for entry in data]

def preprocess_data(data):
    X = []
    y = []
    label_encoder = LabelEncoder()

    for entry in data:
        try:
            # Get image data from URL
            response = requests.get(entry["image_url"])
            response.raise_for_status()  # Raise exception if the request failed

            # Check if the image is in a format that PIL supports
            if response.headers['Content-Type'] not in ['image/jpeg', 'image/png']:
                print(f"Unsupported image format: {entry['image_url']}")
                continue
        except (requests.exceptions.RequestException, IOError):
            print(f"Failed to fetch image: {entry['image_url']}")
            continue

        print(f"Fetched image: {entry['image_url']}")
        image = Image.open(io.BytesIO(response.content))

        # Resize image
        image = image.resize((64, 64))

        image_data = np.array(image)

        # Flatten image data
        image_data_flattened = image_data.flatten()

        # Append image data to features
        X.append(image_data_flattened)

        # Append source to labels
        y.append(entry["source"])

    # Convert lists to numpy arrays
    X = np.array(X)
    y = np.array(y)

    # Encode labels
    y = label_encoder.fit_transform(y)

    # Balance data
    ros = RandomOverSampler(random_state=0)
    X_resampled, y_resampled = ros.fit_resample(X, y)

    return X_resampled, y_resampled

# Preprocess data (e.g., convert image URLs to image data)
# Note: Implement preprocessing logic based on your specific requirements
# For the sake of this example, let's assume that we have a function `preprocess_data` that does this
X, y = preprocess_data(data)

# NN architecture
model = models.Sequential([
    layers.Dense(64, activation='relu', input_shape=(X.shape[1],)),  # Specify input shape
    layers.Dense(64, activation='relu'),
    layers.Dense(1, activation='sigmoid')  # Output layer for binary classification
])

model.compile(optimizer='adam',
              loss='binary_crossentropy',  # Binary cross-entropy loss for binary classification
              metrics=['accuracy'])

# Train-test split
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Train the model
model.fit(X_train, y_train, epochs=5)

# Make predictions on new data
predictions = model.predict(X_test)

# Convert predictions to binary values
predictions = (predictions > 0.5).astype(int)

# Evaluate model
accuracy = accuracy_score(y_test, predictions)
precision = precision_score(y_test, predictions)
recall = recall_score(y_test, predictions)
f1 = f1_score(y_test, predictions)

print(f"Accuracy: {accuracy}")
print(f"Precision: {precision}")
print(f"Recall: {recall}")
print(f"F1 Score: {f1}")

# Save model
model.save('model.h5')

# Select images based on predictions
selected_images = [image_url for image_url, prediction in zip(image_urls, predictions) if prediction == 1]

# Post selected images to API endpoint
api_endpoint = "https://api.b1o.co/player"
for i in range(0, len(selected_images), 10):  # Batch API requests
    batch = selected_images[i:i+10]
    for image_url in batch:
        payload = {
            "name": "image_name",
            "image_url": image_url,
            "source": "manual"
        }
        try:
            response = requests.post(api_endpoint, json=payload)
            response.raise_for_status()  # Raise exception if the request failed
            print(f"Image posted successfully: {image_url}")
        except (requests.exceptions.RequestException, IOError):
            print(f"Failed to post image: {image_url}")