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
    pub bounds: Vec<f32>, // Spatial bounds from geometry config
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
    /// Affective strength metric history
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
    /// Toggle: show entity ID labels
    show_entity_labels: bool,
    /// Toggle: show grid lines for spatial reference
    show_grid: bool,
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
            show_entity_labels: true,
            show_grid: true,
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
                if !state.entities.is_empty() {
                    ui.separator();
                    let avg_essence: f32 = state.entities.iter().map(|e| e.essence).sum::<f32>() / state.entities.len() as f32;
                    ui.label(format!("Avg Essence: {:.1}", avg_essence));
                }
            });
        });
        
        egui::TopBottomPanel::bottom("legend_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Legend in compact horizontal format
                ui.label(egui::RichText::new("Legend:").strong());
                ui.separator();
                ui.colored_label(Color32::from_rgb(100, 255, 100), "●");
                ui.label("High");
                ui.colored_label(Color32::from_rgb(255, 100, 100), "●");
                ui.label("Low");
                ui.separator();
                ui.colored_label(Color32::from_rgb(100, 200, 255), "━");
                ui.label("Attraction");
                ui.colored_label(Color32::from_rgb(150, 220, 255), "◉");
                ui.label("Interact");
                ui.colored_label(Color32::from_rgb(255, 220, 0), "→");
                ui.label("Velocity");
                ui.separator();
                ui.label(format!("Dim: {}", state.dimension));
                ui.label(format!("Attractions: {}", state.attractions.len()));
                if !state.entities.is_empty() {
                    let total_clusters: usize = state.entities.iter().map(|e| e.num_clusters).sum();
                    ui.label(format!("Clusters: {}", total_clusters));
                }
            });
        });
        
        // Right panel for metrics (placed first to reserve space)
        egui::SidePanel::right("metrics_panel")
            .min_width(420.0)
            .default_width(450.0)
            .show(ctx, |ui| {
                ui.heading("Consciousness Metrics");
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let plot_height = 110.0;
                    let plot_width = ui.available_width();
                    
                    // 1. Attention Entropy
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Attention Entropy").strong());
                        Plot::new("attention_entropy")
                            .height(plot_height)
                            .width(plot_width)
                            .show_axes([false, true])
                            .show(ui, |plot_ui| {
                                let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.attention_entropy.iter())
                                    .map(|(x, y)| [*x, *y]).collect();
                                plot_ui.line(Line::new(points).color(Color32::from_rgb(100, 200, 255)));
                                plot_ui.hline(egui_plot::HLine::new(2.0).color(Color32::GREEN));
                            });
                        if let Some(&value) = state.metrics.attention_entropy.back() {
                            let (status, color) = if value >= 2.0 {
                                ("✓ High awareness diversity", Color32::GREEN)
                            } else if value >= 1.0 {
                                ("⚠ Moderate spread", Color32::YELLOW)
                            } else {
                                ("✗ Low diversity", Color32::RED)
                            };
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{:.3}", value)).size(14.0).strong());
                                ui.label(egui::RichText::new(status).color(color).size(13.0));
                            });
                        }
                    });
                    
                    // 2. Memory Diversity
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Memory Diversity").strong());
                        Plot::new("memory_diversity")
                            .height(plot_height)
                            .width(plot_width)
                            .show_axes([false, true])
                            .show(ui, |plot_ui| {
                                let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.memory_diversity.iter())
                                    .map(|(x, y)| [*x, *y]).collect();
                                plot_ui.line(Line::new(points).color(Color32::from_rgb(255, 200, 100)));
                                plot_ui.hline(egui_plot::HLine::new(0.1).color(Color32::GREEN));
                            });
                        if let Some(&value) = state.metrics.memory_diversity.back() {
                            let (status, color) = if value >= 0.1 {
                                ("✓ Rich variance", Color32::GREEN)
                            } else if value >= 0.05 {
                                ("⚠ Limited range", Color32::YELLOW)
                            } else {
                                ("✗ No diversity", Color32::RED)
                            };
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{:.4}", value)).size(14.0).strong());
                                ui.label(egui::RichText::new(status).color(color).size(13.0));
                            });
                        }
                    });
                    
                    // 3. Velocity Stability
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Velocity Stability").strong());
                        Plot::new("velocity_stability")
                            .height(plot_height)
                            .width(plot_width)
                            .show_axes([false, true])
                            .show(ui, |plot_ui| {
                                let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.velocity_stability.iter())
                                    .map(|(x, y)| [*x, *y]).collect();
                                plot_ui.line(Line::new(points).color(Color32::from_rgb(100, 255, 100)));
                                plot_ui.hline(egui_plot::HLine::new(0.8).color(Color32::GREEN));
                            });
                        if let Some(&value) = state.metrics.velocity_stability.back() {
                            let (status, color) = if value >= 0.8 {
                                ("✓ Purposeful motion", Color32::GREEN)
                            } else if value >= 0.5 {
                                ("⚠ Irregular", Color32::YELLOW)
                            } else {
                                ("✗ Erratic", Color32::RED)
                            };
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{:.3}", value)).size(14.0).strong());
                                ui.label(egui::RichText::new(status).color(color).size(13.0));
                            });
                        }
                    });
                    
                    // 4. Identity Coherence
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Identity Coherence").strong());
                        Plot::new("identity_coherence")
                            .height(plot_height)
                            .width(plot_width)
                            .show_axes([false, true])
                            .show(ui, |plot_ui| {
                                let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.identity_coherence.iter())
                                    .map(|(x, y)| [*x, *y]).collect();
                                plot_ui.line(Line::new(points).color(Color32::from_rgb(255, 100, 255)));
                                plot_ui.hline(egui_plot::HLine::new(0.7).color(Color32::GREEN));
                            });
                        if let Some(&value) = state.metrics.identity_coherence.back() {
                            let (status, color) = if value >= 0.7 {
                                ("✓ Strong self-continuity", Color32::GREEN)
                            } else if value >= 0.4 {
                                ("⚠ Fragmented", Color32::YELLOW)
                            } else {
                                ("✗ No stable self", Color32::RED)
                            };
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{:.3}", value)).size(14.0).strong());
                                ui.label(egui::RichText::new(status).color(color).size(13.0));
                            });
                        }
                    });
                    
                    // 5. Cluster Stability
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Cluster Stability").strong());
                        Plot::new("cluster_stability")
                            .height(plot_height)
                            .width(plot_width)
                            .show_axes([false, true])
                            .show(ui, |plot_ui| {
                                let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.cluster_stability.iter())
                                    .map(|(x, y)| [*x, *y]).collect();
                                plot_ui.line(Line::new(points).color(Color32::from_rgb(200, 100, 255)));
                                plot_ui.hline(egui_plot::HLine::new(0.5).color(Color32::GREEN));
                            });
                        if let Some(&value) = state.metrics.cluster_stability.back() {
                            let (status, color) = if value >= 0.5 {
                                ("✓ Well-organized", Color32::GREEN)
                            } else if value >= 0.3 {
                                ("⚠ Loosely structured", Color32::YELLOW)
                            } else {
                                ("✗ Chaotic", Color32::RED)
                            };
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{:.3}", value)).size(14.0).strong());
                                ui.label(egui::RichText::new(status).color(color).size(13.0));
                            });
                        }
                    });
                    
                    // 6. Affective Strength
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("⚡ Affective Strength").strong().color(Color32::from_rgb(255, 100, 100)));
                        Plot::new("affective_strength")
                            .height(plot_height)
                            .width(plot_width)
                            .show_axes([false, true])
                            .show(ui, |plot_ui| {
                                let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.affective_strength.iter())
                                    .map(|(x, y)| [*x, *y]).collect();
                                plot_ui.line(Line::new(points).color(Color32::from_rgb(255, 50, 50)));
                                plot_ui.hline(egui_plot::HLine::new(0.01).color(Color32::GREEN));
                            });
                        if let Some(&value) = state.metrics.affective_strength.back() {
                            let (status, color) = if value >= 0.01 {
                                ("✓ Emotional capacity", Color32::GREEN)
                            } else if value >= 0.001 {
                                ("⚠ Weak signals", Color32::YELLOW)
                            } else {
                                ("✗ NO EMOTION!", Color32::RED)
                            };
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{:.4}", value)).size(14.0).strong());
                                ui.label(egui::RichText::new(status).color(color).size(13.0).strong());
                            });
                        }
                    });
                    
                    // 7. Average Essence
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Average Essence").strong());
                        Plot::new("average_essence")
                            .height(plot_height)
                            .width(plot_width)
                            .show_axes([true, true])
                            .show(ui, |plot_ui| {
                                let points: PlotPoints = state.metrics.steps.iter().zip(state.metrics.average_essence.iter())
                                    .map(|(x, y)| [*x, *y]).collect();
                                plot_ui.line(Line::new(points).color(Color32::from_rgb(255, 255, 100)));
                            });
                        if let Some(&value) = state.metrics.average_essence.back() {
                            let (status, color) = if value >= 7.0 {
                                ("😊 Joyous", Color32::GREEN)
                            } else if value >= 6.0 {
                                ("🙂 Positive", Color32::LIGHT_GREEN)
                            } else if value >= 5.0 {
                                ("😐 Neutral", Color32::YELLOW)
                            } else {
                                ("😢 Suffering", Color32::RED)
                            };
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(format!("{:.2}/10", value)).size(14.0).strong());
                                ui.label(egui::RichText::new(status).color(color).size(13.0));
                            });
                        }
                    });
                });
            });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Geometric space visualization (full remaining space)
            ui.heading("Geometric Space");
            let size = ui.available_size();
                    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
                    let rect = response.rect;
                    let center = rect.center();
                    
                    // Background
                    painter.rect_filled(rect, 0.0, Color32::from_rgb(10, 10, 30));
                    
                    // Draw center marker for debugging
                    painter.circle_stroke(center, 5.0, Stroke::new(2.0, Color32::from_rgb(100, 100, 100)));
                    painter.circle_filled(center, 2.0, Color32::from_rgb(150, 150, 150));
                    
                    if state.dimension >= 2 && !state.bounds.is_empty() {
                        // Calculate auto-scale factor to fit entities in viewport
                        let max_bound = state.bounds.iter().cloned().fold(0.0f32, f32::max).max(1.0);
                        let canvas_size = rect.width().min(rect.height());
                        let auto_scale = (canvas_size * 0.4) / max_bound; // Use 40% of canvas for bounds
                        let effective_scale = auto_scale * self.zoom;
                        
                        // Draw grid lines to show the plane of existence
                        if self.show_grid {
                            let grid_spacing = auto_scale * 5.0; // Grid every 5 units in world space
                            let grid_color = Color32::from_rgba_unmultiplied(40, 40, 60, 100);
                            
                            // Vertical grid lines
                            let mut x = center.x % grid_spacing;
                            while x < rect.max.x {
                                painter.line_segment(
                                    [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                                    Stroke::new(0.5, grid_color),
                                );
                                x += grid_spacing;
                            }
                            
                            // Horizontal grid lines
                            let mut y = center.y % grid_spacing;
                            while y < rect.max.y {
                                painter.line_segment(
                                    [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                                    Stroke::new(0.5, grid_color),
                                );
                                y += grid_spacing;
                            }
                            
                            // Draw center axes
                            painter.line_segment(
                                [Pos2::new(center.x, rect.min.y), Pos2::new(center.x, rect.max.y)],
                                Stroke::new(1.0, Color32::from_rgba_unmultiplied(100, 100, 150, 150)),
                            );
                            painter.line_segment(
                                [Pos2::new(rect.min.x, center.y), Pos2::new(rect.max.x, center.y)],
                                Stroke::new(1.0, Color32::from_rgba_unmultiplied(100, 100, 150, 150)),
                            );
                        }
                        
                        // Draw attractions
                        if self.show_attractions {
                            for (idx_a, idx_b, strength) in &state.attractions {
                                if let (Some(a), Some(b)) = (state.entities.get(*idx_a), state.entities.get(*idx_b)) {
                                    if a.position.len() >= 2 && b.position.len() >= 2 {
                                        let pos_a = Pos2::new(
                                            center.x + a.position[0] * effective_scale,
                                            center.y + a.position[1] * effective_scale,
                                        );
                                        let pos_b = Pos2::new(
                                            center.x + b.position[0] * effective_scale,
                                            center.y + b.position[1] * effective_scale,
                                        );
                                        
                                        // Vary line color and width by strength
                                        let alpha = (strength.abs() * 150.0).min(255.0) as u8;
                                        let line_width = 1.5 + (strength.abs() * 3.0).min(4.0);
                                        let line_color = Color32::from_rgba_unmultiplied(100, 200, 255, alpha);
                                        painter.line_segment(
                                            [pos_a, pos_b],
                                            Stroke::new(line_width, line_color),
                                        );
                                        
                                        // Draw interaction point at midpoint for strong attractions
                                        if strength.abs() > 0.3 {
                                            let mid = Pos2::new(
                                                (pos_a.x + pos_b.x) / 2.0,
                                                (pos_a.y + pos_b.y) / 2.0,
                                            );
                                            let pulse_size = 3.0 + (strength.abs() * 4.0).min(6.0);
                                            painter.circle_filled(mid, pulse_size, Color32::from_rgba_unmultiplied(150, 220, 255, alpha));
                                            painter.circle_stroke(mid, pulse_size * 1.5, Stroke::new(1.5, Color32::from_rgba_unmultiplied(200, 240, 255, alpha / 2)));
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Draw entities
                        if state.step == 0 || state.step % 100 == 0 {
                            println!("[VIZ] Step {}: Drawing {} entities, bounds={:?}, effective_scale={:.2}", 
                                state.step, state.entities.len(), state.bounds.iter().take(2).collect::<Vec<_>>(), effective_scale);
                        }
                        
                        for entity in &state.entities {
                            if entity.position.len() >= 2 {
                                let pos = Pos2::new(
                                    center.x + entity.position[0] * effective_scale,
                                    center.y + entity.position[1] * effective_scale,
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
                                
                                // Size by attention intensity - make entities much larger and more visible
                                let attention_intensity = if self.show_attention && !entity.attention.is_empty() {
                                    entity.attention.iter().sum::<f32>() / entity.attention.len() as f32
                                } else {
                                    1.0
                                };
                                let base_radius = 12.0 * self.zoom; // Zoom affects entity size
                                let radius = base_radius + attention_intensity * 5.0;
                                
                                // Draw entity as a filled circle with prominent outline
                                painter.circle_filled(pos, radius, color);
                                painter.circle_stroke(pos, radius, Stroke::new(2.5, Color32::from_rgba_unmultiplied(255, 255, 255, 200)));
                                
                                // Draw inner ring for more visual interest
                                painter.circle_stroke(pos, radius * 0.6, Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 80)));
                                
                                // Show entity ID label - larger and more visible
                                if self.show_entity_labels {
                                    painter.text(
                                        pos + Vec2::new(0.0, -radius - 15.0),
                                        egui::Align2::CENTER_BOTTOM,
                                        format!("E{}", entity.id),
                                        egui::FontId::proportional(13.0),
                                        Color32::from_rgb(220, 220, 255),
                                    );
                                    
                                    // Show essence value below the circle
                                    painter.text(
                                        pos + Vec2::new(0.0, radius + 15.0),
                                        egui::Align2::CENTER_TOP,
                                        format!("{:.1}", entity.essence),
                                        egui::FontId::proportional(11.0),
                                        Color32::from_rgb(200, 200, 200),
                                    );
                                }
                                
                                // Show cluster count
                                if self.show_clusters && entity.num_clusters > 0 {
                                    let cluster_pos = if self.show_entity_labels {
                                        pos + Vec2::new(radius + 2.0, 0.0)
                                    } else {
                                        pos + Vec2::new(radius + 2.0, 0.0)
                                    };
                                    painter.text(
                                        cluster_pos,
                                        egui::Align2::LEFT_CENTER,
                                        format!("C:{}", entity.num_clusters),
                                        egui::FontId::proportional(10.0),
                                        Color32::from_rgb(255, 200, 100),
                                    );
                                }
                                
                                // Show velocity vector - make arrows more prominent
                                if self.show_velocity && entity.velocity.len() >= 2 {
                                    let arrow_scale = 25.0 * self.zoom; // Zoom affects arrow length
                                    let vel_end = pos + Vec2::new(
                                        entity.velocity[0] * arrow_scale,
                                        entity.velocity[1] * arrow_scale,
                                    );
                                    painter.arrow(pos, vel_end - pos, Stroke::new(2.5, Color32::from_rgb(255, 220, 0)));
                                }
                            }
                        }
                        
                        // Show debug info if no entities
                        if state.entities.is_empty() {
                            painter.text(
                                center,
                                egui::Align2::CENTER_CENTER,
                                format!("Waiting for entities... (Step: {})", state.step),
                                egui::FontId::proportional(14.0),
                                Color32::YELLOW,
                            );
                        }
                    } else {
                        // Show what's missing
                        let msg = if state.dimension < 2 {
                            format!("Invalid dimension: {}", state.dimension)
                        } else if state.bounds.is_empty() {
                            format!("Waiting for bounds configuration... (Step: {})", state.step)
                        } else {
                            format!("Dimension {} not supported", state.dimension)
                        };
                        
                        painter.text(
                            center,
                            egui::Align2::CENTER_CENTER,
                            msg,
                            egui::FontId::proportional(16.0),
                            Color32::YELLOW,
                        );
                    }
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
