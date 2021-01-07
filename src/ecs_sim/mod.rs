pub mod antenna_receiver;
pub mod doppler_shift;
pub mod interaction_detection;
pub mod movement;
pub mod rcs;
pub mod reflection;
pub mod transmit_signal;
pub mod jamming; 

pub mod components;
pub use self::components::*;

pub struct JammingSystem;
pub struct AntennaReceiverSystem;
pub struct DopplerShiftSystem;
pub struct InteractionDetection;
pub struct Movement;
pub struct RCSSystem;
pub struct ReflectionSystem;
pub struct TransmitSignal;

// pub struct RCS;
// pub struct Antenna;
// pub struct Position;
// pub struct Velocity;
// pub struct TargetIllumniation;