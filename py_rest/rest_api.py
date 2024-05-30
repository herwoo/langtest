#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import requests

my_url = 'https://api.beta.swaggystocks.com/wsb/sentiment/rating?timeframe=12+hours'
my_json = requests.get(my_url)
my_data = my_json.json()

for ticker in my_data[:10]:
    print(ticker['ticker'])
