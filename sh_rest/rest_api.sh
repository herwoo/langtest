#!/bin/bash

curl -s "https://api.beta.swaggystocks.com/wsb/sentiment/rating?timeframe=12+hours" | jq -r '.[] | .ticker' | head -n 10

