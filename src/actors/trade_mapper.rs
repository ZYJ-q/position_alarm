pub struct TradeMapper;
pub struct PositionMapper;

pub struct NetWorkMapper;
// use super::http_data::TradeRe;
use crate::actors::database::get_connect;
// use log::info;
use mysql::*;
use mysql::prelude::*;
use tracing_subscriber::fmt::format;
use super::db_data::{Positions, AccAlarm, WxNotices};


impl TradeMapper {
// 获取配置文件数据
pub fn get_positions(tra_id: &u64) -> Result<Vec<Positions>> {
  // 连接数据库
  let mut conn = get_connect();
  let value = &format!("select * from traders where tra_id = {}", tra_id);
  let res = conn.query_map(
    value,
      |(tra_id, tra_venue,  tra_currency, api_key, secret_key, r#type, name,  borrow)| {
        Positions{ tra_id, tra_venue,  tra_currency, api_key, secret_key, r#type, name, borrow }
      }
  ).unwrap();
  return Ok(res);
}

// 获取账户监控信息
pub fn get_acc_alarm() -> Result<Vec<AccAlarm>> {
  // 连接数据库
  let mut conn = get_connect();
  let res = conn.query_map(
    r"select * from acc_alarm",
      |(id, acc_id, tra_id, open_alarm, position_alarm, position_amount, equity_alarm, equity_amount)| {
        AccAlarm{ id, acc_id, tra_id, open_alarm, position_alarm, position_amount, equity_alarm, equity_amount }
      }
  ).unwrap();
  return Ok(res);
}






pub fn get_weixin(acc_id: &u64) -> Result<Vec<WxNotices>> {
  // 连接数据库
  let mut conn = get_connect();
  let value = &format!("select * from wx_notices where acc_id = {}", acc_id);
  let res = conn.query_map(
    value,
    |( id, acc_id, wx_hook, wx_name)| {
      WxNotices{ id, acc_id, wx_hook, wx_name}
    } 
  ).unwrap();
  return Ok(res);
}




}













