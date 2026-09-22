# Flockism

A boids flocking simulation written in Rust using macroquad. A flock of fish moves around the screen following simple local rules, while a wandering shark hunts nearby fish and triggers an evasive flee response.

## The four rules

Each fish computes a steering force every frame from four rules, all applied within a local radius:

- **Separation**: steer away from nearby neighbors, weighted by inverse distance so closer neighbors push harder than distant ones.
- **Alignment**: steer toward the average velocity of nearby neighbors to match the flock's heading.
- **Cohesion**: steer toward the average position (center of mass) of nearby neighbors to stay with the group.
- **Flee**: steer strongly away from the shark when it enters the predator radius.

The four forces are summed each frame and applied to the fish's velocity, which is clamped to a max speed.

## Flash expansion / confusion effect

The flee rule uses a much higher force multiplier than the other three rules. When the shark gets close, fish inside the predator radius scatter outward abruptly instead of gradually adjusting course, producing a visible flash expansion outward from the flock. The school regroups under cohesion once the threat passes.

The UI overlay in the corner reports which rule currently dominates the flock's total steering force, using hysteresis so the label does not flicker between closely matched rules (a new rule must exceed the current one by a set margin before it takes over the display).

## Running it

```
cargo run --release
```

Release mode is recommended since the simulation recomputes neighbor interactions for every fish against every other fish each frame.

## Project structure

```
src/main.rs   all simulation, rendering, and window setup logic
Cargo.toml    package manifest and dependencies (macroquad)
```

Everything lives in a single file: fish and shark state, the per-frame flocking and flee force calculations, boundary wrapping, drawing, and the dominant-rule overlay.
