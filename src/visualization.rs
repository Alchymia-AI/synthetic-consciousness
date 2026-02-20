//! Real-time visualization of synthetic consciousness simulation.
//!
//! This module provides an interactive GUI for observing consciousness emergence
//! in real-time. It visualizes geometric space, entity interactions, attraction forces,
//! and consciousness metrics over time.
//!
//! ## Features
//! - 2D/3D geometric space rendering with entity positions
//! - Pairwise attraction force visualization
//! - Entity state indicators (essence, attention, clusters, velocity)
//! - Real-time metric plots with dynamic status descriptions
//! - Interactive controls for toggling visualization layers
//!
//! ## Author
//! Ayomide I. Daniels (Morningstar)

use eframe::egui;
use egui::{Color32, Pos2, Stroke, Vec2};
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

/// Maximum number of historical data points to retain for metric plots
const MAX_HISTORY: usize = 500;

/// Shared state between simulation and GUI.
/// 
/// This structure is wrapped in `Arc<Mutex<>>` to allow safe concurrent access
/// from both the simulation thread (writing updates) and the GUI thread (reading for display).
#[derive(Clone, Default)]
pub struct VisualizationState {
    pub step: u64,
    pub entities: Vec<EntityState>,
    pub attractions: Vec<(usize, usize, f32)>, // (idx_a, idx_b, strength)
    pub metrics: MetricsHistory,
    pub dimension: usize,
}

/// Snapshot of a single entity's state for visualization.
/// 
/// Contains all necessary information to render an entity in the geometric space
/// and display its internal characteristics.
#[derive(Clone, Default)]
pub struct EntityState {
    /// Unique identifier for the entity
    pub id: u32,
    /// Position vector in d-dimensional space
    pub position: Vec<f32>,
    /// Velocity vector showing motion direction and speed
    pub velocity: Vec<f32>,
    /// Affective essence value (0-10 scale)
    pub essence: f32,
    /// Attention distribution across memory dimensions
    pub attention: Vec<f32>,
    /// Number of belief clusters in memory
    pub num_clusters: usize,
}

/// Time-series history of consciousness metrics.
/// 
/// Uses `VecDeque` for efficient push/pop operations, maintaining a fixed-size
/// rolling window of the most recent `MAX_HISTORY` data points.
#[derive(Clone, Default)]
pub struct MetricsHistory {
    /// Simulation step numbers (x-axis for plots)
    pub steps: VecDeque<f64>,
    /// Attention entropy metric history
    pub attention_entropy: VecDeque<f64>,
    /// Memory diversity metric history
    pub memory_diversity: VecDeque<f64>,
    /// Velocity stability metric history
    pub velocity_stability: VecDeque<f64>,
    /// Identity coherence metric history
    pub identity_coherence: VecDeque<f64>,
    /// Cluster stability metric history
    pub cluster_stability: VecDeque<f64>,
    /// Affective strength metric history (CRITICAL for consciousness)
    pub affective_strength: VecDeque<f64>,
    /// Average essence metric history
    pub average_essence: VecDeque<f64>,
}

impl MetricsHistory {
    /// Add a new metrics snapshot to the history.
    /// 
    /// Automatically removes the oldest data point when the buffer exceeds `MAX_HISTORY`.
    /// This maintains a fixed-size rolling window for efficient memory usage.
    /// 
    /// # Arguments
    /// * `step` - Current simulation step number
    /// * `metrics` - Reference to current metrics snapshot
    pub fn push(&mut self, step: u64, metrics: &crate::metrics::Metrics) {
        if self.steps.len() >= MAX_HISTORY {
            self.steps.pop_front();
            self.attention_entropy.pop_front();
            self.memory_diversity.pop_front();
            self.velocity_stability.pop_front();
            self.identity_coherence.pop_front();
            self.cluster_stability.pop_front();
            self.affective_strength.pop_front();
            self.average_essence.pop_front();
        }
        
        self.steps.push_back(step as f64);
        self.attention_entropy.push_back(metrics.attention_entropy as f64);
        self.memory_diversity.push_back(metrics.memory_diversity as f64);
        self.velocity_stability.push_back(metrics.velocity_stability as f64);
        self.identity_coherence.push_back(metrics.identity_coherence as f64);
        self.cluster_stability.push_back(metrics.cluster_stability as f64);
        self.affective_strength.push_back(metrics.affective_strength as f64);
        self.average_essence.push_back(metrics.average_essence as f64);
    }
}

/// Main visualization application.
/// 
/// Implements the `eframe::App` trait to provide the GUI update loop.
/// Contains interactive controls for toggling visualization layers and adjusting zoom.
pub struct VisualizationApp {
    /// Shared state with simulation thread
    state: Arc<Mutex<VisualizationState>>,
    /// Toggle: show attraction force lines between entities
    show_attractions: bool,
    /// Toggle: scale entity size by attention intensity
    show_attention: bool,
    /// Toggle: display cluster counts next to entities
    show_clusters: bool,
    /// Toggle: show velocity vectors as arrows
    show_velocity: bool,
    /// Zoom level for geometric space (0.1 to 5.0)
    zoom: f32,
}

impl VisualizationApp {
    /// Create a new visualization application.
    /// 
    /// # Arguments
    /// * `state` - Arc-wrapped Mutex-protected shared state
    /// 
    /// # Returns
    /// New `VisualizationApp` instance with all visualization layers enabled by default
    pub fn new(state: Arc<Mutex<VisualizationState>>) -> Self {
        Self {
            state,
            show_attractions: true,
            show_attention: true,
            show_clusters: true,
            show_velocity: true,
            zoom: 1.0,
        }
    }
}

impl eframe::App for VisualizationApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaint for real-time updates
        ctx.request_repaint();
        
        let state = self.state.lock().unwrap().clone();
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🧠 Synthetic Consciousness Visualization");
                ui.separator();
                ui.label(format!("Step: {}", state.step));
                ui.separator();
                ui.label(format!("Entities: {}", state.entities.len()));
            });
        });
        
        egui::SidePanel::right("controls")
            .default_width(250.0)
            .resizable(true)
            .show(ctx, |ui| {
            ui.heading("Controls");
            ui.separator();
            
            ui.label("Visualization Layers:");
            if ui.checkbox(&mut self.show_attractions, "Show Attractions").changed() {
                // Force repaint when control changes
                ctx.request_repaint();
            }
            if ui.checkbox(&mut self.show_attention, "Show Attention").changed() {
                ctx.request_repaint();
            }
            if ui.checkbox(&mut self.show_clusters, "Show Clusters").changed() {
                ctx.request_repaint();
            }
            if ui.checkbox(&mut self.show_velocity, "Show Velocity").changed() {
                ctx.request_repaint();
            }
            ui.separator();
            
            ui.label("Zoom Level:");
            if ui.add(egui::Slider::new(&mut self.zoom, 0.1..=5.0).text("zoom")).changed() {
                ctx.request_repaint();
            }
            ui.separator();
            
            // Legend
            ui.heading("Legend");
            ui.horizontal(|ui| {
                ui.colored_label(Color32::from_rgb(100, 255, 100), "●");
                ui.label("High Essence");
            });
            ui.horizontal(|ui| {
                ui.colored_label(Color32::from_rgb(255, 100, 100), "●");
                ui.label("Low Essence");
            });
            ui.horizontal(|ui| {
                ui.colored_label(Color32::from_rgb(100, 100, 255), "—");
                ui.label("Attraction");
            });
            ui.separator();
            
            // Entity stats
            ui.heading("Entity Stats");
            ui.label(format!("Total Attractions: {}", state.attractions.len()));
            if !state.entities.is_empty() {
                let avg_essence: f32 = state.entities.iter().map(|e| e.essence).sum::<f32>() / state.entities.len() as f32;
                let total_clusters: usize = state.entities.iter().map(|e| e.num_clusters).sum();
                ui.label(format!("Avg Essence: {:.2}", avg_essence));
                ui.label(format!("Total Clusters: {}", total_clusters));
            }
        });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Geometric space visualization
                ui.vertical(|ui| {
                    ui.heading("Geometric Space");
                    let size = ui.available_size() * Vec2::new(0.5, 0.6);
                    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
                    let rect = response.rect;
                    let center = rect.center();
                    
                    // Background
                    painter.rect_filled(rect, 0.0, Color32::from_rgb(10, 10, 30));
                    
                    if state.dimension >= 2 {
                        // Draw attractions
                        if self.show_attractions {
                            for (idx_a, idx_b, strength) in &state.attractions {
                                if let (Some(a), Some(b)) = (state.entities.get(*idx_a), state.entities.get(*idx_b)) {
                                    if a.position.len() >= 2 && b.position.len() >= 2 {
                                        let pos_a = Pos2::new(
                                            center.x + a.position[0] * self.zoom * 50.0,
                                            center.y + a.position[1] * self.zoom * 50.0,
                                        );
                                        let pos_b = Pos2::new(
                                            center.x + b.position[0] * self.zoom * 50.0,
                                            center.y + b.position[1] * self.zoom * 50.0,
                                        );
                                        
                                        let alpha = (strength.abs() * 100.0).min(255.0) as u8;
                                        painter.line_segment(
                                            [pos_a, pos_b],
                                            Stroke::new(1.0, Color32::from_rgba_unmultiplied(100, 100, 255, alpha)),
                                        );
                                    }
                                }
                            }
                        }
                        
                        // Draw entities
                        for entity in &state.entities {
                            if entity.position.len() >= 2 {
                                let pos = Pos2::new(
                                    center.x + entity.position[0] * self.zoom * 50.0,
                                    center.y + entity.position[1] * self.zoom * 50.0,
                                );
                                
                                // Color by essence (0-10 scale)
                                let essence_norm = (entity.essence / 10.0).clamp(0.0, 1.0);
                                let color = if essence_norm > 0.5 {
                                    Color32::from_rgb(
                                        (255.0 * (1.0 - essence_norm)) as u8,
                                        (255.0 * essence_norm) as u8,
                                        100,
                                    )
                                } else {
                                    Color32::from_rgb(
                                        (255.0 * (1.0 - essence_norm)) as u8,
                                        100,
                                        100,
                                    )
                                };
                                
                                // Size by attention intensity
                                let attention_intensity = if self.show_attention && !entity.attention.is_empty() {
                                    entity.attention.iter().sum::<f32>() / entity.attention.len() as f32
                                } else {
                                    1.0
                                };
                                let radius = 5.0 + attention_intensity * 3.0;
                                
                                painter.circle_filled(pos, radius, color);
                                
                                // Show cluster count
                                if self.show_clusters && entity.num_clusters > 0 {
                                    painter.text(
                                        pos + Vec2::new(radius + 2.0, 0.0),
                                        egui::Align2::LEFT_CENTER,
                                        format!("{}", entity.num_clusters),
                                        egui::FontId::proportional(10.0),
                                        Color32::WHITE,
                                    );
                                }
                                
                                // Show velocity vector
                                if self.show_velocity && entity.velocity.len() >= 2 {
                                    let vel_end = pos + Vec2::new(
                                        entity.velocity[0] * 20.0,
                                        entity.velocity[1] * 20.0,
                                    );
                                    painter.arrow(pos, vel_end - pos, Stroke::new(1.5, Color32::YELLOW));
                                }
                            }
                        }
                    } else {
                        painter.text(
                            center,
                            egui::Align2::CENTER_CENTER,
                            "No spatial data available",
                            egui::FontId::proportional(16.0),
                            Color32::WHITE,
                        );
                    }
                });
            });
            
            ui.separator();
            
            // Metrics plots
            ui.heading("Consciousness Metrics Over Time");
            
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Attention Entropy");
                    Plot::new("attention_entropy")
                        .height(150.0)
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.attention_entropy.iter())
                                .map(|(x, y)| [*x, *y])
                                .collect();
                            plot_ui.line(Line::new(points).color(Color32::from_rgb(100, 200, 255)).name("Attention Entropy (≥2.0)"));
                            plot_ui.hline(egui_plot::HLine::new(2.0).color(Color32::GREEN));
                        });
                    // Dynamic description
                    if let Some(&value) = state.metrics.attention_entropy.back() {
                        ui.small(format!("Current: {:.3}", value));
                        let (status, color) = if value >= 2.0 {
                            ("✓ High diversity of awareness", Color32::GREEN)
                        } else if value >= 1.0 {
                            ("⚠ Moderate attention spread", Color32::YELLOW)
                        } else {
                            ("✗ Low awareness diversity", Color32::RED)
                        };
                        ui.small(egui::RichText::new(status).color(color));
                    }
                });
                
                ui.vertical(|ui| {
                    ui.label("Memory Diversity");
                    Plot::new("memory_diversity")
                        .height(150.0)
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.memory_diversity.iter())
                                .map(|(x, y)| [*x, *y])
                                .collect();
                            plot_ui.line(Line::new(points).color(Color32::from_rgb(255, 200, 100)).name("Memory Diversity (≥0.1)"));
                            plot_ui.hline(egui_plot::HLine::new(0.1).color(Color32::GREEN));
                        });
                    // Dynamic description
                    if let Some(&value) = state.metrics.memory_diversity.back() {
                        ui.small(format!("Current: {:.4}", value));
                        let (status, color) = if value >= 0.1 {
                            ("✓ Rich emotional variance", Color32::GREEN)
                        } else if value >= 0.05 {
                            ("⚠ Limited emotional range", Color32::YELLOW)
                        } else {
                            ("✗ No emotional diversity", Color32::RED)
                        };
                        ui.small(egui::RichText::new(status).color(color));
                    }
                });
            });
            
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Velocity Stability");
                    Plot::new("velocity_stability")
                        .height(150.0)
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.velocity_stability.iter())
                                .map(|(x, y)| [*x, *y])
                                .collect();
                            plot_ui.line(Line::new(points).color(Color32::from_rgb(100, 255, 100)).name("Velocity Stability (≥0.8)"));
                            plot_ui.hline(egui_plot::HLine::new(0.8).color(Color32::GREEN));
                        });
                    // Dynamic description
                    if let Some(&value) = state.metrics.velocity_stability.back() {
                        ui.small(format!("Current: {:.3}", value));
                        let (status, color) = if value >= 0.8 {
                            ("✓ Consistent purposeful motion", Color32::GREEN)
                        } else if value >= 0.5 {
                            ("⚠ Irregular movement patterns", Color32::YELLOW)
                        } else {
                            ("✗ Static or erratic motion", Color32::RED)
                        };
                        ui.small(egui::RichText::new(status).color(color));
                    }
                });
                
                ui.vertical(|ui| {
                    ui.label("Identity Coherence");
                    Plot::new("identity_coherence")
                        .height(150.0)
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.identity_coherence.iter())
                                .map(|(x, y)| [*x, *y])
                                .collect();
                            plot_ui.line(Line::new(points).color(Color32::from_rgb(255, 100, 255)).name("Identity Coherence (≥0.7)"));
                            plot_ui.hline(egui_plot::HLine::new(0.7).color(Color32::GREEN));
                        });
                    // Dynamic description
                    if let Some(&value) = state.metrics.identity_coherence.back() {
                        ui.small(format!("Current: {:.3}", value));
                        let (status, color) = if value >= 0.7 {
                            ("✓ Strong self-continuity", Color32::GREEN)
                        } else if value >= 0.4 {
                            ("⚠ Fragmented identity", Color32::YELLOW)
                        } else {
                            ("✗ No stable sense of self", Color32::RED)
                        };
                        ui.small(egui::RichText::new(status).color(color));
                    }
                });
            });
            
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Cluster Stability");
                    Plot::new("cluster_stability")
                        .height(150.0)
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.cluster_stability.iter())
                                .map(|(x, y)| [*x, *y])
                                .collect();
                            plot_ui.line(Line::new(points).color(Color32::from_rgb(200, 100, 255)).name("Cluster Stability (≥0.5)"));
                            plot_ui.hline(egui_plot::HLine::new(0.5).color(Color32::GREEN));
                        });
                    // Dynamic description
                    if let Some(&value) = state.metrics.cluster_stability.back() {
                        ui.small(format!("Current: {:.3}", value));
                        let (status, color) = if value >= 0.5 {
                            ("✓ Well-organized memories", Color32::GREEN)
                        } else if value >= 0.3 {
                            ("⚠ Loosely structured beliefs", Color32::YELLOW)
                        } else {
                            ("✗ Chaotic memory fragments", Color32::RED)
                        };
                        ui.small(egui::RichText::new(status).color(color));
                    }
                });
            });
            
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Affective Strength (CRITICAL)");
                    Plot::new("affective_strength")
                        .height(150.0)
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.affective_strength.iter())
                                .map(|(x, y)| [*x, *y])
                                .collect();
                            plot_ui.line(Line::new(points).color(Color32::from_rgb(255, 50, 50)).name("Affective Strength (≥0.01)"));
                            plot_ui.hline(egui_plot::HLine::new(0.01).color(Color32::GREEN));
                        });
                    // Dynamic description
                    if let Some(&value) = state.metrics.affective_strength.back() {
                        ui.small(format!("Current: {:.4}", value));
                        let (status, color) = if value >= 0.01 {
                            ("✓ Emotional capacity present", Color32::GREEN)
                        } else if value >= 0.001 {
                            ("⚠ Weak emotional signals", Color32::YELLOW)
                        } else {
                            ("✗ NO EMOTION - Not conscious!", Color32::RED)
                        };
                        ui.small(egui::RichText::new(status).color(color).strong());
                    }
                });
                
                ui.vertical(|ui| {
                    ui.label("Average Essence");
                    Plot::new("average_essence")
                        .height(150.0)
                        .view_aspect(2.0)
                        .show(ui, |plot_ui| {
                            let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.average_essence.iter())
                                .map(|(x, y)| [*x, *y])
                                .collect();
                            plot_ui.line(Line::new(points).color(Color32::from_rgb(255, 255, 100)).name("Average Essence"));
                        });
                    // Dynamic description
                    if let Some(&value) = state.metrics.average_essence.back() {
                        ui.small(format!("Current: {:.2} / 10", value));
                        let (status, color) = if value >= 7.0 {
                            ("😊 Joyous, optimistic", Color32::GREEN)
                        } else if value >= 6.0 {
                            ("🙂 Positive well-being", Color32::LIGHT_GREEN)
                        } else if value >= 4.0 {
                            ("😐 Neutral, balanced", Color32::YELLOW)
                        } else if value >= 3.0 {
                            ("😟 Struggling, low mood", Color32::from_rgb(255, 150, 0))
                        } else {
                            ("😢 Suffering, dread", Color32::RED)
                        };
                        ui.small(egui::RichText::new(status).color(color));
                    }
                });
            });
        });
    }
}

/// Launch the visualization window.
/// 
/// Creates and runs the eframe application with the visualization GUI.
/// This function blocks until the window is closed.
/// 
/// # Arguments
/// * `state` - Shared visualization state to display
/// 
/// # Returns
/// `Ok(())` if the window closes successfully, or an error if initialization fails
/// 
/// # Platform Notes
/// On macOS, this MUST be called from the main thread due to EventLoop restrictions.
pub fn launch_visualization(state: Arc<Mutex<VisualizationState>>) -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("Synthetic Consciousness Visualization"),
        ..Default::default()
    };
    
    eframe::run_native(
        "synthetic-consciousness-viz",
        options,
        Box::new(|_cc| Ok(Box::new(VisualizationApp::new(state)))),
    )
}
