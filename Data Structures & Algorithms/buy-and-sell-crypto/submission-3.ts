class Solution {
    /**
     * @param {number[]} prices
     * @return {number}
     */
    maxProfit(prices: number[]): number {
        let profit = 0  

        // prices is an unsorted array of prices, for this array, 
        // we need to find where the best price to buy and sell is
        // we need to find out for each price
        // where the highest higher price is after it and put that
        // 
        let maximum_profit = 0

        // loop over prices, for each price, check prices after the original price index for higher, price,
        // if price is higher, we can say we have a profit, then if that is higher than max profit, set it to max profit

       prices.forEach((price, index) => {
        // check all prices after this price
        prices.forEach((in_price, in_index) => {
            if (in_index < index) return 
            if (prices[in_index] > price && in_price - price > maximum_profit) {
                console.log({price: price, in_price})
                maximum_profit = in_price - price
            }   
        })
       })

    return maximum_profit
    }
}
