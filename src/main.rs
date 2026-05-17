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
    // let mut client = match redis::Client::open("redis://localhost:6379/") {
    //         Ok(client) => {
    //             match client.get_connection_with_timeout(std::time::Duration::from_secs(5)) {
    //                 Ok(conn) => conn,
    //                 Err(e) => {
    //                     println!("Failed to connect to Redis: {e}");
    //                     return;
    //                 }
    //             }
    // }        Err(e) => {
    //         println!("Failed to create Redis client: {e}");
    //         return;
    //     }
    // };

    let conn: redis::Connection;
    let mut client =  redis::Client::open("redis://localhost:6379/");
    match (client) {
            Ok(client) => {
                conn = client.get_connection_with_timeout(std::time::Duration::from_secs(5));
                match client.get_connection_with_timeout(std::time::Duration::from_secs(5)) {
                    Ok(conn) => conn,
                    Err(e) => {
                        println!("Failed to connect to Redis: {e}");
                        return;
                    }
                }
    }        Err(e) => {
            println!("Failed to create Redis client: {e}");
            return;
        }
    };

    // if let Ok(res) = client.set("user_session:100", "active") {
    //         let res: String = res;
    //         println!("{res}");    // >>> OK
    //     } else {
    //         println!("Error setting foo");
    //     }

    // Try to get a value using the connection
    let mut value: String = conn.get::<_, String>("user_session:100");
    match conn.get::<_, String>("user_session:100") {
        Ok(status) => println!("Session Status: {}", status),
        Err(e) => println!("Error retrieving session status: {}", e),
    }


    // // Execute a GET command and read the result into a String
    // let status: String = con.get("user_session:100").unwrap();
    // match client.get ("user_session:100") {
    //     Ok(status) => println!("Session Status: {}", status),
    //     Err(e) => println!("Error retrieving session status: {}", e),
    // }
}

