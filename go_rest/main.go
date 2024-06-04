package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

// Struct to represent the data model
type StockData struct {
	Ticker string `json:"ticker"`
	//SentimentRating     int     `json:"sentiment_rating"`
	// Timestamp           int64   `json:"timestamp"`
	// Positive            string  `json:"positive"`
	// Neutral             string  `json:"neutral"`
	// Negative            string  `json:"negative"`
	// Total               string  `json:"total"`
	// NextEarningsDate    string  `json:"next_earnings_date"`
	// MarketCap           int64   `json:"market_cap"`
	// OptionsOICallRatio  string  `json:"options_oi_call_ratio"`
	// ThirtyDayAvgIV      float64 `json:"30_day_avg_iv"`
	// UnusualOptionVolume string  `json:"unusual_option_volume"`
}

func main() {
	// URL of the REST API
	url := "https://api.beta.swaggystocks.com/wsb/sentiment/rating?timeframe=12+hours"

	// Send HTTP GET request
	resp, err := http.Get(url)
	if err != nil {
		log.Fatal("Error fetching data:", err)
	}
	defer resp.Body.Close()

	// Decode JSON response
	var data []StockData
	if err := json.NewDecoder(resp.Body).Decode(&data); err != nil {
		log.Fatal("Error decoding JSON:", err)
	}

	// Print the parsed data
	for _, item := range data[:10] {
		fmt.Println(item.Ticker)
	}
}
