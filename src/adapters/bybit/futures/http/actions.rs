use chrono::{DateTime, Utc, NaiveDateTime, Local};
use reqwest::Method;
use serde_json::Value;
use std::collections::HashMap;

use super::client::ByBitHttpClient;

pub struct ByBitFuturesApi {
    client: ByBitHttpClient,
}

impl ByBitFuturesApi {
    pub fn new(
        base_url: &str,
        api_key: &str,
        api_secret: &str,
    ) -> Self {
        let client = ByBitHttpClient::new(base_url, api_key, api_secret);
        Self { client: client }
    }

    pub async fn get_account_overview(&self) -> Option<String> {
        // let my_currency = String::from(currency.unwrap_or("USDT"));

        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(String::from("accountType"), Value::from("UNIFIED"));

        let response = self
            .client
            .send(Method::GET, "/v5/account/wallet-balance", true,&mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        // println!("账户信息11111111111111111111{:?}", res_data);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }


    pub async fn position(&self, category: &str) -> Option<String> {
      let mut params: HashMap<String, Value> = HashMap::new();
      params.insert(String::from("category"), Value::from(category));
      params.insert(String::from("settleCoin"), Value::from("USDT"));

      let response = self
          .client
          .send(Method::GET, "/v5/position/list", true, &mut params)
          .await;

      let res_data = self.client.check_response_data(response);

      match res_data {
          Some(data) => {
              return Some(data);
          }
          None => {
              return None;
          }
      }
  }
  pub async fn get_klines(&self, symbol: &str) -> Option<String> {
      let mut params: HashMap<String, Value> = HashMap::new();
      params.insert(String::from("symbol"), Value::from(symbol));

      // let now_time = Utc::now().timestamp_millis();
      // params.insert(String::from("interval"), Value::from("15m"));
      

      let response = self
          .client
          .send(Method::GET, "/fapi/v1/ticker/price", true, &mut params)
          .await;

      let res_data = self.client.check_response_data(response);

      match res_data {
          Some(data) => {
              // print!("K线数据{}", data);
              return Some(data);
          }
          None => {
              return None;
          }
      }
  }



    pub async fn get_bybit_open_orders(&self, category: &str) -> Option<String> {
        // let my_currency = String::from(currency.unwrap_or("USDT"));

        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(
            String::from("category"),
            Value::from(category),
        );
        params.insert(
            String::from("settleCoin"),
             Value::from("USDT")
        );

        let response = self
            .client
            .send(Method::GET, "/v5/order/realtime", true,&mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        // println!("挂单数据{:?}", res_data);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }


    pub async fn get_bybit_usdc_open_orders(&self) -> Option<String> {
        // let my_currency = String::from(currency.unwrap_or("USDT"));

        let mut params: HashMap<String, Value> = HashMap::new();
        params.insert(
            String::from("category"),
            Value::from("spot"),
        );
        params.insert(
            String::from("settleCoin"),
             Value::from("USDC")
        );

        let response = self
            .client
            .send(Method::GET, "/v5/order/realtime", true,&mut params)
            .await;

        let res_data = self.client.check_response_data(response);

        // println!("挂单数据{:?}", res_data);

        match res_data {
            Some(data) => {
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

}
