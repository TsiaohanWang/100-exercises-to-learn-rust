pub mod loss;
pub mod model;

pub trait LogisRegModel {
    type Weight;
    type Bias;
    fn display(&self) ;
}