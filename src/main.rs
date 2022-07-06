/********************************************************************/
/* Load Libraries                                                   */
/********************************************************************/
use binance::api::*;
use binance::account::*;
use mysql::*;
use mysql::prelude::*;


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let api_key = Some("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".into());
    let secret_key = Some("YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY".into());
    let url = "mysql://root:example@localhost:3306/test";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let mut buy_balance: bool = false;
    let mut sale_balance: bool = false;
    let symbol = String::from("BTCUSDT");
    let src = String::from("BTC");
    let to = String::from("USDT");
    //let min_amt_usdt = 15.00;
    let min_amt_usdt = 1.00;
    let min_amt_btc = 0.0001;
    let mut current_price = 0.00;

    /********************************************************************/
    /* Connect to the exchange                                          */
    /********************************************************************/
    let account: Account = Binance::new(api_key, secret_key);

    /********************************************************************/
    /* Check if there is balance to buy                                 */
    /********************************************************************/
    match account.get_balance(to) {
        Ok(answer) => {
                        println!("{:?}", answer);
                        let balance = answer.free;
                        let balance = balance.parse::<f32>().unwrap();
                        println!("{}", balance);
                        if balance >= min_amt_usdt {
                            buy_balance = true;
                            println!("There is balance to buy");
                        }
                    }
        Err(e) => println!("Error: {:?}", e),
    }

    if buy_balance {
        /********************************************************************/
        /* Query prices or other data from the DB                           */
        /* Define the conditions to launch a purchase                       */
        /* Usually based on technical indicators like RSI, MACD, SAR, etc   */
        /********************************************************************/
        let val: Option<f64> = conn.exec_first("SELECT price FROM prices WHERE symbol = ? ORDER BY id DESC LIMIT 1", (&symbol,))?;
        current_price = match val {
            Some(value) => value,
            None => 0.00,
        };
        println!("price: {}", current_price);
        let buy_conditions: bool = true;
        //Variable parameters based on conditions
        let amt_buy = 0.001;
        let price_buy = current_price * 0.99;

        /********************************************************************/
        /* Launch a purchase                       */
        /********************************************************************/
        if buy_conditions {
            println!("Launched purchase");
            match account.limit_buy(&symbol, amt_buy, price_buy) {
                Ok(answer) => println!("{:?}", answer),
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }

    /********************************************************************/
    /* Check if there is balance to sale                                */
    /********************************************************************/
    match account.get_balance(src) {
        Ok(answer) => {
                        println!("{:?}", answer);
                        let balance = answer.free;
                        let balance = balance.parse::<f32>().unwrap();
                        println!("{}", balance);
                        if balance >= min_amt_btc {
                            sale_balance = true;
                            println!("There is balance to sale");
                        }
                    }
        Err(e) => println!("Error: {:?}", e),
    }

    if sale_balance {
        /********************************************************************/
        /* Query prices or other data from the DB                           */
        /* Define the conditions to launch a sale                           */
        /* Usually based on a pct return over the purchase price            */
        /********************************************************************/
        let sale_conditions: bool = true;
        //Variable parameters based on conditions
        let amt_sale = 0.001;
        let price_sale = current_price * 1.01;

        /********************************************************************/
        /* Launch a sale                           */
        /********************************************************************/
        if sale_conditions {
            println!("Launched sale");
            match account.limit_sell(&symbol, amt_sale, price_sale) {
                Ok(answer) => println!("{:?}", answer),
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }

    /********************************************************************/
    /* Cancel past operations                           */
    /********************************************************************/
    // after a certain period of time you must cancel all pending transactions
    match account.cancel_all_open_orders(&symbol) {
        Ok(answer) => println!("{:?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }

    Ok(())
}
