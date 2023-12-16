use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool, FromRow, Pool, sqlite::SqliteQueryResult, sqlite::SqliteRow};
const DB_URL: &str = "sqlite://sqlite.db";
use std::io::stdin;
use std::env;



#[derive(Clone, FromRow, Debug)]
struct User {
    chara_id: i64,
    name: String,
    class: String,
    race: String
}

#[allow(dead_code)]
async fn create_db()
{
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

#[allow(dead_code)]
async fn create_table(db: &Pool<Sqlite>, name_table: String)
{
    let input_text = format!("CREATE TABLE IF NOT EXISTS {name_table} (
                                                                                chara_id INTEGER PRIMARY KEY NOT NULL, 
                                                                                name VARCHAR(255) NOT NULL,
                                                                                class VARCHAR(255) NOT NULL,
                                                                                race VARCHAR(255) NOT NULL
                                                                            );");
    let result = sqlx::query(&input_text).execute(db).await.unwrap();
    println!("Create user table result: {:?}", result);
}

async fn select_all(db: &Pool<Sqlite>) -> Vec<SqliteRow>
{
    let result_vec = sqlx::query(
        "SELECT name, class, race
         FROM characters",
    )
    .fetch_all(db)
    .await
    .unwrap();
    result_vec
}

async fn insert_in_table(db: &Pool<Sqlite>, table_name:String, c_name: String, c_class: String, c_race: String)  -> SqliteQueryResult {
    let command_sql_text: String = format!("INSERT INTO {table_name} VALUES (?, '{c_name}', '{c_class}', '{c_race}')");
    let result = sqlx::query(&command_sql_text)
    .execute(db)
    .await
    .unwrap();
    println!("Query result: {:?}", result);
    result
}

async fn print_all_shit(db: &Pool<Sqlite>, table_name:String)
{
    let cmd_sql = format!("SELECT chara_id, name, class, race FROM {table_name}");
    let user_results = sqlx::query_as::<_, User>(&cmd_sql)
    .fetch_all(db)
    .await
    .unwrap();
    for user in user_results {
        println!("[{}] name: {}   class: {}   race:  {}", user.chara_id, &user.name, &user.class, &user.race);
    }
}

async fn update_row(db: &Pool<Sqlite>, table_name:String) -> SqliteQueryResult
{
    let command_sql_text: String = format!("UPDATE {table_name} SET name = 'Asana' WHERE {table_name}.chara_id = 1;");
    let result = sqlx::query(&command_sql_text)
    .execute(db)
    .await
    .unwrap();
    println!("Query result: {:?}", result);
    result
}


#[tokio::main]
async fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    //create_db().await;
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let mut iname_tbl: String = String::new();
    iname_tbl = "characters".to_string();
    //let _result = stdin().read_line(&mut iname_tbl);
    //create_table(&db, iname_tbl.clone()).await;
    let _result = select_all(&db).await;
    
    let _result = update_row(&db, iname_tbl.clone()).await;
    //for (idx, row) in result.iter().enumerate() {
    //    println!("[{}]: {:?}, {:?}, {:?}", idx, row.get::<String, &str>("name"), row.get::<String, &str>("class"), row.get::<String, &str>("race"));
    //}

    println!("abs");
    //let _result = insert_in_table(&db, iname_tbl.clone(), "Orman".to_string(), "Warrior".to_string(), "Ork".to_string()).await;

    println!("abx");
    print_all_shit(&db, iname_tbl.clone()).await;
    
    
}