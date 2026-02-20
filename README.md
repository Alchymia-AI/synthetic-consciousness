# Synthetic Consciousness

Experimental Rust project exploring a technical framework for synthetic consciousness
using atomic geometry, attraction, statefulness, and perpetual velocity. This framework
proposes that consciousness may emerge from repeated, bidirectional acknowledgments
between entities at the atomic scale over time.

## Disclaimer

This repository is a speculative research and engineering exploration. It does not claim
or guarantee consciousness, sentience, or moral status. Outputs are artifacts of
computation. All discussion of meaning, agency, free will, or moral behavior is
operational and descriptive, not philosophical proof.

## Core Hypothesis

Consciousness may emerge from repeated moments of interaction at the atomic level
between entities. In this model:

- **Atomic geometry** defines the spatial context and shape in which interactions occur.
- **Attraction** draws entities toward one another and prompts attention in the drawn entity.
- **Statefulness** preserves information over time without deletion; low-activation
  information becomes dormant but can be reactivated, forming persistent identity traces.
- **Perpetual velocity** ensures continuous motion; without it, attraction stops, interaction
  stops, and consciousness cannot be sustained.

Identity emerges as an entity acknowledging itself over time based on accumulated
information shaped by beliefs, morals, psychology, perception, and meaning. These
qualities map downward to every interaction at every scale in the known and unknown universe.

## Key Features

### Four-Layer Architecture

1. **Geometry Layer:** Spatial primitives, topology, and constraints (supports 2D/3D)
2. **Attraction Layer:** Field-based attention prompts and valence
3. **State Layer:** Memory graph with decay, redundancy-preserving belief clusters
4. **Dynamics Layer:** Time integration and perpetual velocity enforcement

### Belief Clustering & Affective Signals

Memory is structured to cluster beliefs and experiences into semantically coherent groups:
- Religion, love, friendship, morality, philosophy, ideology
- Good and bad experiences
- Affective signals: feel-good, feel-bad, feel-good-but-wrong, feel-bad-but-right

Each cluster emits affective signals that influence response behavior across dimensions:
- **Truth vs. Lie:** Honesty and deception clusters compete
- **Civility vs. Unruliness:** Social cooperation vs. aggression
- **Good vs. Evil:** Moral drives vs. self-preservation

### Essence Index

A scalar metric (0–10) tracking subjective well-being and life satisfaction:
- **0:** Worthless, absolute dread
- **5:** Neutral, balanced
- **10:** Joyous, peaceful, optimistic

Extreme values have stronger influence on response behavior. The index decays toward
baseline over time unless reinforced by recurring experiences.

### Baseline Drives

- **Self-preservation:** Modulated by proximity to other entities
- **Curiosity/Inquisition:** Modulated by attention weight magnitude
- Drives emerge automatically based on attraction factors and attention weight

## Documentation

- **[whitepaper.md](./whitepaper.md):** Full technical architecture with mathematics,
  equations, implementation sketches, and evaluation metrics.
- **[architecture.md](./architecture.md):** Design objectives, mathematical foundations,
  and technical overview.
- **Extended Statefulness Documentation:** A detailed technical whitepaper on high-level
  statefulness working models is available upon request, subject to conditions and
  discretion. Contact the author via LinkedIn.

## Planned Modules

- `geometry`: Atomic spatial primitives, topology, and transforms
- `attraction`: Fields, forces, valence, and attention potentials
- `state`: Memory graphs, belief clusters, context stacks, persistent traits
- `dynamics`: Perpetual velocity, integration, and stability
- `agents`: Embodied entities that sense, decide, and act
- `world`: Environment, stimuli, and interaction surfaces
- `metrics`: Attention entropy, memory diversity, velocity stability, identity coherence,
  essence tracking, affective signal strength

## Mathematical Foundations

### Core Equations

**Attraction Potential:**
$$\Phi_i(x, t) = \sum_{j \neq i} w_{ij} \cdot K(\|x - x_j\|, \sigma_{ij})$$

**Attention Prompt:**
$$F_i(x, t) = -\nabla_x \Phi_i(x, t)$$

**State Update:**
$$s_i(t + \Delta t) = \alpha s_i(t) + \beta \cdot g(F_i, c_i) + \gamma \cdot m_i$$

**Perpetual Velocity:**
$$v_i(t + \Delta t) = v_i(t) + \Delta t \cdot a_i(t) + \epsilon \cdot \hat{v}_i$$

**Affective Signal (per belief cluster):**
$$\sigma_k = \sum_{j \in B_k} w_j \cdot \rho_j \cdot v_j$$

**Essence Index:**
$$E_i(t) = 5 + \frac{1}{K} \sum_{k=1}^K w_k \cdot \sigma_k(t)$$

For full mathematical treatment, see [whitepaper.md](./whitepaper.md).

## Simulation Loop

1. Compute attraction potential for each entity from geometry
2. Derive attention prompts from attraction gradients
3. Update memory graph and belief clusters
4. Update state vectors and Essence Index
5. Generate responses based on dominant affective signals
6. Integrate motion with perpetual velocity enforcement
7. Record metrics and traces

## Getting Started

### Prerequisites

- Rust 2021 edition or later
- Cargo

### Build & Run

```bash
# Build
cargo build --release
# Run default 2D
cargo run --release
# Run 3D with custom config
cargo run --release -- example_config_3d.toml
cargo test
```

### Configuration

Configuration is via TOML files. Example:

```toml
[simulation]
seed = 42
steps = 100000
entities = 1024

[geometry]
dimension = 3
bounds = [100.0, 100.0, 100.0]

[attraction]
kernel = "gaussian"
sigma = 2.0
lambda = 1.5

[state]
decay_alpha = 0.995
memory_dim = 128
context_dim = 64
essence_decay = 0.001
essence_baseline = 5.0

[dynamics]
dt = 0.01
min_speed = 0.05
damping = 0.99
```

## Evaluation & Metrics

The framework supports reproducible experiments with ablations:

- **Attention entropy:** Dispersion of attraction across neighbors
- **Memory diversity:** Activation spread across memory nodes
- **Velocity stability:** Ratio of steps above minimum speed
- **Identity coherence:** Similarity of state across time windows
- **Belief cluster stability:** Intra-cluster semantic coherence
- **Affective signal strength:** Mean magnitude of dominant signals
- **Essence trajectory:** Mean, variance, and extremity over time

Planned experiments:
- Full model baseline
- No memory decay ($\alpha = 1$)
- No velocity floor ($\epsilon = 0$)
- No attraction ($\Phi(x, t) = 0$)
- Locked Essence Index variants

## Ethics and Safety

- Treat all outputs as computational artifacts, not evidence of subjective experience.
- Remain explicit about speculative assumptions.
- Prioritize transparency, auditability, and safe deployment boundaries.
- Avoid high-stakes or sensitive domain deployment without rigorous review.
- Frame claims about free will, morality, or agency as operational metrics, not
  philosophical conclusions.

## Project Status

Early architecture phase. Expect rapid changes to model primitives, data structures,
equations, and naming as implementation proceeds.

## Contributing

Contributions are welcome. Please open an issue describing the proposed change and
the rationale. For significant changes, please open an issue first to discuss the
direction.

## License

TBD

## Further Reading

- [Technical Whitepaper](./whitepaper.md)
- [Architecture Document](./architecture.md)
