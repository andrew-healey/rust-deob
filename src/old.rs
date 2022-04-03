use std::fs;
use std::time::{Duration,Instant};

use std::error::Error;

use esprit::script;
use easter::{expr::Expr,stmt::{StmtListItem,Stmt}};
use anyhow::anyhow;

#[tokio::main]
async fn main()->Result<(), Box<dyn Error>> {

    let contents=fs::read_to_string("bg.js")?;

    let start=Instant::now();

    let scr=script(&contents[..]).map_err(|_err|anyhow!("Parse error"))?;

    println!("micros: {}",start.elapsed().subsec_micros());

    /*
    println!("{:?}",&scr);

    println!("{}",&contents[..]);
    */

    Ok(())
}
