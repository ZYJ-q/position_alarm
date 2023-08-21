pub struct TradeMapper;
pub struct PositionMapper;

pub struct NetWorkMapper;
// use super::http_data::TradeRe;
use crate::actors::database::get_connect;
// use log::info;
use mysql::*;
use mysql::prelude::*;
use super::db_data::{Positions, AccWeixin};


impl TradeMapper {
// 获取配置文件数据
pub fn get_positions() -> Result<Vec<Positions>> {
  // 连接数据库
  let mut conn = get_connect();
  let res = conn.query_map(
    r"select * from trader",
    |(tra_id, tra_venue,  tra_currency, api_key, secret_key, r#type, name, alarm, threshold, borrow, amount, wx_hook)| {
      Positions{ tra_id, tra_venue,  tra_currency, api_key, secret_key, r#type, name, alarm, threshold, borrow, amount, wx_hook }
    } 
  ).unwrap();
  return Ok(res);
}


pub fn get_weixin() -> Result<Vec<AccWeixin>> {
  // 连接数据库
  let mut conn = get_connect();
  let res = conn.query_map(
    r"select * from notices",
    |( id, tra_id, wx_hook, wx_name, slack_hook, slack_name, mess_hook, mess_name)| {
      AccWeixin{ id, tra_id, wx_hook, wx_name, slack_hook, slack_name, mess_hook, mess_name}
    } 
  ).unwrap();
  return Ok(res);
}




}













