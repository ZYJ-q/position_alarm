
// use std::collections::VecDeque;
use std::{collections::HashMap, fs, time::Duration};
use log::{info, warn};
use positions_alarm::adapters::binance;
use serde_json::{Map, Value};
// use tokio::{sync::broadcast::{self, Receiver}};
use positions_alarm::adapters::binance::futures::http::actions::BinanceFuturesApi;
use positions_alarm::adapters::bybit::futures::http::actions::ByBitFuturesApi;
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
        // let mut server_status: VecDeque<Value> = VecDeque::new();
        // let mut server_process: Map<String, Value> = Map::new();
        // print!("判断是true还是false {}", ssh_api.search_py_ps());
        // match ssh_api.search_py_ps() {
        //     true => {
        //         if !running {
        //             running = true;
        //             print!("改变running的值{}", running);
        //             // let sender = "程序开启";
        //             // let content = format!("process name: {}", ssh_api.get_root_name());
        //             // wx_robot.send_text(sender, &content).await;
        //         }
        //         server_process.insert(String::from("status"), Value::from("running"));
        //         server_process.insert(String::from("info"), Value::from(""));
        //     }
        //     false => {
        //         server_process.insert(String::from("status"), Value::from("stopped"));
        //         let mut info = ssh_api.download_log();
        //         if running {
        //             running = false;
        //             // let sender = "程序停止";
        //             let content;
        //             if info == "" {
        //                 content = format!("{}: 未找到错误，请查看日志", ssh_api.get_root_name());
        //             }else {
        //                 content = format!("{}: {}", ssh_api.get_root_name(), &info);
        //             }
        //             // wx_robot.send_text(sender, &content).await;
        //             info = content;
        //         }
        //         server_process.insert(String::from("info"), Value::from(info));
        //     }
        // }
        // map.insert(String::from("server"), Value::from(server_process));


        let res = trade_mapper::TradeMapper::get_positions();
        let weixin = trade_mapper::TradeMapper::get_weixin().unwrap();
        println!("res:{:?}", res);

        

        
        if let Ok(a) = res{
        for f_config in a {

            let tra_name = &f_config.name;
            let threshold = f_config.threshold;
            let new_tra_id = &f_config.tra_id;
            let thres_am = &f_config.amount;
            let amount: f64  = thres_am.parse().unwrap();
            
            if &f_config.tra_venue == "Binance" && &f_config.r#type == "Futures" {
                let binance_futures_api=BinanceFuturesApi::new(
                    "https://fapi.binance.com",
                    &f_config.api_key,
                    &f_config.secret_key,
                );
                let name = tra_name;
                
                if threshold == "true" && thres_am.len() != 0 {
                    if let Some(data) = binance_futures_api.account(None).await {
                        let v: Value = serde_json::from_str(&data).unwrap();
                        let positions = v.as_object().unwrap().get("positions").unwrap().as_array().unwrap();
                        let mut amts: f64 = 0.0;
                        // let mut prices: f64 = 0.0;
                        
                        println!("获取到的账户持仓:{:?}, 名字{}, 阈值{}", positions, name, threshold);
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
                            for f_weixin in &weixin {
                                let tra_id = &f_weixin.tra_id;
                                let wx_hook = &f_weixin.wx_hook;
                                let slack_hook = &f_weixin.slack_hook;
                                if new_tra_id == tra_id {
                                    if wx_hook.len() != 0 {
                                        let mut wxbot = String::from("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=");
                                        wxbot.push_str(&wx_hook);
                                        info!("wxbot  {}", wxbot);
                                        let wx_robot = WxbotHttpClient::new(&wxbot);

                                        let sender = format!("Binance交易所的----{}普通账号", name);
                        let content = format!("净头寸高于阈值");
                        wx_robot.send_text(&sender, &content).await;

                                    }

                                    if slack_hook.len() != 0 {
                                        let mut slackrobot = String::from("https://hooks.slack.com/services/");
                slackrobot.push_str(&slack_hook);
                let slack_robot = SlackHttpClient::new(&slackrobot);

                let sender = format!("Binance交易所的----{}普通账号", name);
                let content = format!("净头寸高于阈值");
                slack_robot.send_text(&sender, &content).await;

                                    }
                                }
                            }
                            println!("高于阈值")
                        }
                        // net_worth = notional_total/ori_fund;
                        // net_worth_histories.push_back(Value::from(new_account_object));
                    }
                }
            }


            if &f_config.tra_venue == "ByBit" {
                let bybit_futures_api=ByBitFuturesApi::new(
                    "https://api.bybit.com",
                    &f_config.api_key,
                    &f_config.secret_key,
                );
                let name = tra_name;

                if threshold == "true" && thres_am.len() != 0 {
                    if let Some(data) = bybit_futures_api.get_account_overview(None).await {
                        let value: Value = serde_json::from_str(&data).unwrap();
                        let mut spot_positions = 0.0;
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
                                        let usd_value = objs.get("usdValue").unwrap().as_str().unwrap();
                                        println!("wallet_balance:{}, usd_value:{}", amt, usd_value);
                                        

                                    }
                                }
                            }
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
