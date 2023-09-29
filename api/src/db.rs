use postgres::{Client, NoTls};

pub fn establish_connection() -> Result<(), postgres::Error> {

  let conn_string = "bubble.db.elephantsql.com (bubble-01)";
  
  let client = Client::connect(conn_string, NoTls)?;

  // Use client to execute queries
  //client.batch_execute("SELECT * FROM users")?;

  Ok(())

}
//"bubble.db.elephantsql.com (bubble-01)"//"bubble.db.elephantsql.com (bubble-01)"