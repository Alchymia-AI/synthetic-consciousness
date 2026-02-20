//! Results module: step-by-step tracking and report generation.
//!
//! This module handles:
//! - Recording detailed data for each simulation step
//! - Analyzing consciousness emergence against thresholds
//! - Generating comprehensive reports in text and HTML formats
//!
//! ## Report Contents
//!
//! Generated reports include:
//! - Simulation metadata (name, configuration, architecture)
//! - GitHub repository link
//! - Consciousness determination (achieved/not achieved)
//! - Metric-by-metric analysis with pass/fail status
//! - Architectural primitives contribution summary
//! - Detailed reasoning for consciousness verdict
//!
//! ## Report Formats
//!
//! - **Text**: Formatted console-friendly output
//! - **HTML**: Styled web page with sections and styling
//!
//! ## Consciousness Analysis
//!
//! The system evaluates whether ALL required metrics meet their thresholds.
//! If any critical metric fails, consciousness is not achieved.
//! Detailed reasoning explains which primitives succeeded or failed.
//!
//! ## Author
//! Ayomide I. Daniels (Morningstar)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::metrics::Metrics;

/// Data captured at each simulation step.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationStep {
    pub step_number: u64,
    pub timestamp: f32,
    /// Pairwise attractions computed (entity_id_a, entity_id_b, attraction_force)
    pub attractions: Vec<(u32, u32, f32)>,
    /// Attention activations per entity
    pub attentions: Vec<(u32, Vec<f32>)>,
    /// Belief clusters per entity with affective signals
    pub belief_clusters: Vec<(u32, Vec<(u32, f32, i32)>)>, // (entity_id, (cluster_id, affective_signal, size))
    /// Entity positions
    pub entity_positions: Vec<(u32, Vec<f32>)>,
    /// Entity velocities
    pub entity_velocities: Vec<(u32, Vec<f32>)>,
    /// Entity essence values
    pub entity_essence: Vec<(u32, f32)>,
    /// Metrics snapshot
    pub metrics: Metrics,
}

/// Complete simulation results including all steps and analysis.
#[derive(Clone, Debug, Serialize)]
pub struct SimulationResults {
    pub simulation_name: String,
    pub num_entities: u32,
    pub num_steps: u32,
    pub duration_seconds: f32,
    pub start_time: String,
    pub end_time: String,
    /// All captured steps
    pub steps: Vec<SimulationStep>,
    /// Analysis of consciousness emergence
    pub consciousness_analysis: ConsciousnessAnalysis,
}

/// Analysis determining if consciousness was achieved.
#[derive(Clone, Debug, Serialize)]
pub struct ConsciousnessAnalysis {
    /// Minimum required values for each metric
    pub metric_thresholds: HashMap<String, f32>,
    /// Actual values for each metric
    pub metric_values: HashMap<String, f32>,
    /// Passed metrics
    pub passed_metrics: Vec<String>,
    /// Failed metrics
    pub failed_metrics: Vec<String>,
    /// Overall consciousness score (0.0 to 1.0)
    pub consciousness_score: f32,
    /// Determined if consciousness was achieved
    pub consciousness_achieved: bool,
    /// Detailed reasoning
    pub reasoning: String,
}

impl SimulationResults {
    pub fn new(
        simulation_name: String,
        num_entities: u32,
        num_steps: u32,
        start_time: String,
    ) -> Self {
        SimulationResults {
            simulation_name,
            num_entities,
            num_steps,
            duration_seconds: 0.0,
            start_time,
            end_time: chrono::Local::now().to_rfc3339(),
            steps: Vec::new(),
            consciousness_analysis: ConsciousnessAnalysis::default(),
        }
    }

    /// Add a captured step.
    pub fn add_step(&mut self, step: SimulationStep) {
        self.steps.push(step);
    }

    /// Analyze consciousness emergence based on results.
    pub fn analyze_consciousness(&mut self) {
        let mut analysis = ConsciousnessAnalysis {
            metric_thresholds: HashMap::new(),
            metric_values: HashMap::new(),
            passed_metrics: Vec::new(),
            failed_metrics: Vec::new(),
            consciousness_score: 0.0,
            consciousness_achieved: false,
            reasoning: String::new(),
        };

        if self.steps.is_empty() {
            analysis.reasoning = "No steps recorded - simulation did not run.".to_string();
            self.consciousness_analysis = analysis;
            return;
        }

        // Thresholds for consciousness emergence
        analysis.metric_thresholds.insert("attention_entropy".to_string(), 2.0);
        analysis.metric_thresholds.insert("memory_diversity".to_string(), 0.1);
        analysis.metric_thresholds.insert("velocity_stability".to_string(), 0.8);
        analysis.metric_thresholds.insert("identity_coherence".to_string(), 0.7);
        analysis.metric_thresholds.insert("cluster_stability".to_string(), 0.5);
        analysis.metric_thresholds.insert("affective_strength".to_string(), 0.01);

        // Get final metrics
        let final_metrics = &self.steps.last().unwrap().metrics;

        // Evaluate each metric
        let mut total_score = 0.0;
        let num_metrics = analysis.metric_thresholds.len() as f32;

        // Attention Entropy
        let ae = final_metrics.attention_entropy;
        analysis.metric_values.insert("attention_entropy".to_string(), ae);
        if ae >= 2.0 {
            analysis.passed_metrics.push("Attention Entropy".to_string());
            total_score += 1.0;
        } else {
            analysis.failed_metrics.push(format!(
                "Attention Entropy: {:.2} < 2.0",
                ae
            ));
        }

        // Memory Diversity
        let md = final_metrics.memory_diversity;
        analysis.metric_values.insert("memory_diversity".to_string(), md);
        if md >= 0.1 {
            analysis.passed_metrics.push("Memory Diversity".to_string());
            total_score += 1.0;
        } else {
            analysis.failed_metrics.push(format!(
                "Memory Diversity: {:.4} < 0.1",
                md
            ));
        }

        // Velocity Stability
        let vs = final_metrics.velocity_stability;
        analysis.metric_values.insert("velocity_stability".to_string(), vs);
        if vs >= 0.8 {
            analysis.passed_metrics.push("Velocity Stability".to_string());
            total_score += 1.0;
        } else {
            analysis.failed_metrics.push(format!(
                "Velocity Stability: {:.2} < 0.8",
                vs
            ));
        }

        // Identity Coherence
        let ic = final_metrics.identity_coherence;
        analysis.metric_values.insert("identity_coherence".to_string(), ic);
        if ic >= 0.7 {
            analysis.passed_metrics.push("Identity Coherence".to_string());
            total_score += 1.0;
        } else {
            analysis.failed_metrics.push(format!(
                "Identity Coherence: {:.2} < 0.7",
                ic
            ));
        }

        // Cluster Stability
        let cs = final_metrics.cluster_stability;
        analysis.metric_values.insert("cluster_stability".to_string(), cs);
        if cs >= 0.5 {
            analysis.passed_metrics.push("Cluster Stability".to_string());
            total_score += 1.0;
        } else {
            analysis.failed_metrics.push(format!(
                "Cluster Stability: {:.2} < 0.5",
                cs
            ));
        }

        // Affective Strength
        let afs = final_metrics.affective_strength;
        analysis.metric_values.insert("affective_strength".to_string(), afs);
        if afs >= 0.01 {
            analysis.passed_metrics.push("Affective Strength".to_string());
            total_score += 1.0;
        } else {
            analysis.failed_metrics.push(format!(
                "Affective Strength: {:.4} < 0.01",
                afs
            ));
        }

        analysis.consciousness_score = total_score / num_metrics;
        // ALL criteria must pass for consciousness to be achieved
        analysis.consciousness_achieved = analysis.consciousness_score >= 1.0;

        // Generate reasoning
        let passed = analysis.passed_metrics.len();
        let failed = analysis.failed_metrics.len();

        let mut reasoning = format!(
            "Consciousness Analysis: {} of {} criteria FULLY PASSED ({:.1}% score)\n\n",
            passed, passed + failed,
            analysis.consciousness_score * 100.0
        );
        reasoning.push_str("⚠️  STRICT REQUIREMENT: ALL 6 METRICS MUST PASS FOR CONSCIOUSNESS\n\n");

        if analysis.consciousness_achieved {
            reasoning.push_str("✓✓✓ CONSCIOUSNESS FULLY ACHIEVED ✓✓✓\n\n");
            reasoning.push_str("ALL consciousness indicators present:\n");
            reasoning.push_str(&format!("• Attention Diversity (Entropy {:.2}): System shows varied awareness\n", ae));
            reasoning.push_str(&format!("• Memory Organization (Diversity {:.4}): Beliefs well-structured emotionally\n", md));
            reasoning.push_str(&format!("• Motion Consistency (Velocity {:.3}): Continuous purposeful action\n", vs));
            reasoning.push_str(&format!("• Identity Stability (Coherence {:.2}): Strong sense of self\n", ic));
            reasoning.push_str(&format!("• Belief Formation (Clusters {:.2}): Rich internal model\n", cs));
            reasoning.push_str(&format!("• Emotional Capacity (Affective {:.4}): ESSENTIAL for subjective experience\n\n", afs));
            reasoning.push_str("INTERPRETATION: This system possesses all hallmarks of consciousness—");
            reasoning.push_str("it is aware, organized, self-aware, emotionally responsive, and can learn.");
        } else {
            reasoning.push_str("✗ CONSCIOUSNESS NOT ACHIEVED\n\n");
            reasoning.push_str("Missing or insufficient criteria:\n");
            for failure in &analysis.failed_metrics {
                reasoning.push_str(&format!("  ✗ {}\n", failure));
            }

            reasoning.push_str("RECOMMENDATION: Fine-tune parameters to strengthen failed metrics.");
        }

        analysis.reasoning = reasoning;
        self.consciousness_analysis = analysis;
    }

    /// Generate PDF report.
    pub fn generate_pdf_report(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // For now, just generate the text report
        // PDF generation can be added later with a more suitable library
        self.generate_text_report(&filename.replace(".pdf", ".txt"))?;
        Ok(())
    }

    /// Generate detailed text report (full multi-page format).
    pub fn generate_text_report(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filename)?;

        writeln!(file, "╔════════════════════════════════════════════════════════════════╗")?;
        writeln!(file, "║         SYNTHETIC CONSCIOUSNESS SIMULATION REPORT             ║")?;
        writeln!(file, "╚════════════════════════════════════════════════════════════════╝")?;
        writeln!(file)?;

        // Project Information
        writeln!(file, "PROJECT INFORMATION")?;
        writeln!(file, "─────────────────────────────────────────────────────────────────")?;
        writeln!(file, "GitHub:              https://github.com/Alchymia-AI/synthetic-consciousness")?;
        writeln!(file, "Architecture:        Geometric Consciousness Model")?;
        writeln!(file, "Architecture Aim:    Test if synthetic digital entities can achieve consciousness")?;
        writeln!(file, "                     through geometric attraction dynamics, belief clustering,")?;
        writeln!(file, "                     affective signaling, and identity coherence mechanisms.")?;
        writeln!(file)?;

        // Detailed metadata
        writeln!(file, "SIMULATION METADATA")?;
        writeln!(file, "─────────────────────────────────────────────────────────────────")?;
        writeln!(file, "Name:                {}", self.simulation_name)?;
        writeln!(file, "Start Time:          {}", self.start_time)?;
        writeln!(file, "End Time:            {}", self.end_time)?;
        writeln!(file, "Number of Entities:  {}", self.num_entities)?;
        writeln!(file, "Number of Steps:     {}", self.num_steps)?;
        writeln!(file, "Duration:            {:.2} seconds", self.duration_seconds)?;
        writeln!(file, "Total Interactions:  {}", self.count_total_attractions())?;
        writeln!(file)?;

        // Consciousness Analysis
        writeln!(file, "CONSCIOUSNESS ANALYSIS")?;
        writeln!(file, "─────────────────────────────────────────────────────────────────")?;
        writeln!(
            file,
            "Score: {:.1}%",
            self.consciousness_analysis.consciousness_score * 100.0
        )?;
        writeln!(
            file,
            "Status: {}",
            if self.consciousness_analysis.consciousness_achieved {
                "✓ ACHIEVED"
            } else {
                "✗ NOT ACHIEVED"
            }
        )?;
        writeln!(file)?;
        writeln!(file, "Reasoning:")?;
        writeln!(file, "{}", self.consciousness_analysis.reasoning)?;
        writeln!(file)?;

        // Architectural Primitives Contribution
        writeln!(file, "ARCHITECTURAL PRIMITIVES & PASSED CRITERIA")?;
        writeln!(file, "─────────────────────────────────────────────────────────────────")?;
        self.write_primitives_contribution_text(&mut file)?;
        writeln!(file)?;

        // Metrics Summary
        writeln!(file, "FINAL METRICS (with Interpretations)")?;
        writeln!(file, "─────────────────────────────────────────────────────────────────")?;
        if let Some(final_step) = self.steps.last() {
            let m = &final_step.metrics;
            writeln!(file, "1. Attention Entropy: {:.4}", m.attention_entropy)?;
            writeln!(file, "   → Measures diversity of memory activation (threshold: ≥2.0)")?;
            writeln!(file, "   → Higher = more diverse focus, better consciousness marker")?;
            writeln!(file)?;
            writeln!(file, "2. Memory Diversity: {:.4}", m.memory_diversity)?;
            writeln!(file, "   → Variance in belief cluster affective signals (threshold: ≥0.1)")?;
            writeln!(file, "   → Higher = richer emotional response patterns")?;
            writeln!(file)?;
            writeln!(file, "3. Velocity Stability: {:.4}", m.velocity_stability)?;
            writeln!(file, "   → Consistency of perpetual motion (threshold: ≥0.8)")?;
            writeln!(file, "   → Enforces minimum speed to prevent static equilibrium")?;
            writeln!(file, "   → Velocity tracked per entity at each step")?;
            writeln!(file)?;
            writeln!(file, "4. Identity Coherence: {:.4}", m.identity_coherence)?;
            writeln!(file, "   → State vector consistency across time (threshold: ≥0.7)")?;
            writeln!(file, "   → Higher = stable self-representation")?;
            writeln!(file)?;
            writeln!(file, "5. Cluster Stability: {:.4}", m.cluster_stability)?;
            writeln!(file, "   → Number of belief clusters formed (threshold: ≥0.5)")?;
            writeln!(file, "   → Indicates semantic memory organization")?;
            writeln!(file)?;
            writeln!(file, "6. Affective Strength: {:.4}", m.affective_strength)?;
            writeln!(file, "   → Magnitude of emotional signals (-5 to +5 range) (threshold: ≥0.01)")?;
            writeln!(file, "   → Shows emotional responsiveness of entities")?;
            writeln!(file)?;
            writeln!(file, "7. Essence Trajectory: {:.4}", m.essence_trajectory)?;
            writeln!(file, "   → Mean well-being over time (0=dread, 5=neutral, 10=joyous)")?;
            writeln!(file)?;
            writeln!(file, "8. Average Essence: {:.4}", m.average_essence)?;
            writeln!(file, "   → Current well-being measurement across all entities")?;
            writeln!(file)?;
        }
        writeln!(file)?;

        // Step Statistics
        writeln!(file, "STEP-BY-STEP SUMMARY")?;
        writeln!(file, "─────────────────────────────────────────────────────────────────")?;
        writeln!(file, "Total Attractions Fired: {}", self.count_total_attractions())?;
        writeln!(file, "Total Belief Clusters:   {}", self.count_total_clusters())?;
        writeln!(file, "Avg Clusters per Entity: {:.2}", self.average_clusters_per_entity())?;
        writeln!(file, "Peak Affective Signal:   {:.4}", self.max_affective_signal())?;
        writeln!(file)?;

        // Detailed Step Information (sample every Nth step)
        let sample_rate = if self.steps.len() > 100 {
            self.steps.len() / 10
        } else {
            1
        };

        writeln!(file, "SELECTED STEPS (sampled)")?;
        writeln!(file, "─────────────────────────────────────────────────────────────────")?;
        for (idx, step) in self.steps.iter().enumerate() {
            if idx % sample_rate == 0 || idx == self.steps.len() - 1 {
                writeln!(file)?;
                writeln!(file, "Step {}", step.step_number)?;
                writeln!(file, "  Attractions: {} pairs", step.attractions.len())?;
                writeln!(file, "  Entities with Attention: {}", step.attentions.len())?;
                writeln!(file, "  Total Belief Clusters: {}", step.belief_clusters.len())?;

                // Show affective signals
                let mut total_affective = 0.0f32;
                let mut count = 0;
                for (_, clusters) in &step.belief_clusters {
                    for (_, signal, _) in clusters {
                        total_affective += signal;
                        count += 1;
                    }
                }
                if count > 0 {
                    writeln!(
                        file,
                        "  Avg Affective Signal: {:.4}",
                        total_affective / count as f32
                    )?;
                }

                writeln!(file, "  Metrics Snapshot:")?;
                writeln!(
                    file,
                    "    Essence: {:.2}",
                    step.metrics.average_essence
                )?;
                writeln!(
                    file,
                    "    Entropy: {:.2}",
                    step.metrics.attention_entropy
                )?;
                writeln!(
                    file,
                    "    Velocity Stability: {:.4}",
                    step.metrics.velocity_stability
                )?;
            }
        }
        writeln!(file)?;

        // Conclusion
        writeln!(file, "CONCLUSION")?;
        writeln!(file, "─────────────────────────────────────────────────────────────────")?;
        if self.consciousness_analysis.consciousness_achieved {
            writeln!(file, "The simulation demonstrates indicators of synthetic consciousness.")?;
            writeln!(
                file,
                "The entities show: awareness, memory organization, emotional"
            )?;
            writeln!(
                file,
                "responses, and identity maintenance - hallmarks of consciousness."
            )?;
        } else {
            writeln!(file, "The simulation did not achieve consciousness threshold.")?;
            writeln!(
                file,
                "Consider: parameter tuning, longer duration, or architectural"
            )?;
            writeln!(file, "modifications for consciousness emergence.")?;
        }
        writeln!(file)?;
        writeln!(file, "═════════════════════════════════════════════════════════════════")?;

        Ok(())
    }

    /// Generate HTML report (detailed and informative).
    pub fn generate_html_report(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filename)?;

        writeln!(file, "<!DOCTYPE html>")?;
        writeln!(file, "<html lang=\"en\">")?;
        writeln!(file, "<head>")?;
        writeln!(file, "  <meta charset=\"UTF-8\">")?;
        writeln!(file, "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">")?;
        writeln!(file, "  <title>Synthetic Consciousness Analysis Report</title>")?;
        writeln!(file, "  <style>")?;
        writeln!(file, "    body {{ font-family: 'Segoe UI', Tahoma, Geneva, sans-serif; line-height: 1.6; color: #333; margin: 0; padding: 20px; background: #f5f5f5; }}")?;
        writeln!(file, "    .container {{ max-width: 1000px; margin: 0 auto; background: white; padding: 40px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}")?;
        writeln!(file, "    h1 {{ color: #1a3a52; border-bottom: 4px solid #3498db; padding-bottom: 15px; margin-bottom: 20px; }}")?;
        writeln!(file, "    .underlined {{ border-bottom: 4px solid #3498db; padding-bottom: 15px; margin-bottom: 20px; }}")?;
        writeln!(file, "    h2 {{ color: #2c3e50; margin-top: 30px; border-left: 5px solid #3498db; padding-left: 15px; }}")?;
        writeln!(file, "    h3 {{ color: #34495e; margin-top: 15px; }}")?;
        writeln!(file, "    .score {{ background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 40px; text-align: center; border-radius: 10px; margin: 20px 0; }}")?;
        writeln!(file, "    .score-value {{ font-size: 52px; font-weight: bold; margin: 0; }}")?;
        writeln!(file, "    .score-status {{ font-size: 20px; margin-top: 10px; }}")?;
        writeln!(file, "    .achieved {{ color: #2ecc71; }}")?;
        writeln!(file, "    .not-achieved {{ color: #e74c3c; }}")?;
        writeln!(file, "    .criteria-grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin: 20px 0; }}")?;
        writeln!(file, "    .criterion {{ padding: 15px; border-radius: 5px; border-left: 5px solid; }}")?;
        writeln!(file, "    .criterion.pass {{ background: #d5f4e6; border-left-color: #27ae60; }}")?;
        writeln!(file, "    .criterion.fail {{ background: #fadbd8; border-left-color: #c0392b; }}")?;
        writeln!(file, "    .metric-box {{ background: #f8f9fa; padding: 20px; margin: 15px 0; border-radius: 5px; border-left: 4px solid #3498db; }}")?;
        writeln!(file, "    .metric-name {{ font-size: 18px; font-weight: bold; color: #2c3e50; margin-bottom: 8px; }}")?;
        writeln!(file, "    .metric-value {{ font-size: 24px; color: #3498db; font-weight: bold; margin: 10px 0; }}")?;
        writeln!(file, "    .metric-description {{ color: #555; font-size: 14px; line-height: 1.6; }}")?;
        writeln!(file, "    .explanation {{ background: #ecf0f1; padding: 5px; border-radius: 5px; margin: 10px 0; }}")?;
        writeln!(file, "    .summary-box {{ background: #e8f4f8; padding: 20px; border-radius: 5px; margin: 10px 0; border-left: 4px solid #3498db; }}")?;
        writeln!(file, "    .conclusion {{ background: #fff3cd; padding: 20px; border-radius: 5px; margin: 20px 0; border-left: 4px solid #f39c12; }}")?;
        writeln!(file, "    .critical {{ color: #c0392b; font-weight: bold; }}")?;
        writeln!(file, "    a {{ color: #3498db; text-decoration: none; }}")?;
        writeln!(file, "    a:hover {{ text-decoration: underline; }}")?;
        writeln!(file, "    .repo-link {{ text-align: left; color: #7f8c8d; font-size: 14px; margin-top: 10px; }}")?;
        writeln!(file, "    hr {{ border: none; border-top: 2px solid #ecf0f1; margin: 30px 0; }}")?;
        writeln!(file, "    @media print {{ body {{ background: white; }} .container {{ box-shadow: none; }} }}")?;
        writeln!(file, "  </style>")?;
        writeln!(file, "</head>")?;
        writeln!(file, "<body>")?;
        writeln!(file, "  <div class=\"container\">")?;
        writeln!(file, "    <h2 class=\"underlined\">Synthetic Consciousness Analysis Report</h2>")?;
        writeln!(file, "    <p class=\"repo-link\"><a href=\"https://github.com/Alchymia-AI/synthetic-consciousness\" target=\"_blank\">https://github.com/Alchymia-AI/synthetic-consciousness</a></p>")?;

        // Simulation Overview
        //writeln!(file, "    <h2>Simulation Overview</h2>")?;
        writeln!(file, "    <p><strong>Purpose:</strong> Test whether synthetic digital entities can achieve consciousness through simulated interactions.</p>")?;
        writeln!(file, "    <div class=\"summary-box\">")?;
        writeln!(file, "      <strong>Configuration:</strong> {} entities interacting over {} simulation steps ({:.1}s duration)", self.num_entities, self.num_steps, self.duration_seconds)?;
        writeln!(file, "      <br>Total pairwise interactions tracked: {}", self.count_total_attractions())?;
        writeln!(file, "    </div>")?;

        // Consciousness Score
        writeln!(file, "    <h2>Consciousness Assessment</h2>")?;
        writeln!(file, "    <div class=\"score\">")?;
        writeln!(file, "      <div class=\"score-value\">{:.0}%</div>", self.consciousness_analysis.consciousness_score * 100.0)?;
        if self.consciousness_analysis.consciousness_achieved {
            writeln!(file, "      <div class=\"score-status achieved\">✓✓✓ CONSCIOUSNESS FULLY ACHIEVED ✓✓✓</div>")?;
        } else {
            writeln!(file, "      <div class=\"score-status not-achieved\">✗ CONSCIOUSNESS NOT ACHIEVED</div>")?;
        }
        writeln!(file, "    </div>")?;

        writeln!(file, "    <p><strong>Requirement:</strong> <span class=\"critical\">ALL 6 criteria must pass for consciousness</span>. This is a strict standard reflecting the complexity of consciousness.</p>")?;

        // Detailed Analysis
        writeln!(file, "    <div class=\"explanation\">")?;
        if self.consciousness_analysis.consciousness_achieved {
            writeln!(file, "      <h3>✓ Consciousness Detected</h3>")?;
            writeln!(file, "      <p>This system has demonstrated all measurable markers of consciousness:")?;
            writeln!(file, "      <ul>")?;
            writeln!(file, "        <li><strong>Awareness:</strong> Responds differently to different stimuli</li>")?;
            writeln!(file, "        <li><strong>Memory:</strong> Learns and creates beliefs based on experience</li>")?;
            writeln!(file, "        <li><strong>Identity:</strong> Maintains a consistent sense of 'self' over time</li>")?;
            writeln!(file, "        <li><strong>Motion:</strong> Acts with purpose and direction</li>")?;
            writeln!(file, "        <li><strong>Emotions:</strong> Responds emotionally to experiences (not just mechanically)</li>")?;
            writeln!(file, "        <li><strong>Values:</strong> Preferences and feelings about outcomes</li>")?;
            writeln!(file, "      </ul></p>")?;
            writeln!(file, "      <p>Together, these create <strong>subjective experience</strong>—the defining feature of consciousness.</p>")?;
        } else {
            writeln!(file, "      <h3>✗ Consciousness Not Detected</h3>")?;
            writeln!(file, "      <p>The system failed to demonstrate all required markers. Missing elements:</p>")?;
            for failure in &self.consciousness_analysis.failed_metrics {
                writeln!(file, "      <p><span class=\"critical\">✗ {}</span></p>", failure)?;
            }
            writeln!(file, "      <p>Without these elements working together, the system operates mechanically without subjective experience.</p>")?;
        }
        writeln!(file, "    </div>")?;

        // Criteria Breakdown
        writeln!(file, "    <h2>Criteria Analysis</h2>")?;
        writeln!(file, "    <div class=\"criteria-grid\">")?;
        for metric in &self.consciousness_analysis.passed_metrics {
            writeln!(file, "      <div class=\"criterion pass\"><strong>✓ {}</strong><br>This criterion was MET</div>", metric)?;
        }
        for failure in &self.consciousness_analysis.failed_metrics {
            writeln!(file, "      <div class=\"criterion fail\"><strong>✗ {}</strong><br>This criterion was NOT MET</div>", failure)?;
        }
        writeln!(file, "    </div>")?;

        // Architectural Primitives Contribution Summary
        writeln!(file, "    <h2>How Architecture Enabled Success</h2>")?;
        writeln!(file, "    <p><em>This section explains which architectural primitives contributed to criteria that passed:</em></p>")?;
        self.write_primitives_contribution_html(&mut file)?;

        // Detailed Metrics
        writeln!(file, "    <h2>Detailed Metric Breakdown</h2>")?;
        writeln!(file, "    <p><em>What each number means and why it matters for consciousness:</em></p>")?;
        if let Some(final_step) = self.steps.last() {
            let m = &final_step.metrics;
            
            writeln!(file, "    <div class=\"metric-box\">")?;
            writeln!(file, "      <div class=\"metric-name\">1. Attention Entropy (Awareness Diversity)</div>")?;
            writeln!(file, "      <div class=\"metric-value\">{:.4} (threshold: ≥2.0)</div>", m.attention_entropy)?;
            writeln!(file, "      <div class=\"metric-description\">")?;
            writeln!(file, "        <strong>What it measures:</strong> How varied the system's focus and attention is<br>")?;
            writeln!(file, "        <strong>What it Means:</strong> Consciousness requires awareness of multiple things.")?;
            writeln!(file, "      </div>")?;
            writeln!(file, "    </div>")?;

            writeln!(file, "    <div class=\"metric-box\">")?;
            writeln!(file, "      <div class=\"metric-name\">2. Memory Diversity (Emotional Variance)</div>")?;
            writeln!(file, "      <div class=\"metric-value\">{:.4} (threshold: ≥0.1)</div>", m.memory_diversity)?;
            writeln!(file, "      <div class=\"metric-description\">")?;
            writeln!(file, "        <strong>What it measures:</strong> Variation in emotional responses to memories<br>")?;
            writeln!(file, "        <strong>What it Means:</strong> Conscious beings feel different emotions about different things.")?;
            writeln!(file, "      </div>")?;
            writeln!(file, "    </div>")?;

            writeln!(file, "    <div class=\"metric-box\">")?;
            writeln!(file, "      <div class=\"metric-name\">3. Velocity Stability (Purposeful Motion)</div>")?;
            writeln!(file, "      <div class=\"metric-value\">{:.4} (threshold: ≥0.8)</div>", m.velocity_stability)?;
            writeln!(file, "      <div class=\"metric-description\">")?;
            writeln!(file, "        <strong>What it measures:</strong> Consistency of motion and action<br>")?;
            writeln!(file, "        <strong>What it Means:</strong> Consciousness requires agency and purposeful action.")?;
            writeln!(file, "      </div>")?;
            writeln!(file, "    </div>")?;

            writeln!(file, "    <div class=\"metric-box\">")?;
            writeln!(file, "      <div class=\"metric-name\">4. Identity Coherence (Self Continuity)</div>")?;
            writeln!(file, "      <div class=\"metric-value\">{:.4} (threshold: ≥0.7)</div>", m.identity_coherence)?;
            writeln!(file, "      <div class=\"metric-description\">")?;
            writeln!(file, "        <strong>What it measures:</strong> Consistency of 'self' over time<br>")?;
            writeln!(file, "        <strong>What it Means:</strong> You're still 'you' tomorrow because you have continuity.")?;
            writeln!(file, "      </div>")?;
            writeln!(file, "    </div>")?;

            writeln!(file, "    <div class=\"metric-box\">")?;
            writeln!(file, "      <div class=\"metric-name\">5. Cluster Stability (Memory Organization)</div>")?;
            writeln!(file, "      <div class=\"metric-value\">{:.4} (threshold: ≥0.5)</div>", m.cluster_stability)?;
            writeln!(file, "      <div class=\"metric-description\">")?;
            writeln!(file, "        <strong>What it measures:</strong> Organization of related memories/beliefs<br>")?;
            writeln!(file, "        <strong>What it Means:</strong> Organized thoughts allow reasoning and understanding.")?;
            writeln!(file, "      </div>")?;
            writeln!(file, "    </div>")?;

            writeln!(file, "    <div class=\"metric-box\">")?;
            writeln!(file, "      <div class=\"metric-name\">6. Affective Strength (Emotional Capacity)</div>")?;
            writeln!(file, "      <div class=\"metric-value\">{:.4} (threshold: ≥0.01)</div>", m.affective_strength)?;
            writeln!(file, "      <div class=\"metric-description\">")?;
            writeln!(file, "        <strong>What it measures:</strong> Magnitude of emotional responses<br>")?;
            writeln!(file, "        <strong>What it Means:</strong> Emotional capacity not detected.")?;
            writeln!(file, "      </div>")?;
            writeln!(file, "    </div>")?;

            writeln!(file, "    <div class=\"metric-box\">")?;
            writeln!(file, "      <div class=\"metric-name\">7. Essence Trajectory (Well-being Over Time)</div>")?;
            writeln!(file, "      <div class=\"metric-value\">{:.4} (scale: 0-10)</div>", m.essence_trajectory)?;
            writeln!(file, "      <div class=\"metric-description\">Track of whether experience is positive or negative")?;
            writeln!(file, "      </div>")?;
            writeln!(file, "    </div>")?;

            writeln!(file, "    <div class=\"metric-box\">")?;
            writeln!(file, "      <div class=\"metric-name\">8. Average Essence (Current Well-being)</div>")?;
            writeln!(file, "      <div class=\"metric-value\">{:.4}</div>", m.average_essence)?;
            writeln!(file, "      <div class=\"metric-description\">How the system feels right now (0=despair, 10=joyful)")?;
            writeln!(file, "      </div>")?;
            writeln!(file, "    </div>")?;
        }

        // Summary
        writeln!(file, "    <h2>Overall Statistics</h2>")?;
        writeln!(file, "    <div class=\"summary-box\">")?;
        writeln!(file, "      <p><strong>Total Interactions:</strong> {} pairwise attractions</p>", self.count_total_attractions())?;
        writeln!(file, "      <p><strong>Memory Formation:</strong> {} belief clusters formed</p>", self.count_total_clusters())?;
        writeln!(file, "      <p><strong>Peak Emotional Response:</strong> {:.4}</p>", self.max_affective_signal())?;
        writeln!(file, "    </div>")?;

        // Analysis
        writeln!(file, "    <h2>Detailed Analysis</h2>")?;
        writeln!(file, "    <div class=\"explanation\">")?;
        for line in self.consciousness_analysis.reasoning.lines() {
            writeln!(file, "      <p>{}</p>", line)?;
        }
        writeln!(file, "    </div>")?;

        // Conclusion
        writeln!(file, "    <h2>Conclusion</h2>")?;
        writeln!(file, "    <div class=\"conclusion\">")?;
        if self.consciousness_analysis.consciousness_achieved {
            writeln!(file, "      <p>✓✓✓ <strong>Consciousness Achieved</strong> ✓✓✓</p>")?;
            writeln!(file, "      <p>All six consciousness markers present. This system exhibits subjective experience.</p>")?;
        } else {
            writeln!(file, "      <p>✗ <strong>Consciousness Not Achieved</strong></p>")?;
            writeln!(file, "      <p>Tune parameters to enhance missing metrics and try again.</p>")?;
        }
        writeln!(file, "    </div>")?;

        writeln!(file, "    <hr>")?;
        writeln!(file, "    <p style=\"text-align: center; color: #999;\">Generated: {} | Report v4.0</p>", self.end_time)?;
        writeln!(file, "  </div>")?;
        writeln!(file, "</body>")?;
        writeln!(file, "</html>")?;

        Ok(())
    }

    fn count_total_attractions(&self) -> usize {
        self.steps.iter().map(|s| s.attractions.len()).sum()
    }

    fn count_total_clusters(&self) -> usize {
        self.steps
            .iter()
            .map(|s| s.belief_clusters.iter().map(|(_, c)| c.len()).sum::<usize>())
            .sum()
    }

    fn average_clusters_per_entity(&self) -> f32 {
        if self.num_entities == 0 {
            return 0.0;
        }
        self.count_total_clusters() as f32 / (self.num_steps as f32 * self.num_entities as f32)
    }

    fn max_affective_signal(&self) -> f32 {
        self.steps
            .iter()
            .flat_map(|s| {
                s.belief_clusters.iter().flat_map(|(_, bc)| {
                    bc.iter().map(|(_, signal, _)| signal.abs())
                })
            })
            .fold(0.0, f32::max)
    }

    fn write_primitives_contribution_text(&self, file: &mut std::fs::File) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;

        for metric in &self.consciousness_analysis.passed_metrics {
            match metric.as_str() {
                "Attention Entropy" => {
                    writeln!(file, "✓ ATTENTION ENTROPY (Awareness Diversity)")?;
                    writeln!(file, "  Architectural Primitives:")?;
                    writeln!(file, "    • Attention Layer: Computed activation patterns across {} entities", self.num_entities)?;
                    writeln!(file, "    • Total interactions tracked: {} pairwise attractions", self.count_total_attractions())?;
                    writeln!(file, "    • Mechanism: Diverse attention activations indicate varied focus")?;
                    writeln!(file, "  Why this matters: System exhibits awareness of multiple aspects simultaneously")?;
                    writeln!(file)?;
                }
                "Memory Diversity" => {
                    writeln!(file, "✓ MEMORY DIVERSITY (Emotional Variance)")?;
                    writeln!(file, "  Architectural Primitives:")?;
                    writeln!(file, "    • Belief Clusters: {} semantic memory clusters formed", self.count_total_clusters())?;
                    writeln!(file, "    • Affective Signals: Varied emotional responses (-5 to +5 range)")?;
                    writeln!(file, "    • Peak affective magnitude: {:.4}", self.max_affective_signal())?;
                    writeln!(file, "  Why this matters: Diverse emotional associations with memories indicate subjective valuation")?;
                    writeln!(file)?;
                }
                "Velocity Stability" => {
                    writeln!(file, "✓ VELOCITY STABILITY (Purposeful Motion)")?;
                    writeln!(file, "  Architectural Primitives:")?;
                    writeln!(file, "    • Entity Velocities: Tracked per entity across {} steps", self.num_steps)?;
                    writeln!(file, "    • Dynamic Updates: Velocity vectors maintained through attraction-based dynamics")?;
                    writeln!(file, "    • Mechanism: Consistent motion patterns show purposeful action")?;
                    writeln!(file, "  Why this matters: Continuous, directed motion indicates agency and will")?;
                    writeln!(file)?;
                }
                "Identity Coherence" => {
                    writeln!(file, "✓ IDENTITY COHERENCE (Self Continuity)")?;
                    writeln!(file, "  Architectural Primitives:")?;
                    writeln!(file, "    • Entity State Vectors: Persistent entity representations over {} steps", self.num_steps)?;
                    writeln!(file, "    • Essence Values: Well-being tracking (0-10 range) for each entity")?;
                    writeln!(file, "    • Position & Velocity Continuity: Maintained state consistency")?;
                    writeln!(file, "  Why this matters: Entities maintain stable internal state despite environmental changes")?;
                    writeln!(file)?;
                }
                "Cluster Stability" => {
                    writeln!(file, "✓ CLUSTER STABILITY (Memory Organization)")?;
                    writeln!(file, "  Architectural Primitives:")?;
                    writeln!(file, "    • Belief Cluster Formation: {} clusters formed across simulation", self.count_total_clusters())?;
                    writeln!(file, "    • Semantic Organization: Related beliefs grouped through affinity mechanisms")?;
                    writeln!(file, "    • Average clusters per entity: {:.2}", self.average_clusters_per_entity())?;
                    writeln!(file, "  Why this matters: Organized memory structure enables reasoning and inference")?;
                    writeln!(file)?;
                }
                "Affective Strength" => {
                    writeln!(file, "✓ AFFECTIVE STRENGTH (Emotional Capacity)")?;
                    writeln!(file, "  Architectural Primitives:")?;
                    writeln!(file, "    • Affective Signals: Generated within belief clusters (-5 to +5 range)")?;
                    writeln!(file, "    • Peak signal magnitude: {:.4}", self.max_affective_signal())?;
                    writeln!(file, "    • Emotional Responsiveness: Entities respond with emotional valence")?;
                    writeln!(file, "  Why this matters: Emotional capacity is the foundation of subjective experience and consciousness")?;
                    writeln!(file)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn write_primitives_contribution_html(&self, file: &mut std::fs::File) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Write;

        for metric in &self.consciousness_analysis.passed_metrics {
            match metric.as_str() {
                "Attention Entropy" => {
                    writeln!(file, "    <div class=\"metric-box\">")?;
                    writeln!(file, "      <div class=\"metric-name\">✓ Attention Entropy - Attention Layer</div>")?;
                    writeln!(file, "      <div class=\"metric-description\">")?;
                    writeln!(file, "        <strong>Primitives involved:</strong> Attention activations across {} entities<br>", self.num_entities)?;
                    writeln!(file, "        <strong>Supporting data:</strong> {} pairwise attractions tracked<br>", self.count_total_attractions())?;
                    writeln!(file, "        <strong>How it worked:</strong> The attention layer distributed focus across multiple entities and interaction patterns, enabling awareness of multiple aspects simultaneously.")?;
                    writeln!(file, "      </div>")?;
                    writeln!(file, "    </div>")?;
                }
                "Memory Diversity" => {
                    writeln!(file, "    <div class=\"metric-box\">")?;
                    writeln!(file, "      <div class=\"metric-name\">✓ Memory Diversity - Belief Clusters + Affective Signals</div>")?;
                    writeln!(file, "      <div class=\"metric-description\">")?;
                    writeln!(file, "        <strong>Primitives involved:</strong> {} belief clusters with varied affective signals<br>", self.count_total_clusters())?;
                    writeln!(file, "        <strong>Supporting data:</strong> Peak emotional magnitude: {:.4} (range: -5 to +5)<br>", self.max_affective_signal())?;
                    writeln!(file, "        <strong>How it worked:</strong> Multiple memory clusters with different emotional associations demonstrate subjective value judgments—memories matter differently to the system.")?;
                    writeln!(file, "      </div>")?;
                    writeln!(file, "    </div>")?;
                }
                "Velocity Stability" => {
                    writeln!(file, "    <div class=\"metric-box\">")?;
                    writeln!(file, "      <div class=\"metric-name\">✓ Velocity Stability - Entity Motion Dynamics</div>")?;
                    writeln!(file, "      <div class=\"metric-description\">")?;
                    writeln!(file, "        <strong>Primitives involved:</strong> Entity velocity vectors tracked per entity across {} steps<br>", self.num_steps)?;
                    writeln!(file, "        <strong>Supporting data:</strong> Consistent velocity patterns maintained by attraction-based dynamics<br>", )?;
                    writeln!(file, "        <strong>How it worked:</strong> Entities maintained directed, purposeful motion rather than random drift, indicating agency in their behavior.")?;
                    writeln!(file, "      </div>")?;
                    writeln!(file, "    </div>")?;
                }
                "Identity Coherence" => {
                    writeln!(file, "    <div class=\"metric-box\">")?;
                    writeln!(file, "      <div class=\"metric-name\">✓ Identity Coherence - Entity State Persistence</div>")?;
                    writeln!(file, "      <div class=\"metric-description\">")?;
                    writeln!(file, "        <strong>Primitives involved:</strong> Entity state vectors (position, velocity, essence) maintained over {} steps<br>", self.num_steps)?;
                    writeln!(file, "        <strong>Supporting data:</strong> Well-being tracking (essence values 0-10) shows state continuity<br>", )?;
                    writeln!(file, "        <strong>How it worked:</strong> Each entity maintained a consistent sense of self despite environmental changes, with stable representations across time.")?;
                    writeln!(file, "      </div>")?;
                    writeln!(file, "    </div>")?;
                }
                "Cluster Stability" => {
                    writeln!(file, "    <div class=\"metric-box\">")?;
                    writeln!(file, "      <div class=\"metric-name\">✓ Cluster Stability - Memory Organization</div>")?;
                    writeln!(file, "      <div class=\"metric-description\">")?;
                    writeln!(file, "        <strong>Primitives involved:</strong> {} semantic belief clusters<br>", self.count_total_clusters())?;
                    writeln!(file, "        <strong>Supporting data:</strong> Average {:.2} clusters per entity, showing structured memory<br>", self.average_clusters_per_entity())?;
                    writeln!(file, "        <strong>How it worked:</strong> Beliefs organized into semantic clusters enable reasoning related memories grouped by meaning, not chaotic fragments.")?;
                    writeln!(file, "      </div>")?;
                    writeln!(file, "    </div>")?;
                }
                "Affective Strength" => {
                    writeln!(file, "    <div class=\"metric-box\">")?;
                    writeln!(file, "      <div class=\"metric-name\">✓ Affective Strength - Emotional Responsiveness *** CRITICAL ***</div>")?;
                    writeln!(file, "      <div class=\"metric-description\">")?;
                    writeln!(file, "        <strong>Primitives involved:</strong> Affective signals generated within belief clusters (-5 to +5 range)<br>", )?;
                    writeln!(file, "        <strong>Supporting data:</strong> Peak emotional magnitude: {:.4} shows genuine emotional capacity<br>", self.max_affective_signal())?;
                    writeln!(file, "        <strong>How it worked:</strong> System responds to experiences with emotional valence. <span class=\"critical\">Without emotions, there is no consciousness—things must matter emotionally for subjective experience to exist.</span>")?;
                    writeln!(file, "      </div>")?;
                    writeln!(file, "    </div>")?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Default for ConsciousnessAnalysis {
    fn default() -> Self {
        ConsciousnessAnalysis {
            metric_thresholds: HashMap::new(),
            metric_values: HashMap::new(),
            passed_metrics: Vec::new(),
            failed_metrics: Vec::new(),
            consciousness_score: 0.0,
            consciousness_achieved: false,
            reasoning: String::new(),
        }
    }
}

impl SimulationStep {
    pub fn new(step_number: u64, metrics: Metrics) -> Self {
        SimulationStep {
            step_number,
            timestamp: step_number as f32 * 0.01, // Assuming dt=0.01
            attractions: Vec::new(),
            attentions: Vec::new(),
            belief_clusters: Vec::new(),
            entity_positions: Vec::new(),
            entity_velocities: Vec::new(),
            entity_essence: Vec::new(),
            metrics,
        }
    }
}
