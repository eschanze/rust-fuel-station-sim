#[macro_use]
extern crate timeit;

mod customer;
mod event;
use customer::{Customer, PaymentMethod};
use event::Event;

use std::env;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_distr::{Normal, Distribution};

use plotly::*;
use plotly::common::Mode;

#[allow(unused_macros)]
macro_rules! cumulative_sum {
    ($a_vec:expr) => {{
        let mut cumsum = Vec::new();
        let mut acc = 0.0;
        for x in $a_vec {
            acc += x;
            cumsum.push(acc);
        }
        cumsum
    }};
}

struct EventQueue {
    q: Vec<Event>,
}

impl EventQueue {
    fn new() -> EventQueue {
        EventQueue { q: Vec::new() }
    }

    fn add(&mut self, event: Event) {
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
                    existing_event.scheduled_time.partial_cmp(&event.scheduled_time).unwrap()
                }
            })
            .unwrap_or_else(|index| index);

        self.q.insert(index, event);
    }
}

fn time_routine(event_queue: &mut EventQueue, clock: &mut f64) -> Option<Event> {
    if !event_queue.q.is_empty() {
        let event = event_queue.q.remove(0);
        *clock = event.scheduled_time as f64;
        Some(event)
    } else {
        None
    }
}

fn format_customer_queues(customer_queues: &[Vec<Customer>]) -> String {
    let queue_string: Vec<String> = customer_queues
        .iter()
        .map(|queue| {
            let ids: Vec<String> = queue.iter().map(|customer| customer.id.to_string()).collect();
            format!("[ {} ]", ids.join(" "))
        })
        .collect();
    format!("[{}]", queue_string.join(" "))
}

fn any_available(arr: &[i64]) -> (bool, Option<u64>) {
    let zero_indexes: Vec<u64> = arr
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| {
            if value == 0 {
                Some(index as u64)
            } else {
                None
            }
        })
        .collect();

    let has_zero = !zero_indexes.is_empty();
    let random_index = zero_indexes.choose(&mut rand::thread_rng()).copied();

    (has_zero, random_index)
}

fn get_shortest_or_random_index(customer_queues: &[Vec<Customer>]) -> usize {
    let mut shortest_indexes = Vec::new();
    let mut shortest_length = usize::MAX;

    for (index, queue) in customer_queues.iter().enumerate() {
        let queue_length = queue.len();
        if queue_length < shortest_length {
            shortest_length = queue_length;
            shortest_indexes.clear();
            shortest_indexes.push(index);
        } else if queue_length == shortest_length {
            shortest_indexes.push(index);
        }
    }

    let mut rng = rand::thread_rng();
    if shortest_indexes.is_empty() {
        rng.gen_range(0..customer_queues.len())
    } else {
        shortest_indexes[rng.gen_range(0..shortest_indexes.len())]
    }
}

fn process_customer_queues(customer_queues: &mut [Vec<Customer>], index: usize) -> Option<Customer> {
    if !customer_queues.is_empty() {
        if let Some(customer) = customer_queues[index].first().cloned() {
            customer_queues[index].remove(0);
            return Some(customer);
        }
    }
    None
}

fn arrive_routine(
    event_queue: &mut EventQueue, 
    sim_time: &mut f64, 
    e: &mut Event, 
    customer_count: &mut u64
) {
    *customer_count += 1;
    let customer_frequency: f64 = rand::thread_rng().gen_range(15.0..=45.0);

    let queue_event = Event::new(1, e.customer.clone(), *sim_time, None);
    event_queue.add(queue_event);

    let new_customer = Customer::new(*customer_count, *sim_time + customer_frequency);
    let new_event = Event::new(0, new_customer, *sim_time + customer_frequency, None);
    event_queue.add(new_event);
}

fn queue_routine(
    event_queue: &mut EventQueue,
    sim_time: &mut f64,
    e: &mut Event,
    fuel_stations: &mut [i64],
    customer_queues: &mut Vec<Vec<Customer>>,
) {
    let (available_station, idx) = any_available(&fuel_stations);
    if available_station {
        if let Some(station_idx) = idx {
            let refuel_event = Event::new(2, e.customer.clone(), *sim_time, idx);
            event_queue.add(refuel_event);
            fuel_stations[station_idx as usize] = 1;
        }
    } else {
        let queue_index = get_shortest_or_random_index(customer_queues);
        customer_queues[queue_index].push(e.customer.clone());
    }
}

fn refuel_routine(
    event_queue: &mut EventQueue, 
    sim_time: &mut f64, 
    e: &mut Event
) {
    let refuel_time: f64 = 120.0; 
    let payment_event = Event::new(3, e.customer.clone(), *sim_time + refuel_time, e.chosen_queue);
    event_queue.add(payment_event);
}

fn payment_routine(
    event_queue: &mut EventQueue, 
    sim_time: &mut f64, 
    e: &mut Event
) {
    let payment_time: f64; 
    match e.customer.payment_method {
        PaymentMethod::Efectivo => payment_time = rand::thread_rng().gen_range(40.0..=60.0),
        PaymentMethod::Tarjeta => payment_time = rand::thread_rng().gen_range(20.0..=30.0),
        PaymentMethod::CopecApp => payment_time = rand::thread_rng().gen_range(10.0..=20.0)
    }

    let departure_event = Event::new(4, e.customer.clone(), *sim_time + payment_time, e.chosen_queue);
    event_queue.add(departure_event);
}

fn departure_routine(
    event_queue: &mut EventQueue, 
    sim_time: &mut f64, 
    e: &mut Event, 
    fuel_stations: &mut [i64], 
    customer_queues: &mut Vec<Vec<Customer>>,
) {
    e.customer.total_time = *sim_time - e.customer.arrive_time;
    if let Some(queue) = e.chosen_queue {
        fuel_stations[queue as usize] = 0;
        if let Some(customer) = process_customer_queues(customer_queues, queue as usize) {
            let refuel_event = Event::new(2, customer, *sim_time, Some(queue)); 
            event_queue.add(refuel_event);
        }
    }
    /* println!(
        "Customer {} que pagó con {:?} terminó después de {:.2} segs.",
        e.customer.id,
        e.customer.payment_method,
        e.customer.total_time
    ); */
}

fn simulation(steps: i32) {
    let mut event_queue = EventQueue::new();
    let mut sim_time = 0.0;
    let mut customer_count = 0;

    let mut fuel_stations = [0, 0, 0, 0];
    let mut customer_queues: Vec<Vec<Customer>> = vec![Vec::new(); 4];

    let initial_event = Event::new(0, Customer::new(0, 0.0), 0.0, None);
    event_queue.add(initial_event);
    println!("{:<8} | {:<10} | {:<8} | {:<4} | {:<10}", "TIEMPO", "EVENTO", "CLIENTE", "COLA", "ESTADO COLA");

    let sec = timeit_loops!(1, {
        //for _i in 0..steps {
        while sim_time < 43200.0 {
            match time_routine(&mut event_queue, &mut sim_time) {
                Some(mut e) => {
                    match e.id {
                        0 => {
                            arrive_routine(&mut event_queue, &mut sim_time, &mut e, &mut customer_count);
                        }
                        1 => {
                            queue_routine(&mut event_queue, &mut sim_time, &mut e, &mut fuel_stations, &mut customer_queues);
                        }
                        2 => {
                            refuel_routine(&mut event_queue, &mut sim_time, &mut e);
                        }
                        3 => {
                            payment_routine(&mut event_queue, &mut sim_time, &mut e);
                        }
                        4 => {
                            departure_routine(&mut event_queue, &mut sim_time, &mut e, &mut fuel_stations, &mut customer_queues);
                        }
                        _ => {
                            todo!();
                        }
                    }
                    println!("{:<8} | {}", e.pretty_print(), format_customer_queues(&customer_queues));
                    
                },
                None => {
                    todo!();
                }
            }
        }
    });
    println!("Simulación terminada en {} segs.", sec);
}

fn main() {

    let arg = env::args().nth(1);
    let arg_steps = if let Some(arg) = arg {
        if let Ok(arg_as_int) = arg.parse::<i32>() {
            arg_as_int
        } else {
            100
        }   
    } else {
        100
    };
    simulation(arg_steps);


}