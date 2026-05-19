// 
//
// 
use redis::Commands;

fn main() {
    println!("Hello, world!");

    let i: i32 = 42;
    println!("The value of i is: {}", i);

    let f: f64 = 3.14;
    println!("The value of f is: {}, {}", i, f);

    let b: i32 = i;
    println!("The value of b is: {}", b);
    println!("The value of i is: {}", i);

    let mut ys: [i32; 500] = [0; 500];

    for _i in 0..500 {
        ys[_i] = _i as i32;
    }
    println!("The value of ys[1] is: {}", ys[499]);

    redis_test();
}

fn redis_test () {
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

    let mut get_value: String = "uninitialized".to_string();
    let get_result: redis::RedisResult<String>;
    get_result = my_conn.get::<&str, String>("name");
    // match my_conn.get::<&str, String>("name") {
    match get_result {
        Ok(value) => {
            // println!("Value for 'name': {}", value);
            get_value = value;
            // println!("get_value for 'name': {}", get_value);
        },
        Err(_) => println!("Key 'Elliot' not found"),
    };

    if get_value == "Elliot" {
        println!("Value for 'name' is Elliot");
    } else {
        println!("Value for 'name' is not Elliot");
    }
        
}