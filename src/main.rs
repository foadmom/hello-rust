use std::thread::sleep;

// 
//
// 
use redis::{Commands};

fn main() {
    println!("Application started!");

    redis_test();

    println! ("Application finished!");
}

fn redis_test () {

    // let mut result: redis::Connection = match redis::Client::open("redis://localhost:6379/") {
    //     Ok(client) => {
    //         match client.get_connection() {
    //             Ok(conn) => conn,
    //             Err(e) => {
    //                 println!("Failed to connect to Redis: {e}");
    //                 return;
    //             }
    //         }
    //     },
    //     Err(e) => {
    //         println!("Failed to create Redis client: {e}");
    //         return;
    //     }
    // };
    
    let open_result =  redis::Client::open("redis://localhost:6379/");
    let my_client: redis::Client;
    match open_result {
        Ok(client) => {
            println! ("Successfully created Redis client");
            my_client = client;
        },
        Err(e) => {
            println!("Failed to create Redis client: {e}");
            return;
        }
    };

    let mut my_conn: redis::Connection;
    let conn_result: redis::RedisResult<redis::Connection>;
    conn_result = my_client.get_connection_with_timeout(std::time::Duration::from_secs(5));
    match conn_result {
        Ok(conn) => {
            println!("Successfully connected to Redis");
            my_conn = conn;
        },
        Err(e) => {
            println!("Failed to connect to Redis: {e}");
            return;
        }
    };

    let res: bool = redis_set(&mut my_conn, "name", "Elliot Rusty");
    println!("Result of set is {}", res);

    let my_value: Option<String> = redis_get(&mut my_conn, "name");
    println!("Key 'name' found in Redis and value is {} ", my_value.unwrap_or("Something went wrong".to_string()));
    // if my_value.is_some() {
    //     let local_value: String = my_value.clone().unwrap();
    //     println!("Key 'name' found in Redis and value is {} ", my_value.unwrap_or("Something went wrong".to_string()));
    // }

    for i in 0..100 {
        sleep(std::time::Duration::from_secs(10));
        let new_value: String = format!("{} - Elliot Rusty", i);
        redis_set(&mut my_conn, "name", &new_value);
    }
}

fn redis_get (conn: &mut redis::Connection, key: &str) -> Option<String> {
    let get_result: redis::RedisResult<String>;
    get_result = conn.get::<&str, String>(key);
    match get_result {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn redis_set (conn: &mut redis::Connection, key: &str, value: &str) -> bool {
    let set_result: redis::RedisResult<()>;
    set_result = conn.set::<&str, &str, ()>(key, value);
    match set_result {
        Ok(_) => true,
        Err(_) => false,
    }
}