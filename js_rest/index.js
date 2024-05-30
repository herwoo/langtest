const axios = require('axios');

const apiUrl = 'https://api.beta.swaggystocks.com/wsb/sentiment/rating?timeframe=12+hours'

async function fetchData() {
    try {
        const response = await axios.get(apiUrl);
        return response.data; // Return the parsed JSON data
    } catch (error) {
        console.error('Error fetching data:', error);
        return null;
    }
}

async function main() {
    const data = await fetchData();
    if (data) {
        const myKey = 'ticker';
        const tickers = data.map(obj => obj[myKey]);
        const tickers10 = tickers.slice(0,10);
        for (let i in tickers10) {
            console.log(tickers10[i]);
        }
    } else {
        console.log('No data received');
    }
}

main();
