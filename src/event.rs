use crate::customer::Customer;

pub struct Event {
    pub id: u64,
    pub customer: Customer,
    pub scheduled_time: f64,
    pub chosen_queue: Option<u64>,
}

impl Event {
    pub fn new(id: u64, customer: Customer, scheduled_time: f64, chosen_queue: Option<u64>) -> Event {
        Event {
            id,
            customer,
            scheduled_time,
            chosen_queue,
        }
    }

    pub fn pretty_print(&self) -> String {
        let event_type = match self.id {
            0 => "ARRIVE",
            1 => "QUEUE",
            2 => "REFUEL",
            3 => "PAYMENT",
            4 => "DEPARTURE",
            _ => "UNKNOWN",
        };

        let chosen_queue_str = match self.chosen_queue {
            Some(queue) => queue.to_string(),
            None => "-".to_string(),
        };

        format!(
            "{:<8.2} | {:<10} | {:<8} | {:<4}",
            self.scheduled_time,
            event_type,
            self.customer.id,
            chosen_queue_str
        )
    }
}