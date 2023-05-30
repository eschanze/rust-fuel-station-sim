use rand::Rng;
use std::fmt;

#[derive(Clone)]
pub struct Customer {
    pub id: u64,
    pub arrive_time: f64,
    pub total_time: f64,
    pub payment_method: PaymentMethod,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PaymentMethod {
    Efectivo,
    Tarjeta,
    CopecApp
}

impl fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            PaymentMethod::Efectivo => "EFECTIVO",
            PaymentMethod::Tarjeta => "TARJETA",
            PaymentMethod::CopecApp => "APP"
        };
        write!(f, "{}", string_representation.to_uppercase())
    }
}

impl Customer {
    pub fn new(id: u64, arrive_time: f64) -> Self {
        let payment_methods = [
            PaymentMethod::Efectivo,
            PaymentMethod::Tarjeta,
            PaymentMethod::CopecApp
        ];

        let rng = rand::thread_rng().gen_range(0..payment_methods.len());
        let payment_method = payment_methods[rng].clone();

        Customer {
            id,
            arrive_time,
            total_time: 0.0,
            payment_method,
        }
    }
}