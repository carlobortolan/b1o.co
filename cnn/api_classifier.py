from tensorflow.keras.applications.resnet50 import ResNet50, preprocess_input, decode_predictions
from tensorflow.keras.preprocessing import image as keras_image
import os
from dotenv import load_dotenv
import pymongo
import requests
import numpy as np
from PIL import Image
import io
from concurrent.futures import ThreadPoolExecutor
import requests
from concurrent.futures import ThreadPoolExecutor, as_completed

print("Starting classifier script...")

# Load .env file
load_dotenv()

image_urls = []
for i in range(150945, 160945, 20):
    url = f"https://images.pexels.com/photos/{i}/pexels-photo-{i}.jpeg"
    image_urls.append(url)
print(f"Found {len(image_urls)} images")

# Load pre-trained ResNet50 model
print("Loading pre-trained ResNet50 model...")
model = ResNet50(weights='imagenet')
print("Model loaded successfully")

session = requests.Session()

def classify_image(image_url):
    if image_url.startswith('data:'):
        return [(None, None, None)]

    try:
        response = session.get(image_url, timeout=10)
        img_data = response.content
    except (requests.exceptions.Timeout, requests.exceptions.RequestException) as e:
        print(f"Request to {image_url} failed: {e}, skipping.")
        return [(None, None, None)]

    try:
        # Load image into PIL Image object
        img = Image.open(io.BytesIO(img_data))
        img = img.convert('RGB')
        img = img.resize((224, 224))

        # Convert the image to a numpy array and preprocess it for ResNet50
        x = keras_image.img_to_array(img)
        x = np.expand_dims(x, axis=0)
        x = preprocess_input(x)

        # Use the ResNet50 model to classify the image
        preds = model.predict(x)

        # Decode the predictions into a list of tuples (class, description, probability)
        # (one such list for each sample in the batch)
        decoded_preds = decode_predictions(preds, top=1)[0]

        return decoded_preds
    except IOError:
        print(f"Unable to classify image: {image_url}")
        return [(None, None, None)]

# Enable parallel processing
with ThreadPoolExecutor(max_workers=10) as executor:
    futures = {executor.submit(classify_image, image_url): image_url for image_url in image_urls}
    preds = {}
    for future in as_completed(futures):
        image_url = futures[future]
        try:
            result = future.result(timeout=10)
            preds[image_url] = result
        except Exception as e:
            print(f"Classification took too long for image: {image_url}, skipping. Error: {e}")
            preds[image_url] = [(None, None, None)]

# Post selected images
api_endpoint = "https://api.b1o.co/player"
for image_url, pred in preds.items():
    _, label, _ = pred[0]
    print(f"Label: {label}")
    if label is None or label in ['maillot', 'brasserie', 'brassiere', 'wooden_spoon', 'bikini', 'swimsuit', 'miniskirt', 'sunscreen', 'volleyball', 'iron', 'snorkel', 'stethoscope', 'bathing_cap', 'swimming_trunks', 'maraca', 'socks', 'neck_brace', 'dumbbell', 'pole', 'toilet_seat', 'cradle', 'tub', 'swing', 'jeep', 'ocarina']:
        continue
    payload = {
        "name": label.replace('_', ' ').title(),
        "image_url": image_url,
        "source": "images.pexels.com"
    }
    try:
        response = requests.post(api_endpoint, json=payload)
        response.raise_for_status()
        print(f"Image posted successfully: {image_url}")
    except (requests.exceptions.RequestException, IOError):
        print(f"Failed to post image: {image_url}")