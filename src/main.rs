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

use ordered_float::OrderedFloat;
use std::{env, time};

use plotly::{
    color::{NamedColor, Rgb, Rgba},
    common::{
        ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode,
        Orientation, Title,
    },
    layout::{Axis, BarMode, Layout, Legend, TicksDirection, TraceOrder},
    sankey::{Line as SankeyLine, Link, Node},
    Bar, Plot, Sankey, Scatter, ScatterPolar,
};

fn simulation(_steps: i32, customer_data: &mut HashMap<u64, (u8, f64, f64, f64)>, fuel_station_length: usize) {
    let mut event_queue = EventQueue::new();
    let mut sim_time = 0.0;
    let mut customer_count = 0;

    let mut fuel_stations = vec![0; fuel_station_length];
    let mut customer_queues: Vec<Vec<Customer>> = vec![Vec::new(); fuel_station_length];

    let initial_event = Event::new(0, Customer::new(0, 0.0), 0.0, None);
    event_queue.add(initial_event);
    println!(
        "{:<8} | {:<10} | {:<8} | {:<4} | {:<10}",
        "TIEMPO", "EVENTO", "CLIENTE", "COLA", "ESTADO COLA"
    );

    let sec = timeit_loops!(1, {
        for _i in 0.._steps {
            match time_routine(&mut event_queue, &mut sim_time) {
                Some(mut e) => {
                    match e.id {
                        0 => {
                            arrive_routine(
                                &mut event_queue,
                                &mut sim_time,
                                &mut e,
                                &mut customer_count,
                                customer_data
                            );
                        }
                        1 => {
                            queue_routine(
                                &mut event_queue,
                                &mut sim_time,
                                &mut e,
                                &mut fuel_stations,
                                &mut customer_queues,
                                customer_data
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
                                customer_data
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

use std::collections::HashMap;
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
    // HashMap con la información para hacer los gráficos.
    // El format del HashMap es key: (int, float, float, float)
    // Esto es ID: (Método de pago, tiempo de llegada (dentro de la simulación), tiempo esperando en cola, tiempo total de atención al momento de salir)
    let mut customer_data: HashMap<u64, (u8, f64, f64, f64)> = HashMap::new();
    simulation(arg_steps, &mut customer_data, 4);
    /*if let Some(max) = customer_data.keys().max() {
        for i in 0..=*max {
            if let Some(tuple) = customer_data.get(&i) {
                println!("ID: {}, Value: {:?}", i, tuple);
            }
        }
    }*/

    /*let mut averages: HashMap<u8, f64> = HashMap::new();
    let mut counts: HashMap<u8, usize> = HashMap::new();
    
    // Iterate over the customer_data HashMap
    for (_, &(payment_method, _, _, last_element)) in customer_data.iter() {
        // Update the sum and count for the payment method
        let sum = averages.entry(payment_method).or_insert(0.0);
        let count = counts.entry(payment_method).or_insert(0);
    
        *sum += last_element;
        *count += 1;
    }
    
    // Print the averages for each payment method
    for (payment_method, sum) in averages.iter() {
        let count = *counts.get(payment_method).unwrap();
        let average = sum / (count as f64);
        println!("Payment Method {}: Average = {}", payment_method, average);
    } */

    let mut time_values: Vec<f64> = Vec::new();
    let mut avg_values: Vec<f64> = Vec::new();

    let mut wait_time_sum: f64 = 0.0;
    let mut count: f64 = 0.0;

    for (_, &(_, time, wait_time, _)) in customer_data.iter() {
        wait_time_sum += wait_time;
        count += 1.0;

        time_values.push(time);
        avg_values.push(wait_time_sum / count);
    }

    let trace = Scatter::new(time_values, avg_values).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.write_html("out.html");

}
