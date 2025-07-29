use super::config_util::CFG;
use crate::model::config::Database;
use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use thiserror::Error;

fn get_mysql_url() -> String {
    let Database {
        user,
        password,
        host,
        port,
        dbname,
    } = &CFG.database;
    format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, dbname)
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to initialize RBatis: {0}")]
    InitError(#[from] rbatis::error::Error),
}

/**
 * 获取db实例
 */
pub async fn get_db_instance() -> Result<RBatis, DatabaseError> {
    let rb = RBatis::new();
    rb.init(MysqlDriver {}, &get_mysql_url())?;
    Ok(rb)
}

#[cfg(test)]
mod test {
    use super::*;

    #[actix_rt::test]
    async fn test_connect() {
        let rb = get_db_instance().await.unwrap();

        let count: u64 = rb
            .query_decode("select count(1) from sys_role", vec![])
            .await
            .unwrap();
        println!("{}", count);
    }
}
