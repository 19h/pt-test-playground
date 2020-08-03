use std::sync::Arc;

use colored::*;

use pt::apis::{
    ForexCurrenciesApi,
    ForexCurrenciesApiClient,
    ReferenceApi,
    ReferenceApiClient,
    StocksEquitiesApi,
    StocksEquitiesApiClient,
    CryptoApi,
    CryptoApiClient,
};

#[tokio::main]
async fn main() {
    let https = hyper_tls::HttpsConnector::new();
    let hyper_client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let config =
        pt::apis::configuration::Configuration {
            api_key: Some(pt::apis::configuration::ApiKey {
                key: "F??????_??????????????_".to_string(),
                prefix: None,
            }),
            ..pt::apis::configuration::Configuration::new(
                hyper_client,
            )
        };

    let wrap_config =
        Arc::new(
            config,
        );

    {
        let ref_client =
            ReferenceApiClient::new(
                wrap_config.clone(),
            );

        println!("{}", "ref#v1_meta_symbols_symbol_company_get".green());
        ref_client.v1_meta_symbols_symbol_company_get("AAPL").await.unwrap();

        println!("{}", "ref#v1_meta_symbols_symbol_news_get".green());
        ref_client.v1_meta_symbols_symbol_news_get("AAPL", 10, 0).await.unwrap();

        println!("{}", "ref#v1_marketstatus_now_get".green());
        ref_client.v1_marketstatus_now_get().await.unwrap();

        println!("{}", "ref#v1_marketstatus_upcoming_get".green());
        ref_client.v1_marketstatus_upcoming_get().await.unwrap();

        println!("{}", "ref#v2_reference_dividends_symbol_get".green());
        ref_client.v2_reference_dividends_symbol_get("AAPL").await.unwrap();

        println!("{}", "ref#v2_reference_financials_symbol_get".green());
        ref_client.v2_reference_financials_symbol_get(
            "AAPL",
            5,
            "Y",
            "calendarDate",
        ).await.unwrap();

        println!("{}", "ref#v2_reference_locales_get".green());
        ref_client.v2_reference_locales_get().await.unwrap();

        println!("{}", "ref#v2_reference_markets_get".green());
        ref_client.v2_reference_markets_get().await.unwrap();

        println!("{}", "ref#v2_reference_splits_symbol_get".green());
        ref_client.v2_reference_splits_symbol_get("AAPL").await.unwrap();

        println!("{}", "ref#v2_reference_tickers_get".green());
        ref_client.v2_reference_tickers_get(
            "ticker",
            "",
            "STOCKS",
            "us",
            "microsoft",
            50,
            1,
            true,
        ).await.unwrap();

        println!("{}", "ref#v2_reference_types_get".green());
        ref_client.v2_reference_types_get().await.unwrap();
    }

    {
        let stock_client =
            StocksEquitiesApiClient::new(
                wrap_config.clone(),
            );

        println!("{}", "stocks#v1_last_quote_stocks_symbol_get".green());
        stock_client.v1_last_quote_stocks_symbol_get("AAPL").await.unwrap();

        println!("{}", "stocks#v1_last_stocks_symbol_get".green());
        stock_client.v1_last_stocks_symbol_get("AAPL").await.unwrap();

        println!("{}", "stocks#v1_meta_conditions_ticktype_get".green());
        stock_client.v1_meta_conditions_ticktype_get("trades").await.unwrap();

        println!("{}", "stocks#v1_meta_exchanges_get".green());
        stock_client.v1_meta_exchanges_get().await.unwrap();

        println!("{}", "stocks#v1_open_close_symbol_date_get".green());
        stock_client.v1_open_close_symbol_date_get(
            "AAPL",
            "2020-06-03".into(),
        ).await.unwrap();

        println!("{}", "stocks#v2_aggs_grouped_locale_locale_market_market_date_get".green());
        stock_client.v2_aggs_grouped_locale_locale_market_market_date_get(
            "US",
            "STOCKS",
            "2020-06-03",
            false,
        ).await.unwrap();

        println!("{}", "stocks#v2_aggs_ticker_ticker_prev_get".green());
        stock_client.v2_aggs_ticker_ticker_prev_get(
            "AAPL",
            false,
        ).await.unwrap();

        println!("{}", "stocks#v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get".green());
        stock_client.v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get(
            "AAPL",
            1,
            "day",
            "2019-01-01",
            "2019-02-01",
            false,
            "asc".into(),
        ).await.unwrap();

        println!("{}", "stocks#v2_snapshot_locale_us_markets_stocks_direction_get".green());
        stock_client.v2_snapshot_locale_us_markets_stocks_direction_get("gainers").await.unwrap();
        //stock_client.v2_snapshot_locale_us_markets_stocks_direction_get("loosers").await.unwrap();

        println!("{}", "stocks#v2_snapshot_locale_us_markets_stocks_tickers_get".green());
        stock_client.v2_snapshot_locale_us_markets_stocks_tickers_get().await.unwrap();

        println!("{}", "stocks#v2_snapshot_locale_us_markets_stocks_tickers_ticker_get".green());
        //stock_client.v2_snapshot_locale_us_markets_stocks_tickers_ticker_get("AAPL").await.unwrap();

        println!("{}", "stocks#v2_ticks_stocks_nbbo_ticker_date_get".green());
        stock_client.v2_ticks_stocks_nbbo_ticker_date_get(
            "AAPL",
            "2018-02-02".into(),
            1517618978479449000,
            1517619600072498000,
            false,
            10,
        ).await.unwrap();

        println!("{}", "stocks#v2_ticks_stocks_trades_ticker_date_get".green());
        stock_client.v2_ticks_stocks_trades_ticker_date_get(
            "AAPL",
            "2018-02-02".into(),
            1517618978479449000,
            1517619600072498000,
            false,
            10,
        ).await.unwrap();
    }

    {
        let forex_client =
            ForexCurrenciesApiClient::new(
                wrap_config.clone(),
            );

        println!("{}", "forex#v1_conversion_from_to_get".green());
        forex_client.v1_conversion_from_to_get(
            "USD",
            "EUR",
            100,
            4,
        ).await.unwrap();

        println!("{}", "forex#v1_historic_forex_from_to_date_get".green());
        forex_client.v1_historic_forex_from_to_date_get(
            "USD",
            "EUR",
            "2020-2-2".into(),
            1517529600225, // optional, but can't be left out here
            100,
        ).await.unwrap();

        println!("{}", "forex#v1_last_quote_currencies_from_to_get".green());
        forex_client.v1_last_quote_currencies_from_to_get(
            "USD",
            "EUR",
        ).await.unwrap();

        println!("{}", "forex#v2_aggs_grouped_locale_locale_market_market_date_get".green());
        forex_client.v2_aggs_grouped_locale_locale_market_market_date_get(
            "US",
            "STOCKS",
            "2020-07-20",
            true,
        ).await.unwrap();

        println!("{}", "forex#v2_aggs_ticker_ticker_prev_get".green());
        forex_client.v2_aggs_ticker_ticker_prev_get("AAPL", false).await.unwrap();

        println!("{}", "forex#v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get".green());
        forex_client.v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get(
            "AAPL",
            1,
            "day",
            "2019-01-01",
            "2019-02-01",
            false,
            "asc".into(),
        ).await.unwrap();

        println!("{}", "forex#v2_snapshot_locale_global_markets_forex_direction_get".green());
        forex_client.v2_snapshot_locale_global_markets_forex_direction_get(
            "gainers",
        ).await.unwrap();

        println!("{}", "forex#v2_snapshot_locale_global_markets_forex_tickers_get".green());
        forex_client.v2_snapshot_locale_global_markets_forex_tickers_get().await.unwrap();
    }

    {
        let crypto_client =
            CryptoApiClient::new(
                wrap_config.clone(),
            );

        println!("{}", "crypto#v1_historic_crypto_from_to_date_get".green());
        crypto_client.v1_historic_crypto_from_to_date_get(
            "BTC",
            "USD",
            "2018-5-9".into(),
            1525824000000,
            100,
        ).await.unwrap();

        println!("{}", "crypto#v1_last_crypto_from_to_get".green());
        crypto_client.v1_last_crypto_from_to_get(
            "BTC",
            "USD",
        ).await.unwrap();

        println!("{}", "crypto#v1_meta_crypto_exchanges_get".green());
        crypto_client.v1_meta_crypto_exchanges_get().await.unwrap();

        println!("{}", "crypto#v1_open_close_crypto_from_to_date_get".green());
        crypto_client.v1_open_close_crypto_from_to_date_get(
            "BTC",
            "USD",
            "2018-7-5".into(),
        ).await.unwrap();

        println!("{}", "crypto#v2_aggs_grouped_locale_locale_market_market_date_get".green());
        crypto_client.v2_aggs_grouped_locale_locale_market_market_date_get(
            "US",
            "CRYPTO",
            "2019-02-01",
            true,
        ).await.unwrap();

        println!("{}", "crypto#v2_aggs_ticker_ticker_prev_get".green());
        crypto_client.v2_aggs_ticker_ticker_prev_get(
            "BTC",
            false,
        ).await.unwrap();

        println!("{}", "crypto#v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get".green());
        crypto_client.v2_aggs_ticker_ticker_range_multiplier_timespan_from_to_get(
            "X:BTCUSD",
            1,
            "day",
            "2020-05-01",
            "2020-06-01",
            false,
            "asc".into(),
        ).await.unwrap();

        println!("{}", "crypto#v2_snapshot_locale_global_markets_crypto_direction_get".green());
        crypto_client.v2_snapshot_locale_global_markets_crypto_direction_get(
            "gainers",
        ).await.unwrap();

        println!("{}", "crypto#v2_snapshot_locale_global_markets_crypto_tickers_get".green());
        crypto_client.v2_snapshot_locale_global_markets_crypto_tickers_get().await.unwrap();

        println!("{}", "crypto#v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get".green());
        crypto_client.v2_snapshot_locale_global_markets_crypto_tickers_ticker_book_get("X:BTCUSD").await.unwrap();

        println!("{}", "crypto#v2_snapshot_locale_global_markets_crypto_tickers_ticker_get".green());
        crypto_client.v2_snapshot_locale_global_markets_crypto_tickers_ticker_get("X:BTCUSD").await.unwrap();
    }
}
