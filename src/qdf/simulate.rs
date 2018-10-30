use state::*;

/// Trait that tells QDF how to simulate states of space.
pub trait Simulate<S>
where
    S: State,
{
    /// Performs simulation of state based on neighbor states.
    ///
    /// # Arguments
    /// * `state` - current state.
    /// * `neighbor_states` - current neighbor states.
    fn simulate(state: &S, neighbor_states: &[&S]) -> S;
}

impl<S> Simulate<S> for ()
where
    S: State,
{
    fn simulate(state: &S, _: &[&S]) -> S {
        state.clone()
    }
}
