use aspirin_eats::db::AspirinEatsDb;
use aspirin_eats::error::AspirinEatsError;
use aspirin_eats::food::*;
use aspirin_eats::http::{HttpRequest, HttpResponse};
use serde_json::to_string;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::{from_utf8, FromStr},
};

/// Change this path to match where you want to store the database file
const DB_PATH: &str = "/Users/rishitbansal/Downloads/aspirin_eats.db";

fn handle_connection(mut stream: TcpStream, db: &AspirinEatsDb) {
    let mut buffer = [0; 65536];

    let bytes_read = stream
        .read(&mut buffer)
        .map_err(AspirinEatsError::Io)
        .unwrap();

    let request_str = match from_utf8(&buffer[..bytes_read]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to convert buffer to string: {:?}", e);
            return;
        }
    };

    if request_str.is_empty() {
        eprintln!("Failed to parse empty request");
        return;
    }

    let request = match HttpRequest::from_str(request_str) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to parse HTTP request: {:?}", e);
            return;
        }
    };

    let response = {
        if request.path.is_none() || request.path == Some("/".to_string()) {
            HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!")
        } else {
            match request.method.as_deref() {
                Some("GET") => match get_request(&request, db) {
                    Ok(body) => HttpResponse::new(200, "OK", &body),
                    Err(e) => HttpResponse::from(e),
                },
                Some("POST") => match post_request(&request, db) {
                    Ok(()) => HttpResponse::new(200, "OK", "OK"),
                    Err(e) => HttpResponse::from(e),
                },
                Some("DELETE") => match delete_request(&request, db) {
                    Ok(()) => HttpResponse::new(200, "OK", "OK"),
                    Err(e) => HttpResponse::from(e),
                },
                _ => HttpResponse::from(AspirinEatsError::MethodNotAllowed),
            }
        }
    };

    stream
        .write(response.to_string().as_bytes())
        .map_err(AspirinEatsError::Io)
        .unwrap();
}

fn get_request(request: &HttpRequest, db: &AspirinEatsDb) -> Result<String, AspirinEatsError> {
    match request.path.as_deref() {
        Some("/orders") => {
            let orders = db.get_all_orders().map_err(AspirinEatsError::Database)?;
            Ok(to_string(&orders).unwrap())
        }

        Some(path) if path.starts_with("/orders/") => {
            if let Some(id_str) = path.strip_prefix("/orders/") {
                match id_str.parse::<i64>() {
                    Ok(id) => {
                        if let Some(order) = db.get_order(id)? {
                            Ok(to_string(&order).unwrap())
                        } else {
                            Err(AspirinEatsError::NotFound)
                        }
                    }
                    Err(_) => Err(AspirinEatsError::InvalidRequest),
                }
            } else {
                Err(AspirinEatsError::InvalidRequest)
            }
        }

        _ => Err(AspirinEatsError::NotFound),
    }
}

fn post_request(request: &HttpRequest, db: &AspirinEatsDb) -> Result<(), AspirinEatsError> {
    match request.path.as_deref() {
        Some("/orders") => {
            if let Some(body) = &request.body {
                let order_request: OrderRequest =
                    serde_json::from_str(body).map_err(AspirinEatsError::ParseError)?;

                let order = Order::from(order_request);

                db.add_order(order).map_err(AspirinEatsError::Database)?;
            } else {
                return Err(AspirinEatsError::InvalidRequest);
            }
        }
        _ => return Err(AspirinEatsError::InvalidRequest),
    }

    Ok(())
}

fn delete_request(request: &HttpRequest, db: &AspirinEatsDb) -> Result<(), AspirinEatsError> {
    match request.path.as_deref() {
        Some("/orders") => {
            db.reset_orders().map_err(AspirinEatsError::Database)?;
            Ok(())
        }

        Some(path) if path.starts_with("/orders/") => {
            if let Some(id_str) = path.strip_prefix("/orders/") {
                match id_str.parse::<i64>() {
                    Ok(id) => match db.get_order(id) {
                        Ok(_) => {
                            db.remove_order(id).map_err(AspirinEatsError::Database)?;
                            Ok(())
                        }
                        Err(_) => Err(AspirinEatsError::NotFound),
                    },
                    Err(_) => Err(AspirinEatsError::InvalidRequest),
                }
            } else {
                Err(AspirinEatsError::InvalidRequest)
            }
        }

        _ => Err(AspirinEatsError::NotFound),
    }
}

fn main() {
    let db = AspirinEatsDb::from_path(DB_PATH).expect("Failed to open database");

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &db);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aspirin_eats::http::HttpRequest;

    #[test]
    fn test_handle_connection_get_orders() {
        // Create an in-memory database
        let db = AspirinEatsDb::in_memory().unwrap();

        // Add an order to the database
        let order_request = OrderRequest {
            customer: "Alice".to_string(),
            food: vec![
                MenuItem::Burger(Burger::new(
                    Bun::Sesame,
                    Patty::Beef,
                    vec![Topping::Lettuce, Topping::Tomato],
                )),
                MenuItem::Fries,
                MenuItem::Drink,
            ],
        };
        let order: Order = order_request.into();
        db.add_order(order).unwrap();

        // Prepare a GET request
        let request = HttpRequest {
            method: Some("GET".to_string()),
            path: Some("/orders".to_string()),
            body: None,
        };

        // Simulate handling the request
        let response = get_request(&request, &db);

        assert!(response.is_ok());
        let orders: Vec<Order> = serde_json::from_str(&response.unwrap()).unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].customer, "Alice");
        assert_eq!(orders[0].total, 17.0); // Total for 1 burger + fries + drink
    }

    #[test]
    fn test_handle_connection_post_order() {
        // Create an in-memory database
        let db = AspirinEatsDb::in_memory().unwrap();

        // Prepare a POST request with order data
        let order_request = OrderRequest {
            customer: "Bob".to_string(),
            food: vec![
                MenuItem::Burger(Burger::new(
                    Bun::GlutenFree,
                    Patty::Chicken,
                    vec![Topping::Cheese],
                )),
                MenuItem::Fries,
            ],
        };
        let order_request_json = serde_json::to_string(&order_request).unwrap();

        let request = HttpRequest {
            method: Some("POST".to_string()),
            path: Some("/orders".to_string()),
            body: Some(order_request_json),
        };

        // Simulate handling the request
        let result = post_request(&request, &db);

        assert!(result.is_ok());

        // Verify the order was added
        let orders = db.get_all_orders().unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].customer, "Bob");
        assert_eq!(orders[0].total, 15.0);
    }

    #[test]
    fn test_handle_connection_delete_order() {
        // Create an in-memory database
        let db = AspirinEatsDb::in_memory().unwrap();

        // Add an order to the database
        let order_request = OrderRequest {
            customer: "Charlie".to_string(),
            food: vec![MenuItem::Burger(Burger::new(
                Bun::Plain,
                Patty::Veggie,
                vec![Topping::Bacon],
            ))],
        };
        let order: Order = order_request.into();
        db.add_order(order).unwrap();

        // Prepare a DELETE request
        let request = HttpRequest {
            method: Some("DELETE".to_string()),
            path: Some("/orders/1".to_string()),
            body: None,
        };

        // Simulate handling the request
        let result = delete_request(&request, &db);

        assert!(result.is_ok());

        // Verify the order was deleted
        let orders = db.get_all_orders().unwrap();
        assert_eq!(orders.len(), 0);
    }

    #[test]
    fn test_get_request() {
        // Create an in-memory database
        let db = AspirinEatsDb::in_memory().unwrap();

        // Add an order to the database
        let order_request = OrderRequest {
            customer: "Diana".to_string(),
            food: vec![MenuItem::Burger(Burger::new(
                Bun::Sesame,
                Patty::Beef,
                vec![Topping::Cheese, Topping::Onion],
            ))],
        };
        let order: Order = order_request.into();
        db.add_order(order).unwrap();

        // Prepare a GET request
        let request = HttpRequest {
            method: Some("GET".to_string()),
            path: Some("/orders".to_string()),
            body: None,
        };

        // Simulate handling the request
        let result = get_request(&request, &db);

        assert!(result.is_ok());
        let orders: Vec<Order> = serde_json::from_str(&result.unwrap()).unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].customer, "Diana");
        assert_eq!(orders[0].total, 10.0);
    }

    #[test]
    fn test_post_request() {
        // Create an in-memory database
        let db = AspirinEatsDb::in_memory().unwrap();

        // Prepare a POST request with order data
        let order_request = OrderRequest {
            customer: "Eve".to_string(),
            food: vec![
                MenuItem::Burger(Burger::new(
                    Bun::GlutenFree,
                    Patty::Chicken,
                    vec![Topping::Bacon, Topping::Cheese],
                )),
                MenuItem::Fries,
            ],
        };
        let order_request_json = serde_json::to_string(&order_request).unwrap();

        let request = HttpRequest {
            method: Some("POST".to_string()),
            path: Some("/orders".to_string()),
            body: Some(order_request_json),
        };

        // Simulate handling the request
        let result = post_request(&request, &db);

        assert!(result.is_ok());

        // Verify the order was added
        let orders = db.get_all_orders().unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].customer, "Eve");
        assert_eq!(orders[0].total, 17.0);
    }

    #[test]
    fn test_delete_request() {
        // Create an in-memory database
        let db = AspirinEatsDb::in_memory().unwrap();

        // Add an order to the database
        let order_request = OrderRequest {
            customer: "Frank".to_string(),
            food: vec![
                MenuItem::Burger(Burger::new(
                    Bun::Sesame,
                    Patty::Veggie,
                    vec![Topping::Lettuce, Topping::Tomato],
                )),
                MenuItem::Drink,
            ],
        };
        let order: Order = order_request.into();
        db.add_order(order).unwrap();

        // Prepare a DELETE request
        let request = HttpRequest {
            method: Some("DELETE".to_string()),
            path: Some("/orders/1".to_string()),
            body: None,
        };

        // Simulate handling the request
        let result = delete_request(&request, &db);

        assert!(result.is_ok());

        // Verify the order was deleted
        let orders = db.get_all_orders().unwrap();
        assert_eq!(orders.len(), 0);
    }
}
