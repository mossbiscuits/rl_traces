use rand::seq::SliceRandom;
use rand::Rng;

use crate::model::{Transition, INITIAL_STATE, TARGET};
use std::fs::OpenOptions;
use std::io::Write;

struct Learning {
    reward: Vec<(Transition, f64)>,
    history: Vec<f64>,
}

fn initialize_learning(transitions: Vec<Transition>) -> Learning {
    let mut learning = Learning {
        reward: Vec::new(),
        history: Vec::new(),
    };

    for transition in transitions {
        let reward = if transition.in_dep_graph {
            10.0
        } else {
            1.0
        };
        learning.reward.push((transition, reward));
    }

    learning
}

fn transition_probability(transition: &Transition, current_state: &[u64], learning: &Learning) -> f64 {
    let valid_transitions: Vec<&Transition> = learning
        .reward
        .iter()
        .filter(|(transition, _)| {
            transition.decrement.iter().zip(current_state.iter()).all(|(t, c)| t <= c)
        })
        .map(|(transition, _)| transition)
        .collect();

    let mut total_outgoing_rate = 0.0;
    for t in valid_transitions.iter() {
        let mut transition_rate = transition.rate;
        for (i, &c) in current_state.iter().enumerate() {
            if c < transition.decrement[i] {
                return 0.0; // Invalid transition
            }
            if transition.decrement[i] > 0 {
                transition_rate = (t.rate + (c as f64) + transition_rate) / 2.0;
            }
        }
        total_outgoing_rate += transition_rate;
    }
    // println!("Total Outgoing Rate: {:.e}", total_outgoing_rate);

    let mut transition_rate = transition.rate;

    for (i, &c) in current_state.iter().enumerate() {
        if c < transition.decrement[i] {
            return 0.0; // Invalid transition
        }
        if transition.decrement[i] > 0 {
            transition_rate = ((transition.rate + (c as f64)) + transition_rate) / 2.0;
        }
    }
    // println!("Transition Rate: {:.e}", transition_rate);

    transition_rate / total_outgoing_rate
}

fn generate_trace(learning: &Learning) -> (Vec<Transition>, f64) {
    let target_state = TARGET.to_vec();
    let mut trace: Vec<Transition> = Vec::new();
    let mut trace_probability = 1.0;
    let mut current_state = INITIAL_STATE.to_vec();

    loop {
        for i in 0..target_state.len() {
            if target_state[i] != 0 && current_state[i] == target_state[i] {
                return (trace, trace_probability);
            }
        }
        
        let valid_transitions: Vec<&(Transition, f64)> = learning
            .reward
            .iter()
            .filter(|(transition, _)| {
            transition.decrement.iter().zip(&current_state).all(|(t, c)| t <= c)
            })
            .collect();

        if valid_transitions.is_empty() {
            return (trace, trace_probability); // No valid transitions, terminate the loop
        }

        let total_reward: f64 = valid_transitions.iter().map(|(_, reward)| *reward).sum();
        let mut cumulative_probability = 0.0;
        
        // println!("Total Outgoing Rate: {:.e}", total_outgoing_rate);
        let mut shuffled_transitions = valid_transitions.clone();
        let mut rng = rand::rng();
        shuffled_transitions.shuffle(&mut rng);

        for (transition, reward) in shuffled_transitions {
            cumulative_probability += reward / total_reward;
            let random_value: f64 = rng.random_range(0.0..0.9);
            if random_value <= cumulative_probability {
                trace.push((*transition).clone());
                let transition_probability = transition_probability(transition, &current_state, learning);
                // println!("Transition Rate: {:.e}", transition_rate);
                trace_probability *= transition_probability;
                current_state = current_state
                    .iter()
                    .zip(&transition.increment)
                    .map(|(c, inc)| c + inc)
                    .zip(&transition.decrement)
                    .map(|(c, dec)| c - dec)
                    .collect();
                    // println!("  {} {:.4e} {:.4e}", transition.name, transition_probability, trace_probability);
                    break;
            }
        }
    }
}

fn adjust_rewards(learning: &mut Learning, trace: &Vec<Transition>, trace_probability: f64) {
    let recent_history_weight: f64 = 0.8; // Weight for recent history
    let historical_average: f64 = if learning.history.len() < 5 {
        0.0
    } else {
        let recent_entries = learning.history.iter().rev().take((learning.history.len() / 3).min(5));
        let recent_average: f64 = recent_entries.clone().map(|x| x.log10().max(-300.0)).sum::<f64>() / recent_entries.count() as f64;
        recent_history_weight * recent_average + (1.0 - recent_history_weight) * trace_probability.log10().max(-300.0)
    };

    
    let improvement = if trace_probability == 0.0 {
        -100.0
    } else if historical_average == 0.0 {
        0.0
    } else {
        (trace_probability.log10().max(-100.0) / historical_average).max(0.0)
    };
    
    let trace_length_penalty = if learning.history.len() < 5 {
        0.0
    } else {
        let historical_trace_length: f64 = learning.history.iter().map(|_| trace.len() as f64).sum::<f64>() / learning.history.len() as f64;
        let penalty_factor = 0.005; // Adjust this factor to control the penalty strength
        penalty_factor * ((trace.len() as f64 - historical_trace_length) / historical_trace_length).max(0.0)
    };
    
    let improvement = improvement - trace_length_penalty;
    
    // println!("Trace Probability : {:.4e}", trace_probability);
    // println!("Trace Prob Log10  : {:.4e}", trace_probability.log10().max(-300.0));
    // println!("Historical Average: {:.4e}", historical_average);
    // println!("Improvement: {:.4e}", improvement);

    for (transition, reward) in &mut learning.reward {
        let occurrence_count = trace.iter().filter(|&t| t == transition).count() as f64;
        if occurrence_count > 0.0 {
            if improvement > 0.0 {
                *reward += (0.001 * improvement * (occurrence_count.powf(0.95)));
            } else {
                *reward += (0.001 * improvement * (occurrence_count.powf(0.85)));
            }
            // // keep giving preference to transitions in the dependency graph
            // if transition.in_dep_graph {
            //     *reward *= 10.0 * *reward;
            // }
        }
        if transition.in_dep_graph {
            *reward += 0.005;
        }
    }
    learning.history.push(trace_probability);

    // for (transition, reward) in &learning.reward {
    //     println!("  {}\t{:.4}", transition.name, reward);
    // }

    // Normalize rewards if they get too high
    let max_reward = 1000.0; // Define a threshold for maximum reward
    let rewards: Vec<f64> = learning.reward.iter().map(|(_, reward)| *reward).collect();
    let total_reward: f64 = rewards.iter().sum();

    if total_reward > max_reward {
        let mean_reward = total_reward / rewards.len() as f64;
        let variance: f64 = rewards.iter().map(|r| (r - mean_reward).powi(2)).sum::<f64>() / rewards.len() as f64;
        let stdev = variance.sqrt();

        let normalization_factor = max_reward / total_reward;
        for (_, reward) in &mut learning.reward {
            // Adjust rewards based on their deviation from the mean
            if *reward > mean_reward + 4.0*stdev {
                *reward = (*reward).powf(0.4); // Apply square root to reduce the impact of extreme values
            } else if *reward < mean_reward - 4.0*stdev {
                *reward = (*reward).powf(2.2); // Apply square to increase the impact of low values
            }
            if *reward > mean_reward + stdev {
                *reward *= normalization_factor * 0.5; // Penalize outliers more
            } else if *reward < mean_reward - stdev {
                *reward *= normalization_factor * 1.2; // Boost under-rewarded transitions
            } else {
                *reward *= normalization_factor; // Normalize normally
            }
        }
        // println!(
        //     "Rewards normalized with mean {:.4e}, stdev {:.4e}, and total under {:.4e}",
        //     mean_reward, stdev, max_reward
        // );
    }
}

pub fn make_traces(transitions: Vec<Transition>, traces: u64) {
    println!("Generating {} traces...", traces);
    let mut learning = initialize_learning(transitions.clone());
    println!("Learning initialized.");
    for i in 0..traces {
        let trace = generate_trace(&learning);
        // println!("{}", trace.0.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(" "));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("traces.txt")
            .unwrap();
        writeln!(file, "{}", trace.0.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(" ")).unwrap();
        if traces < 1000 || i % if traces < 1000 {100} else {500} == 0 {
            println!("Trace {:4} Probability: {:0.4e}", i, trace.1);
        }
        adjust_rewards(&mut learning, &trace.0, trace.1);
        // println!("Trace: {:?}", trace.0);
    }
    use plotters::prelude::*;

    let root = BitMapBackend::new("learning_history.png", (640, 480))
        .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Learning History", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..learning.history.len(), -330.0..0.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    if learning.history.len() > 1000 {
        chart
            .draw_series(LineSeries::new(
                learning.history.iter().enumerate().step_by((learning.history.len() / 100).max(1)).map(|(i, &v)| (i, v.log10())),
                &RED,
            ))
            .unwrap()
            .label("Log Trace Probability")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + traces as i32, y)], &RED));
    } else {
        chart
            .draw_series(LineSeries::new(
                learning.history.iter().enumerate().map(|(i, &v)| (i, v.log10())),
                &RED,
            ))
            .unwrap()
            .label("Log Trace Probability")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + traces as i32, y)], &RED));
    }

    println!("Learning history saved to learning_history.png");
    println!("Traces saved to traces.txt");
    println!("Learning completed.");

    println!("If all traces were unique, the cumultive probability is {:.4e}", learning.history.iter().map(|x| x).sum::<f64>());
    println!("If all traces were unique, the average probability is   {:.4e}", learning.history.iter().map(|x| x).sum::<f64>() / learning.history.len() as f64);
}