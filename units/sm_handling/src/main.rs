// Overall, this code demonstrates a basic usage of the `sfsm` crate in Rust. It defines a state machine with two
// states (`WaitForLaunch` and `Launch`) and a transition from the initial state to the final state. The code also
// showcases the implementation of state behavior functions and transition actions, as well as logging and testing
// capabilities.

// This line imports all the necessary items from the `sfsm` crate.
use sfsm::*;

// These lines define two state structs: `Launch` and `WaitForLaunch`. These structs represent the states in the state
// machine.
#[derive(Debug)]
pub struct Launch {}
pub struct WaitForLaunch {}

// This macro invocation defines the whole state machine. It creates a state machine named `Rocket`,
// with an initial state of `WaitForLaunch`. The square brackets `[WaitForLaunch, Launch]` specify
// all the possible states in the state machine, and the square brackets below specify the
// transitions. In this case, there is a single transition from `WaitForLaunch` to `Launch`.

add_state_machine!(
    Rocket, // Name of the state machine
    WaitForLaunch, // Initial state of the state machine
    [WaitForLaunch, Launch], // Possible states in the state machine
    [
        WaitForLaunch => Launch, // Transitions
    ]
);

// These lines implement the `State` trait for the `WaitForLaunch` struct. The `entry`, `execute`, and `exit` functions
// are defined to customize the behavior of the state. The `entry` function is called when the state is entered, the
// `execute` function is called repeatedly while the state is active, and the `exit` function is called when the state
// is exited.

impl State for WaitForLaunch {
    fn entry(&mut self) {
        println!("WaitForLaunch: Entry")
    }
    fn execute(&mut self) {
        println!("WaitForLaunch: Execute");
    }
    fn exit(&mut self) {
        println!("WaitForLaunch: Exit");
    }
}

// These lines define the transition from `WaitForLaunch` to `Launch`. The `Into<Launch> for WaitForLaunch`
// implementation specifies that `WaitForLaunch` can be converted into `Launch`.
impl Into<Launch> for WaitForLaunch {
    fn into(self) -> Launch {
        Launch {}
    }
}

// The `Transition<Launch> for WaitForLaunch` implementation defines the action and guard functions for the transition.
impl Transition<Launch> for WaitForLaunch {
    // The `action` function is called during the transition
    fn action(&mut self) {
        println!("WaitForLaunch => Launch: Exit");
    }

    // The `guard` function determines whether the transition should occur.
    fn guard(&self) -> TransitGuard {
        println!("WaitForLaunch => Launch: Guard");
        return TransitGuard::Transit;
    }
}

// These lines implement the `State` trait for the `Launch` struct. The `entry`, `execute`, and `exit` functions are
// defined to customize the behavior of the state, similar to the implementation for `WaitForLaunch`.

impl State for Launch {
    fn entry(&mut self) {
        println!("Launch: Entry")
    }
    fn execute(&mut self) {
        println!("Launch: Execute");
    }
    fn exit(&mut self) {
        println!("Launch: Exit");
    }
}

// This line defines a logger function decorated with the `sfsm_trace` attribute. It enables the tracing feature and
// allows the state machine to send logs to the function
#[sfsm_trace]
fn trace(log: &str) {
    println!("{}", log);
}

// These lines define the `run_basic_example` function, which demonstrates the usage of the state machine. Inside the
// function, a `Rocket` state machine is instantiated. The initial state is set to `WaitForLaunch`, and the state
// machine is started. Then, the current state is checked using the `IsState` trait, and the `rocket.step()` function
// is called to advance the state machine. Afterward, the state machine is stopped, and the stopped state is accessed
// and printed.
fn run_basic_example() -> Result<(), SfsmError> {
    // State machine instantiation and start
    let mut rocket = Rocket::new();
    let wait_for_launch = WaitForLaunch {};
    rocket.start(wait_for_launch)?;

    // Checking the state and running the state machine
    assert!(IsState::<WaitForLaunch>::is_state(&rocket));
    rocket.step()?;

    assert!(IsState::<Launch>::is_state(&rocket));

    // Stopping the state machine and accessing the stopped state
    let stopped_state = rocket.stop()?;
    match stopped_state {
        match_state_entry!(Rocket, Launch, exit_state) => {
            println!("Exit state: {:?}", exit_state);
            assert!(true);
        }
        _ => {
            assert!(false);
        }
    }

    Ok(())
}

fn main() {
    run_basic_example().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::run_basic_example;

    #[test]
    fn basic_example() {
        run_basic_example().unwrap();
    }
}

// These lines define a test module with a single test function `basic_example`. The test function calls the
// `run_basic_example` function and asserts that it completes successfully.
