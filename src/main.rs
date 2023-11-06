
// use std::collections::VecDeque;
use std::{collections::HashMap, fs, time::Duration};
use log::{info, warn, error};
use positions_alarm::adapters::binance;
use serde_json::{Map, Value};
// use tokio::{sync::broadcast::{self, Receiver}};
use positions_alarm::adapters::binance::futures::http::actions::BinanceFuturesApi;
use positions_alarm::adapters::bybit::futures::http::actions::ByBitFuturesApi;
use positions_alarm::adapters::binance::papi::http::actions::BinancePapiApi;
// use open_order_alarm::base::ssh::SshClient;
use positions_alarm::base::wxbot::WxbotHttpClient;
use positions_alarm::base::slackbot::SlackHttpClient;
use positions_alarm::actors::*;
// use test_alarm::models::http_data::*;
use positions_alarm::actors::trade_mapper::*;

#[warn(unused_mut, unused_variables, dead_code)]
async fn real_time(
    symbols: &Vec<Value>,
) {
    //rece: &mut Receiver<&str>){
    info!("get ready for real time loop");
    let running = true;
    // let mut day_pnl = 0.0;

    let mut i = 0;
    // let mut end = 6;

    // 每个品种的上一个trade_id
    let mut last_trade_ids: HashMap<String, u64> = HashMap::new();
    for symbol_v in symbols {
        let symbol = String::from(symbol_v.as_str().unwrap());
        let symbol = format!("{}USDT", symbol);
        last_trade_ids.insert(symbol, 0);
    }

    // 交易历史
    // let trade_histories: VecDeque<Value> = VecDeque::new();

    // 净值数据
    // let mut net_worth_histories: VecDeque<Value> = VecDeque::new();

    info!("begin real time loop");
    // 监控循环
    loop {
        info!("again");
        // json对象
        // let mut response: Map<String, Value> = Map::new();
        // let mut json_data: Map<String, Value> = Map::new();
        let mut map: Map<String, Value> = Map::new();
        map.insert(String::from("productId"), Value::from("TRADER_001"));
        // let now = Utc::now();
        // let date = format!("{}", now.format("%Y/%m/%d %H:%M:%S"));

        // 监控服务器状态
        info!("server process");


        
        let acc_alarm = trade_mapper::TradeMapper::get_acc_alarm();

        

        
        if let Ok(acc) = acc_alarm{
        for f_config in acc {

            

            let acc_id = &f_config.acc_id;
            let tra_id = &f_config.tra_id;
            let position_alarm = &f_config.position_alarm;
            let thres_amt = &f_config.position_amount;
            let amount: f64 = thres_amt.parse().unwrap();

            let traders = trade_mapper::TradeMapper::get_positions(tra_id).unwrap();
            
            for trader in traders {
                let tra_name = &trader.name;
                if &trader.tra_venue == "Binance" && &trader.r#type == "Futures" {
                    let binance_futures_api=BinanceFuturesApi::new(
                        "https://fapi.binance.com",
                        &trader.api_key,
                        &trader.secret_key,
                    );
                    let name = tra_name;
    
                    
                            if position_alarm == "true" && amount != 0.0{
                                if let Some(data) = binance_futures_api.account(None).await {
                                    let v: Value = serde_json::from_str(&data).unwrap();
                                    let positions = v.as_object().unwrap().get("positions").unwrap().as_array().unwrap();
                                    let mut amts: f64 = 0.0;
                                    // let mut prices: f64 = 0.0;
                                    for p in positions {
                                        let obj = p.as_object().unwrap();
                                        let position_amt: f64 = obj.get("positionAmt").unwrap().as_str().unwrap().parse().unwrap();
                                        
                                        if position_amt == 0.0 {
                                            continue;
                                        } else {
                                            
                                        let symbol = obj.get("symbol").unwrap().as_str().unwrap();
                                        let symbols= &symbol[0..symbol.len()-4];
                                        // println!("symbols: {},symbol: {}", symbols, symbol);
                                        let sbol = format!("{}USDT", symbols);
                                        // println!("传过去的参数{}", sbol);
                                            if let Some(data) = binance_futures_api.get_klines(&sbol).await {
                                                let v: Value = serde_json::from_str(&data).unwrap();
                                                let price_obj = v.as_object().unwrap();
                            
                                                let price:f64 = price_obj.get("price").unwrap().as_str().unwrap().parse().unwrap();
                                                let new_amt = position_amt * price;
                                                amts += new_amt;
                                                // prices = price;
                                            }
                                        }
                            
                                    }
                                    if amts.abs() > amount {
                                        let wx_notices = trade_mapper::TradeMapper::get_weixin(acc_id).unwrap();
                                        for f_weixin in wx_notices {
                                            let wx_hook = &f_weixin.wx_hook;
                                                    let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
                                                    wxbot.push_str(&wx_hook);
                                                    info!("wxbot  {}", wxbot);
                                                    let wx_robot = WxbotHttpClient::new(&wxbot);
            
                                                    let sender = format!("Binance交易所的----{}普通账号", name);
                                    let content = format!("净头寸高于阈值");
                                    wx_robot.send_text(&sender, &content).await;
            
                            //                     if slack_hook.len() != 0 {
                            //                         let mut slackrobot = String::from("https://hooks.slack.com/services/");
                            // slackrobot.push_str(&slack_hook);
                            // let slack_robot = SlackHttpClient::new(&slackrobot);
            
                            // let sender = format!("Binance交易所的----{}普通账号", name);
                            // let content = format!("净头寸高于阈值");
                            // slack_robot.send_text(&sender, &content).await;
            
                            //                     }
                                        }
                                        println!("高于阈值")
                                    } else {
                                        let wx_notices = trade_mapper::TradeMapper::get_weixin(acc_id).unwrap();
                                        for f_weixin in wx_notices {
                                            let wx_hook = &f_weixin.wx_hook;
                                                    let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
                                                    wxbot.push_str(&wx_hook);
                                                    info!("wxbot  {}", wxbot);
                                                    let wx_robot = WxbotHttpClient::new(&wxbot);
            
                                                    let sender = format!("Binance交易所的----{}普通账号", name);
                                    let content = format!("净头寸低于阈值");
                                    wx_robot.send_text(&sender, &content).await;
                                        
                                    }
                                }
                                    // net_worth = notional_total/ori_fund;
                                    // net_worth_histories.push_back(Value::from(new_account_object));
                                } else {
                                    error!("Can't get bian_futures_positions {} account.", name);
                                    continue;
                                    
                                }
    
    
                            }
                            
    
                }
    
    
                if &trader.tra_venue == "ByBit" {
                    let bybit_futures_api=ByBitFuturesApi::new(
                        "https://api.bybit.com",
                        &trader.api_key,
                        &trader.secret_key,
                    );
                    let name = tra_name;
    
                    if position_alarm == "true" && amount != 0.0  {
                        let mut spot_positions = 0.0;
                        if let Some(data) = bybit_futures_api.get_account_overview().await {
                            let value: Value = serde_json::from_str(&data).unwrap();
                            
                            let assets = value.as_object().unwrap().get("result").unwrap().as_object().unwrap();
                            let list = assets.get("list").unwrap().as_array().unwrap();
    
                            for a in list {
                                let obj = a.as_object().unwrap();
                                let coin = obj.get("coin").unwrap().as_array().unwrap();
                                for c in coin {
                                    let objs = c.as_object().unwrap();
                                    let amt: f64 = objs.get("walletBalance").unwrap().as_str().unwrap().parse().unwrap();
    
                                    if amt == 0.0 {
                                        continue;
                                    } else {
                                        let symbol = objs.get("coin").unwrap().as_str().unwrap();
                                        if symbol != "USDT" && symbol != "USDC" {
                                            let usd_value: f64 = objs.get("usdValue").unwrap().as_str().unwrap().parse().unwrap();
                                            spot_positions += usd_value
                                            
    
                                        }
                                    }
                                }
                            }
                            
                        } else {
                            error!("Can't get bybit_spot_positions {} account.", name);
                        }
    
                        let category_lear = "linear";
                        if let Some(data) = bybit_futures_api.position(category_lear).await {
                            let value: Value = serde_json::from_str(&data).unwrap();
                            let result = value.as_object().unwrap().get("result").unwrap().as_object().unwrap();
                            let positions = result.get("list").unwrap().as_array().unwrap();
                            let mut amts = 0.0;
    
                            for p in positions {
                                let mut pos = 0.0;
                                let obj = p.as_object().unwrap();
                                let position_amt: f64 = obj.get("size").unwrap().as_str().unwrap().parse().unwrap();
                                let side = obj.get("side").unwrap().as_str().unwrap();
                                if side == "Sell"{
                                   pos = spot_positions - position_amt;
                               } else {
                                  pos = position_amt + spot_positions;
                              }
                              let price: f64 = obj.get("markPrice").unwrap().as_str().unwrap().parse().unwrap();
                              let pos_price = pos * price;
                              amts += pos_price; 
                            }
    
                            if amts.abs() > amount {
                                let wx_notices = trade_mapper::TradeMapper::get_weixin(acc_id).unwrap();
                                for f_weixin in &wx_notices {
                                    let wx_hook = &f_weixin.wx_hook;
                                            let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
                                            wxbot.push_str(&wx_hook);
                                            info!("wxbot  {}", wxbot);
                                            let wx_robot = WxbotHttpClient::new(&wxbot);
    
                                            let sender = format!("Bybit交易所的----{}账号", name);
                            let content = format!("净头寸高于阈值");
                            wx_robot.send_text(&sender, &content).await;
    
                    //                     if slack_hook.len() != 0 {
                    //                         let mut slackrobot = String::from("https://hooks.slack.com/services/");
                    // slackrobot.push_str(&slack_hook);
                    // let slack_robot = SlackHttpClient::new(&slackrobot);
    
                    // let sender = format!("ByBit交易所的----{}账号", name);
                    // let content = format!("净头寸高于阈值");
                    // slack_robot.send_text(&sender, &content).await;
    
                    //                     }
                                }
    
                            }
                        } else {
                            error!("Can't get bybit_linear_positions {} account.", name);
                        }
    
    
                        
    
                    }
    
    
                }
                
                
    
                if trader.tra_venue == "Binance" && trader.r#type == "Papi" {
                    let binance_papi_api=BinancePapiApi::new(
                        "https://papi.binance.com",
                        &trader.api_key,
                        &trader.secret_key,
                    );
    
                    let binance_futures_api=BinanceFuturesApi::new(
                        "https://fapi.binance.com",
                        &trader.api_key,
                        &trader.secret_key,
                    );
                    let name = tra_name;
    
                    if position_alarm == "true" && amount != 0.0 {
                        if let Some (data) = binance_papi_api.account().await {
                            let value: Value = serde_json::from_str(&data).unwrap();
                            // println!("value{}", value);
                            let positions = value.as_object().unwrap().get("positions").unwrap().as_array().unwrap();
                            let mut amts = 0.0;
                            for p in positions {
                                let obj = p.as_object().unwrap();
                                let position_amt: f64 = obj.get("positionAmt").unwrap().as_str().unwrap().parse().unwrap();
                                if position_amt == 0.0 {
                                    continue;
                                } else {
                                    // println!("positions{:?}", obj);
                                    
                                let symbol = obj.get("symbol").unwrap().as_str().unwrap();
                                let new_symbol= &symbol[0..symbol.len()-4];
                                let sbol = format!("{}USDT",  new_symbol);
                                if let Some(data) = binance_futures_api.get_klines(&sbol).await{
                                    let v: Value = serde_json::from_str(&data).unwrap();
                                    let price_obj = v.as_object().unwrap();
                                    let price: f64 = price_obj.get("price").unwrap().as_str().unwrap().parse().unwrap();
                                    let new_amt = position_amt * price;
                                    amts += new_amt;
                                }
                                }
                                
                            }
    
                            if amts.abs() > amount {
                                let wx_notices = trade_mapper::TradeMapper::get_weixin(acc_id).unwrap();
                                for f_weixin in wx_notices {
                                    let wx_hook = &f_weixin.wx_hook;
                                            let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
                                            wxbot.push_str(&wx_hook);
                                            info!("wxbot  {}", wxbot);
                                            let wx_robot = WxbotHttpClient::new(&wxbot);
    
                                            let sender = format!("Binance交易所的----{}统一账号", name);
                            let content = format!("净头寸高于阈值");
                            wx_robot.send_text(&sender, &content).await;
    
                    //                     if slack_hook.len() != 0 {
                    //                         let mut slackrobot = String::from("https://hooks.slack.com/services/");
                    // slackrobot.push_str(&slack_hook);
                    // let slack_robot = SlackHttpClient::new(&slackrobot);
    
                    // let sender = format!("Binance交易所的----{}统一账号", name);
                    // let content = format!("净头寸高于阈值");
                    // slack_robot.send_text(&sender, &content).await;
    
                    //                     }
                                }
    
                            }
                        } else {
                            error!("Can't get bian_papi_positions {} account.", name);
                            continue;
                        }
    
                    }
    
                }

                
            }

             
        }
    }
        i += 1;


        

        // 等待下次执行
        info!("waiting for next real time task...({})", 6000 * 10);
        tokio::time::delay_for(Duration::from_millis(6000 * 10)).await;
    }
}

#[warn(unused_mut, unused_variables)]
#[tokio::main]
async fn main() {
    // 日志
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();

    init();
    // let time = format!("{}", Local::now().format("%Y/%m/%d %H:%M:%S"));

    // 测试用api
    // let api_key="JwYo1CffkOLqmv2sC3Qhe2Qu5GgzbeLVw2BxWB5HgK6tnmc8yGfkzLuDImBgDkXm";
    // let api_secret="7FtQARZqM2PDgIZ5plr3nwEVYBXXbvmSuvmpf6Viz9e7Cq2B87grRTG3VZQiEC5C";

    // 连接数据库
    // let config_db: Value =
    //     serde_json::from_str(&fs::read_to_string("./configs/database.json").unwrap()).unwrap();

    // 读取配置
    let config: Value = serde_json::from_str(
        &fs::read_to_string("./configs/total.json").expect("Unable to read file"),
    )
    .expect("Unable to parse");

    // 任务间通信信道
    // let (send, mut rece) = broadcast::channel(32);

    // 创建任务
    let real_time_handle = tokio::spawn(async move {
        // let mut futures_config: Map<String, Value> = Map::new();
        // let mut servers_config = Map::new();
        let binance_config = config.get("Binance").unwrap();
        let binance_future_config = binance_config.get("futures").unwrap().as_array().unwrap();
        // let server_config = config.get("Server").unwrap();
        let symbols = config.get("Symbols").unwrap().as_array().unwrap();
        let key = config.get("Alarm").unwrap().get("webhook").unwrap().as_str().unwrap();
        // info!("获取key");
        // let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
        // wxbot.push_str(key);
        // info!("wxbot  {}", wxbot);
        // let wx_robot = WxbotHttpClient::new(&wxbot);
        info!("preparing...");

        // for s_config in server_config{
        //     let obj = s_config.as_object().unwrap(); 
        //     let host = obj.get("host").unwrap().as_str().unwrap();
        //     let port = obj.get("port").unwrap().as_str().unwrap();
        //     let username = obj.get("username").unwrap().as_str().unwrap();
        //     let password = obj.get("password").unwrap().as_str().unwrap();
        //     let root_path = obj.get("root_path").unwrap().as_str().unwrap();
        //     let root_name = obj.get("root_name").unwrap().as_str().unwrap();
        //     servers_config.insert(String::from("host"), Value::from(host));
        //     servers_config.insert(String::from("port"), Value::from(port));
        //     servers_config.insert(String::from("username"), Value::from(username));
        //     servers_config.insert(String::from("password"), Value::from(password));
        //     servers_config.insert(String::from("root_path"), Value::from(root_path));
        //     servers_config.insert(String::from("root_name"), Value::from(root_name));
        // }
        
        
        
        // let ssh_api = SshClient::new(
        //     server_config.get("host").unwrap().as_str().unwrap(),
        //     server_config.get("port").unwrap().as_str().unwrap(),
        //     server_config.get("username").unwrap().as_str().unwrap(),
        //     server_config.get("password").unwrap().as_str().unwrap(),
        //     server_config.get("root_path").unwrap().as_str().unwrap(),
        //     server_config.get("root_name").unwrap().as_str().unwrap(),
        // );
        

        
        // for f_config in binance_future_config{
        //     let obj = f_config.as_object().unwrap(); 
        //     let base_url = obj.get("base_url").unwrap().as_str().unwrap();
        //     let api_key = obj.get("api_key").unwrap().as_str().unwrap();
        //     let secret_key = obj.get("secret_key").unwrap().as_str().unwrap();
        //     futures_config.insert(String::from("base_url"), Value::from(base_url));
        //     futures_config.insert(String::from("api_key"), Value::from(api_key));
        //     futures_config.insert(String::from("secret_key"), Value::from(secret_key));
        // }

        info!("created ssh client");
        // let binance_futures_api=BinanceFuturesApi::new(
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("base_url")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("api_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        //     binance_config
        //         .get("futures")
        //         .unwrap()
        //         .get("secret_key")
        //         .unwrap()
        //         .as_str()
        //         .unwrap(),
        // );

        
        info!("created http client");

            real_time(symbols).await;
        
    });

    // 开始任务
    info!("alarm begin(binance account)");
    real_time_handle.await.unwrap();
    info!("alarm done");
}
