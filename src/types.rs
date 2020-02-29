use amiquip::Delivery;

pub type Handler = Box<dyn Fn(&Delivery) -> Result<(), String>>;
