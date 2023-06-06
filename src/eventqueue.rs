use crate::event::Event;

pub struct EventQueue {
    pub q: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue { q: Vec::new() }
    }

    pub fn add(&mut self, event: Event) {
        let index = self
            .q
            .binary_search_by(|existing_event| {
                if existing_event.scheduled_time == event.scheduled_time {
                    if existing_event.id == event.id {
                        existing_event.customer.id.cmp(&event.customer.id)
                    } else {
                        existing_event.id.cmp(&event.id)
                    }
                } else {
                    existing_event
                        .scheduled_time
                        .partial_cmp(&event.scheduled_time)
                        .unwrap()
                }
            })
            .unwrap_or_else(|index| index);

        self.q.insert(index, event);
    }
}
