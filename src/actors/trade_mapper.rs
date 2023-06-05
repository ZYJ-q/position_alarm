pub struct TradeMapper;
pub struct PositionMapper;

pub struct NetWorkMapper;
// use super::http_data::TradeRe;
use crate::actors::database::get_connect;
// use log::info;
use mysql::*;
use mysql::prelude::*;
use super::db_data::Positions;


impl TradeMapper {
  // 获取配置文件数据
  pub fn get_positions() -> Result<Vec<Positions>> {
    // 连接数据库
    let mut conn = get_connect();
    let res = conn.query_map(
      r"select * from position_alarm",
      |(id, api_key, secret_key, name, threshold)| {
        Positions{ id, api_key, secret_key, name, threshold }
      } 
    ).unwrap();
    return Ok(res);
  }
}













