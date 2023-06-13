use crate::customer::Customer;

pub struct Event {
    pub id: u64,
    pub customer: Customer,
    pub scheduled_time: f64,
    pub chosen_queue: Option<u64>,
}

impl Event {
    pub fn new(
        id: u64,
        customer: Customer,
        scheduled_time: f64,
        chosen_queue: Option<u64>,
    ) -> Event {
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

        let scheduled_time_format = format_time(self.scheduled_time);

        format!(
            //{:<8.2} for self.scheduled_time
            "{:<8} | {:<10} | {:<8} | {:<4}",
            scheduled_time_format, event_type, self.customer.id, chosen_queue_str
        )
    }
}

fn format_time(minutes: f64) -> String {
    let hours = (4.0 + (minutes / 60.0)) % 24.0;
    let is_pm = hours >= 12.0;
    let formatted_hours = if hours == 0.0 || hours == 12.0 {
        12
    } else {
        (hours % 12.0) as u32
    };
    let formatted_minutes = (minutes % 60.0) as u32;

    let am_pm = if is_pm { "PM" } else { "AM" };
    format!("{:02}:{:02} {}", formatted_hours, formatted_minutes, am_pm)
}
