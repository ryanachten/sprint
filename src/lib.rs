use config::{get_input_config, SprintConfig};

mod config;

struct SprintCapacity {
    proposed_sprint_points: f32,
    sprint_capacity: f32,
}

pub fn run() {
    let config = get_input_config();
    let capacity = get_capacity(config);

    println!(
        "team proposed_sprint_points: {}. sprint_capacity: {}%",
        capacity.proposed_sprint_points, capacity.sprint_capacity
    )
}

fn get_capacity(config: SprintConfig) -> SprintCapacity {
    let total_days = config.days_per_sprint * config.team_members as f32;
    let capacity = 1.0 - config.days_of_leave / total_days;
    let proposed_sprint_points = capacity * config.total_sprint_points;
    let sprint_capacity = capacity * 100.0;

    return SprintCapacity {
        proposed_sprint_points,
        sprint_capacity,
    };
}