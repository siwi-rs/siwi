use siwi::Result;
pub struct Logger;
#[async_trait::async_trait]
impl siwi::Middleware for Logger {
  async fn handle(&self, req: siwi::Request, next: siwi::Next<'_>) -> Result {
    let method = req.method().to_string();
    let uri = req.uri().to_string();
    let res = next.run(req).await?;
    let status = res.status().to_string();
    let log = format!("{} {} {}", method, uri, status);
    println!("{}", log);
    Ok(res)
  }
}
