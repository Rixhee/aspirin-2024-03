#[derive(Debug, PartialEq, Copy, Clone)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

#[derive(Debug, Copy, Clone)]
struct TrafficLightState {
    current_color: TrafficLightColor,
    last_transition_time_ms: u32,
}

/// Returns the next color of the traffic light
///
/// # Arguments
///
/// * `state`: TrafficLightState - The current state of the traffic light
///
/// # Returns
///
/// * `TrafficLightColor`: TrafficLightColor - The next color of the traffic light
fn get_next_color(state: TrafficLightState) -> TrafficLightColor {
    match state.current_color {
        TrafficLightColor::Red => TrafficLightColor::Green,
        TrafficLightColor::Yellow => TrafficLightColor::Red,
        TrafficLightColor::Green => TrafficLightColor::Yellow,
    }
}

/// Returns the next state of the traffic light
///
/// # Arguments
///
/// * `state`: TrafficLightState - The current state of the traffic light
/// * `current_time_ms`: u32 - The current time in milliseconds
/// * `pedestrian_walk_request`: bool - Whether a pedestrian walk request is made
///
/// # Returns
///
/// * `TrafficLightColor`: TrafficLightColor - The next state of the traffic light
fn get_next_state(
    state: TrafficLightState,
    current_time_ms: u32,
    pedestrian_walk_request: bool,
) -> TrafficLightColor {
    let time_gap: u32 = current_time_ms - state.last_transition_time_ms;
    if (state.current_color == TrafficLightColor::Yellow && time_gap > 5000)
        || state.current_color == TrafficLightColor::Red && time_gap > 25000
    {
        return get_next_color(state);
    } else if state.current_color == TrafficLightColor::Green {
        let threshold = if pedestrian_walk_request {
            20000
        } else {
            30000
        };
        if time_gap > threshold {
            return get_next_color(state);
        }
    }

    state.current_color
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::traffic_light::{
        get_next_color, get_next_state, TrafficLightColor, TrafficLightState,
    };

    #[test_case(TrafficLightColor::Green, TrafficLightColor::Yellow ; "green -> yellow")]
    #[test_case(TrafficLightColor::Yellow, TrafficLightColor::Red ; "yellow -> red")]
    #[test_case(TrafficLightColor::Red, TrafficLightColor::Green ; "red -> green")]
    fn test_get_next_color(start_color: TrafficLightColor, next_color: TrafficLightColor) {
        let state = TrafficLightState {
            current_color: start_color,
            last_transition_time_ms: 0,
        };

        assert_eq!(get_next_color(state), next_color);
    }

    #[test]
    fn test_get_next_state_no_pedestrians() {
        let mut state = TrafficLightState {
            current_color: TrafficLightColor::Green,
            last_transition_time_ms: 0,
        };

        assert_eq!(get_next_state(state, 0, false), TrafficLightColor::Green);
        assert_eq!(
            get_next_state(state, 19000, false),
            TrafficLightColor::Green
        );
        assert_eq!(
            get_next_state(state, 21000, false),
            TrafficLightColor::Green
        );
        assert_eq!(
            get_next_state(state, 29000, false),
            TrafficLightColor::Green
        );
        assert_eq!(
            get_next_state(state, 31000, false),
            TrafficLightColor::Yellow
        );

        state.current_color = TrafficLightColor::Yellow;
        state.last_transition_time_ms = 30000;

        assert_eq!(
            get_next_state(state, 30000, false),
            TrafficLightColor::Yellow
        );
        assert_eq!(
            get_next_state(state, 34000, false),
            TrafficLightColor::Yellow
        );
        assert_eq!(get_next_state(state, 36000, false), TrafficLightColor::Red);

        state.current_color = TrafficLightColor::Red;
        state.last_transition_time_ms = 35000;

        assert_eq!(get_next_state(state, 35000, false), TrafficLightColor::Red);
        assert_eq!(get_next_state(state, 59000, false), TrafficLightColor::Red);
        assert_eq!(
            get_next_state(state, 61000, false),
            TrafficLightColor::Green
        );
    }

    #[test]
    fn test_get_next_state_pedestrians() {
        let mut state = TrafficLightState {
            current_color: TrafficLightColor::Green,
            last_transition_time_ms: 0,
        };

        assert_eq!(get_next_state(state, 0, true), TrafficLightColor::Green);
        assert_eq!(get_next_state(state, 19000, true), TrafficLightColor::Green);
        assert_eq!(
            get_next_state(state, 21000, true),
            TrafficLightColor::Yellow
        );

        state.current_color = TrafficLightColor::Yellow;
        state.last_transition_time_ms = 20000;

        assert_eq!(
            get_next_state(state, 20000, true),
            TrafficLightColor::Yellow
        );
        assert_eq!(
            get_next_state(state, 24000, true),
            TrafficLightColor::Yellow
        );
        assert_eq!(get_next_state(state, 26000, true), TrafficLightColor::Red);

        state.current_color = TrafficLightColor::Red;
        state.last_transition_time_ms = 25000;

        assert_eq!(get_next_state(state, 25000, true), TrafficLightColor::Red);
        assert_eq!(get_next_state(state, 49000, true), TrafficLightColor::Red);
        assert_eq!(get_next_state(state, 51000, true), TrafficLightColor::Green);
    }
}
