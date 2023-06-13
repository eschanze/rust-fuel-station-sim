use crate::Event;
use crate::EventQueue;
use crate::{Customer, PaymentMethod};

use rand::seq::SliceRandom;
use rand::Rng;
use rand_distr::Exp;
use rand_distr::{Normal, Distribution};
use std::collections::HashMap;
//use std::env;

#[allow(unused_macros)]
#[macro_export]
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

pub fn format_customer_queues(customer_queues: &[Vec<Customer>]) -> String {
    let queue_string: Vec<String> = customer_queues
        .iter()
        .map(|queue| {
            let ids: Vec<String> = queue
                .iter()
                .map(|customer| customer.id.to_string())
                .collect();
            format!("[ {} ]", ids.join(" "))
        })
        .collect();
    format!("[{}]", queue_string.join(" "))
}

pub fn any_available(arr: &[i64]) -> (bool, Option<u64>) {
    let zero_indexes: Vec<u64> = arr
        .iter()
        .enumerate()
        .filter_map(
            |(index, &value)| {
                if value == 0 {
                    Some(index as u64)
                } else {
                    None
                }
            },
        )
        .collect();

    let has_zero = !zero_indexes.is_empty();
    let random_index = zero_indexes.choose(&mut rand::thread_rng()).copied();

    (has_zero, random_index)
}

pub fn get_shortest_or_random_index(customer_queues: &[Vec<Customer>]) -> usize {
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

pub fn process_customer_queues(
    customer_queues: &mut [Vec<Customer>],
    index: usize,
) -> Option<Customer> {
    if !customer_queues.is_empty() {
        if let Some(customer) = customer_queues[index].first().cloned() {
            customer_queues[index].remove(0);
            return Some(customer);
        }
    }
    None
}

fn normalize(minutes: u16) -> f64 {
    let range_min = 0;
    let range_max = 1440;
    let target_min = 0.0;
    let target_max = 1.0;
    
    let clamped_minutes = minutes % (range_max + 1); // Wrap around the value if it exceeds range_max
    
    let normalized = (clamped_minutes - range_min) as f64 / (range_max - range_min) as f64;
    
    // Adjust the normalized value to match the desired range
    let adjusted_normalized = (normalized - target_min) / (target_max - target_min);
    
    adjusted_normalized
}

fn beta_distr(x: f64) -> f64 {
    let numerator = x.powf(7.0) * (1.0 - x).powf(5.075);
    let denominator = 0.0000908345394559;
    numerator / denominator + 4.0
}

fn update_value(
    customer_data: &mut HashMap<u64, (u8, f64, f64, f64)>,
    id: u64,
    index: usize,
    value: f64,
) -> Result<(), String> {
    if let Some(tuple) = customer_data.get_mut(&id) {
        match index {
            0 => tuple.0 = value as u8,
            1 => tuple.1 = value,
            2 => tuple.2 = value,
            3 => tuple.3 = value,
            _ => {
                return Err(String::from("Invalid index"));
            }
        }
    } else {
        return Err(String::from("ID not found in the HashMap"));
    }
    
    Ok(())
}

fn get_value(
    customer_data: &HashMap<u64, (u8, f64, f64, f64)>,
    id: u64,
    index: usize,
) -> Result<f64, String> {
    if let Some(tuple) = customer_data.get(&id) {
        match index {
            0 => Ok(tuple.0 as f64),
            1 => Ok(tuple.1),
            2 => Ok(tuple.2),
            3 => Ok(tuple.3),
            _ => Err(String::from("Invalid index")),
        }
    } else {
        Err(String::from("ID not found in the HashMap"))
    }
}

// Desc: Routines for the simulation used in main.rs
pub fn time_routine(event_queue: &mut EventQueue, clock: &mut f64) -> Option<Event> {
    if !event_queue.q.is_empty() {
        let event = event_queue.q.remove(0);
        *clock = event.scheduled_time as f64;
        Some(event)
    } else {
        None
    }
}

pub fn arrive_routine(
    event_queue: &mut EventQueue,
    sim_time: &mut f64,
    e: &mut Event,
    customer_count: &mut u64,
    customer_data: &mut HashMap<u64, (u8, f64, f64, f64)>
) {
    *customer_count += 1;
    let arrival_rate = beta_distr(normalize(*sim_time as u16));
    let exp_distr = Exp::new(10.0 / arrival_rate).unwrap();
    let next_arrival_time = rand::thread_rng().sample(exp_distr);

    println!("Current arrival rate {} -> {}", *sim_time, arrival_rate);
    println!("Next arrival in {}", next_arrival_time);

    let queue_event = Event::new(1, e.customer.clone(), *sim_time, None);

    match e.customer.payment_method {
        PaymentMethod::Efectivo => customer_data.insert(e.customer.id, (0, *sim_time, 0.0, 0.0)),
        PaymentMethod::Tarjeta => customer_data.insert(e.customer.id, (1, *sim_time, 0.0, 0.0)),
        PaymentMethod::CopecApp => customer_data.insert(e.customer.id, (2, *sim_time, 0.0, 0.0))
    };

    event_queue.add(queue_event);

    let new_customer = Customer::new(*customer_count, *sim_time + next_arrival_time);
    let new_event = Event::new(0, new_customer, *sim_time + next_arrival_time, None);
    event_queue.add(new_event);
}

pub fn queue_routine(
    event_queue: &mut EventQueue,
    sim_time: &mut f64,
    e: &mut Event,
    fuel_stations: &mut [i64],
    customer_queues: &mut Vec<Vec<Customer>>,
    customer_data: &mut HashMap<u64, (u8, f64, f64, f64)>
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
        let _ = update_value(customer_data, e.customer.id, 2, *sim_time);
    }
}

pub fn refuel_routine(event_queue: &mut EventQueue, sim_time: &mut f64, e: &mut Event) {
    let normal = Normal::new(2.0, 0.15).unwrap();
    let refuel_time: f64 = normal.sample(&mut rand::thread_rng());
    let payment_event = Event::new(
        3,
        e.customer.clone(),
        *sim_time + refuel_time,
        e.chosen_queue,
    );
    event_queue.add(payment_event);
}

pub fn payment_routine(event_queue: &mut EventQueue, sim_time: &mut f64, e: &mut Event) {
    let payment_time: f64;
    match e.customer.payment_method {
        PaymentMethod::Efectivo => payment_time = Normal::new(0.875, 0.1).unwrap().sample(&mut rand::thread_rng()),
        PaymentMethod::Tarjeta => payment_time = Normal::new(0.425, 0.075).unwrap().sample(&mut rand::thread_rng()),
        PaymentMethod::CopecApp => payment_time = Normal::new(0.275, 0.055).unwrap().sample(&mut rand::thread_rng()),
    }

    let departure_event = Event::new(
        4,
        e.customer.clone(),
        *sim_time + payment_time,
        e.chosen_queue,
    );
    event_queue.add(departure_event);
}

pub fn departure_routine(
    event_queue: &mut EventQueue,
    sim_time: &mut f64,
    e: &mut Event,
    fuel_stations: &mut [i64],
    customer_queues: &mut Vec<Vec<Customer>>,
    customer_data: &mut HashMap<u64, (u8, f64, f64, f64)>
) {
    e.customer.total_time = *sim_time - e.customer.arrive_time;
    let _ = update_value(customer_data, e.customer.id, 3, *sim_time - e.customer.arrive_time);
    if let Some(queue) = e.chosen_queue {
        fuel_stations[queue as usize] = 0;
        if let Some(customer) = process_customer_queues(customer_queues, queue as usize) {
            let refuel_event = Event::new(2, customer.clone(), *sim_time, Some(queue));
            event_queue.add(refuel_event);
            let _ = update_value(customer_data, customer.id, 2, *sim_time - customer.arrive_time);
        }
    }
    /* println!(
        "Customer {} que pagó con {:?} terminó después de {:.2} segs.",
        e.customer.id,
        e.customer.payment_method,
        e.customer.total_time
    ); */
}
