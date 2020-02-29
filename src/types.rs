use amiquip::Delivery;

pub type Handler = Box<Fn(&Delivery) -> Result<(), String>>;
