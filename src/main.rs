#[macro_use]
extern crate timeit;

mod customer;
mod event;
mod eventqueue;
mod routines;
use customer::{Customer, PaymentMethod};
use event::Event;
use eventqueue::EventQueue;
use routines::*;
use std::env;
// use plotly::common::Mode;
// use plotly::*;

fn simulation(_steps: i32) {
    let mut event_queue = EventQueue::new();
    let mut sim_time = 0.0;
    let mut customer_count = 0;

    let mut fuel_stations = [0, 0, 0, 0];
    let mut customer_queues: Vec<Vec<Customer>> = vec![Vec::new(); 4];

    let initial_event = Event::new(0, Customer::new(0, 0.0), 0.0, None);
    event_queue.add(initial_event);
    println!(
        "{:<8} | {:<10} | {:<8} | {:<4} | {:<10}",
        "TIEMPO", "EVENTO", "CLIENTE", "COLA", "ESTADO COLA"
    );

    let sec = timeit_loops!(1, {
        //for _i in 0..steps {
        while sim_time < 43200.0 {
            match time_routine(&mut event_queue, &mut sim_time) {
                Some(mut e) => {
                    match e.id {
                        0 => {
                            arrive_routine(
                                &mut event_queue,
                                &mut sim_time,
                                &mut e,
                                &mut customer_count,
                            );
                        }
                        1 => {
                            queue_routine(
                                &mut event_queue,
                                &mut sim_time,
                                &mut e,
                                &mut fuel_stations,
                                &mut customer_queues,
                            );
                        }
                        2 => {
                            refuel_routine(&mut event_queue, &mut sim_time, &mut e);
                        }
                        3 => {
                            payment_routine(&mut event_queue, &mut sim_time, &mut e);
                        }
                        4 => {
                            departure_routine(
                                &mut event_queue,
                                &mut sim_time,
                                &mut e,
                                &mut fuel_stations,
                                &mut customer_queues,
                            );
                        }
                        _ => {
                            todo!();
                        }
                    }
                    println!(
                        "{:<8} | {}",
                        e.pretty_print(),
                        format_customer_queues(&customer_queues)
                    );
                }
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
