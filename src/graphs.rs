use plotly::{common::Mode, Plot, Scatter};
use std::collections::HashMap;

pub fn payment_method_avg_time(customer_data: &mut HashMap<u64, (u8, f64, f64, f64, u8)>) {
    let mut time_values_0: Vec<f64> = Vec::new();
    let mut avg_values_0: Vec<f64> = Vec::new();

    let mut time_values_1: Vec<f64> = Vec::new();
    let mut avg_values_1: Vec<f64> = Vec::new();

    let mut time_values_2: Vec<f64> = Vec::new();
    let mut avg_values_2: Vec<f64> = Vec::new();

    let mut sum_0: f64 = 0.0;
    let mut count_0: f64 = 0.0;

    let mut sum_1: f64 = 0.0;
    let mut count_1: f64 = 0.0;

    let mut sum_2: f64 = 0.0;
    let mut count_2: f64 = 0.0;

    for (_, &(payment_method, _, _, time_total, departure_key)) in customer_data.iter() {
        if departure_key == 0 {
            continue;
        }
        match payment_method {
            0 => {
                sum_0 += time_total;
                count_0 += 1.0;

                time_values_0.push(count_0);
                avg_values_0.push(sum_0 / count_0);
            }
            1 => {
                sum_1 += time_total;
                count_1 += 1.0;

                time_values_1.push(count_1);
                avg_values_1.push(sum_1 / count_1);
            }
            2 => {
                sum_2 += time_total;
                count_2 += 1.0;

                time_values_2.push(count_2);
                avg_values_2.push(sum_2 / count_2);
            }
            _ => {}
        }
    }

    let trace_0 = Scatter::new(time_values_0, avg_values_0)
        .mode(Mode::Lines)
        .name("Efectivo");
    let trace_1 = Scatter::new(time_values_1, avg_values_1)
        .mode(Mode::Lines)
        .name("Tarjeta");
    let trace_2 = Scatter::new(time_values_2, avg_values_2)
        .mode(Mode::Lines)
        .name("App");

    let mut plot = Plot::new();
    plot.add_trace(trace_0);
    plot.add_trace(trace_1);
    plot.add_trace(trace_2);

    plot.write_html("promedio_atencion_metodo_pago.html");
}

pub fn four_vs_five_stations(
    customer_data: &mut HashMap<u64, (u8, f64, f64, f64, u8)>,
    customer_data_5s: &mut HashMap<u64, (u8, f64, f64, f64, u8)>,
) {
    let mut time_values1: Vec<f64> = Vec::new();
    let mut avg_values1: Vec<f64> = Vec::new();

    let mut sum1: f64 = 0.0;
    let mut count1: f64 = 0.0;

    for (_, &(_, _, _, time_total, departure_key)) in customer_data.iter() {
        if departure_key == 0 {
            continue;
        }
        sum1 += time_total;
        count1 += 1.0;

        // eje x: total
        time_values1.push(count1);
        // eje y: promedio
        avg_values1.push(sum1 / count1);
    }

    let mut time_values2: Vec<f64> = Vec::new();
    let mut avg_values2: Vec<f64> = Vec::new();

    let mut sum2: f64 = 0.0;
    let mut count2: f64 = 0.0;

    // Iterate over the time steps in customer_data2 and update the sum and count
    for (_, &(_, _, _, time_total, departure_key)) in customer_data_5s.iter() {
        if departure_key == 0 {
            continue;
        }
        sum2 += time_total;
        count2 += 1.0;

        // eje x: total
        time_values2.push(count2);
        // eje y: promedio
        avg_values2.push(sum2 / count2);
    }

    let trace1 = Scatter::new(time_values1, avg_values1)
        .mode(Mode::Markers)
        .name("4 Estaciones");
    let trace2 = Scatter::new(time_values2, avg_values2)
        .mode(Mode::Markers)
        .name("5 Estaciones");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.write_html("cuatro_vs_cinco_estaciones.html");
}

pub fn queue_avg_waittime(customer_data: &mut HashMap<u64, (u8, f64, f64, f64, u8)>) {
    // Promedio tiempo de simulación vs tiempo promedio espera cola

    let mut time_values: Vec<i64> = Vec::new();
    let mut avg_values: Vec<f64> = Vec::new();

    let mut wait_time_sum: f64 = 0.0;
    let mut count: f64 = 0.0;

    for (_, &(_, time, wait_time, _, departure_key)) in customer_data.iter() {
        if departure_key == 0 {
            continue;
        }
        // if (departure_key == 1) {
        //     wait_time_sum += wait_time;
        //     count += 1.0;

        //     time_values.push(time);
        //     avg_values.push(wait_time_sum / count);
        // }
        wait_time_sum += wait_time;
        count += 1.0;

        time_values.push((time * 100.0) as i64);
        avg_values.push(wait_time_sum / count);
    }

    let trace = Scatter::new(time_values, avg_values).mode(Mode::Markers);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.write_html("promedio_cola_vs_dia.html");
}

pub fn payment_method_sensitivity(customer_data: &mut HashMap<u64, (u8, f64, f64, f64, u8)>) {
    let mut averages: HashMap<u8, f64> = HashMap::new();
    let mut counts: HashMap<u8, usize> = HashMap::new();

    // Iterate over the customer_data HashMap
    for (_, &(payment_method, _, _, last_element, departure_key)) in customer_data.iter() {
        if departure_key == 0 {
            continue;
        }
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
        match payment_method {
            0 => println!("Tiempo promedio con Pago Efectivo = {} mins.", average),
            1 => println!("Tiempo promedio con Pago Tarjeta  = {} mins.", average),
            2 => println!("Tiempo promedio con Pago CopecApp = {} mins.", average),
            _ => (),
        }
    }
}

pub fn lambda_avg(customer_data: &mut HashMap<u64, (u8, f64, f64, f64, u8)>) {}
