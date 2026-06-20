use std::collections::HashMap;

use sqlx::PgPool;
use tokio::sync::{Mutex, broadcast};

use crate::config::Config;

#[derive(Debug, Clone)]
 pub struct DbClient{
    pub pool: PgPool  
 }

#[derive(Debug)]
 pub struct AppState{
    pub db_client: DbClient,
    pub envs: Config,
   //  you can use a dashmap if this app should scale later on
    pub tx: Mutex<HashMap<String, broadcast::Sender<String>>>
 }