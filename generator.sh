#!/bin/bash
# Script to manually post 200 samples

for i in {1..200}
do
  curl -X POST "http://localhost:8080/player" \
    -H "Content-Type: application/json" \
    -d "{
      \"name\": \"Winston Smith\",
      \"image_url\": \"https://picsum.photos/id/$i/400/600\",
      \"source\": \"manual\"
    }"
done