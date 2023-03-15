use std::sync::Arc;
use std::sync::Mutex;

#[derive(PartialEq, Debug)]
pub enum EventType {
    Restock,
    Update,
    Buy,
}

#[derive(Debug)]
pub struct Log {
    time: chrono::DateTime<chrono::Local>,
    event_type: EventType,
    id: i32,
    old_stock: Option<i32>,
    new_stock: Option<i32>,
    price: Option<f64>,
}

pub type Logs = Arc<Mutex<Vec<Log>>>;

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.event_type {
                EventType::Restock => format!(
                    "[{}] Restock book {}: stock {} => {}",
                    self.time,
                    self.id,
                    self.old_stock.unwrap(),
                    self.new_stock.unwrap()
                ),
                EventType::Update => format!(
                    "[{}] Update book {} price to {}",
                    self.time,
                    self.id,
                    self.price.unwrap()
                ),
                EventType::Buy => format!(
                    "[{}] Buy book {}: stock {} => {}",
                    self.time,
                    self.id,
                    self.old_stock.unwrap(),
                    self.new_stock.unwrap()
                ),
            }
        )
    }
}

impl Log {
    pub fn new_buy(id: i32, old_stock: i32, new_stock: i32) -> Self {
        Self {
            time: chrono::Local::now(),
            event_type: EventType::Buy,
            id,
            old_stock: Some(old_stock),
            new_stock: Some(new_stock),
            price: None,
        }
    }

    pub fn new_restock(id: i32, old_stock: i32, new_stock: i32) -> Self {
        Self {
            time: chrono::Local::now(),
            event_type: EventType::Restock,
            id,
            old_stock: Some(old_stock),
            new_stock: Some(new_stock),
            price: None,
        }
    }

    pub fn new_price(id: i32, price: f64) -> Self {
        Self {
            time: chrono::Local::now(),
            event_type: EventType::Update,
            id,
            old_stock: None,
            new_stock: None,
            price: Some(price),
        }
    }
}
