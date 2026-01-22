# Autonomous-Infrastructure-Risk
Attempting to identify and mitigate infrastructure risk utilizing only NLP techniques.

--- 

## Project Overview

Policy and institutional decision-making often relies on large volumes of written reports—incident logs, compliance narratives, audit summaries—that are rarely structured for statistical modeling. This project demonstrates how **interpretable NLP techniques** can extract meaningful risk signals from text to support data-driven policy decisions. This project combines manual causal inference and systems-level analysis to study AI reliability and failure modes in autonomous space operations.

**Key Philosophy**: Clarity, reproducibility, and transparency over black-box complexity.

---

## Motivation

This project addresses a real-world challenge: **How do we systematically assess risk from unstructured policy documents?**

By combining natural language processing with transparent machine learning, this pipeline:
- Extracts interpretable features from text (TF-IDF, lexical markers, metadata)
- Builds explainable classification models
- Evaluates performance with policy-relevant metrics
- Demonstrates the practical tradeoffs between precision and recall in imbalanced settings

---

## Key Features

- **Transparent Feature Engineering**: TF-IDF, sentiment encoding, capacity utilization, and custom lexical features
- **Multiple Model Comparison**: Logistic Regression, Random Forest, Gradient Boosting—all with proper class imbalance handling
- **Policy-Focused Evaluation**: Emphasis on false negatives vs. false positives, not just accuracy
- **Comprehensive Diagnostics**: Built-in tools to identify and fix common modeling issues
- **Mars-Themed Visualizations**: 7 publication-ready charts with unique aesthetic
- **End-to-End Reproducibility**: Clone, run, and reproduce all results

---

## Results Summary

### Best Model Performance
- **Model**: Gradient Boosting with Sample Weights
- **ROC-AUC**: 0.91
- **F1-Score**: 0.52
- **Recall**: 0.57 (critical for catching high-risk cases)

### Key Insights
1. **Class Imbalance Matters**: High accuracy (~94%) masks the real challenge—detecting minority high-risk cases
2. **Feature Engineering is Critical**: TF-IDF and metadata features dramatically outperform simple keyword counts
3. **Policy Tradeoffs**: Conservative thresholds catch more risks but generate false alarms; aggressive thresholds reduce alarms but miss risks

---

## Installation

### Prerequisites
- Python 3.9 or higher
- pip package manager

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/policy-risk-inference.git
cd policy-risk-inference

# Install dependencies
pip install -r requirements.txt
```

### Required Packages
```
pandas
numpy
scikit-learn
matplotlib
seaborn
imbalanced-learn
jupyter
```

---

## Pipeline Walkthrough

### Step 1: Create Target Variable
```bash
python 00_create_target_variable.py
```
Generates risk labels from multiple signals (sentiment, load factor, capacity). Creates a realistic imbalanced dataset (~15% high-risk).

### Step 2: Language Analysis
```bash
python 02_report_language_analysis.py
```
Extracts and analyzes text features. Generates visualizations showing language patterns by risk level.

### Step 3: Model Training
```bash
python 03_risk_inference_model.py
```
Trains baseline models (Logistic Regression, Random Forest) and evaluates with confusion matrices, ROC curves, and classification reports.

### Step 4: Policy Comparison
```bash
python 04_policy_comparison.py
```
Simulates three decision thresholds (Conservative, Balanced, Aggressive) and analyzes policy implications.

### Step 5: Diagnostic & Fix
```bash
python 05_model_diagnostic_and_fix.py
```
Runs comprehensive diagnostics to identify issues (class imbalance, feature quality, model collapse) and applies fixes (SMOTE, class weights, undersampling).

### Step 6: Enhanced Modeling
```bash
python 06_complete_model_fix.py
```
**Main pipeline**: Performs advanced feature engineering, trains 4 models, selects the best, and saves everything for deployment.

### Step 7: Visualizations
```bash
python 07_mars_themed_visualizations.py
```
Generates 7 publication-ready Mars-themed visualizations showcasing model performance, feature importance, and NLP insights.

---

## Visualizations

The project includes seven professional visualizations with a distinctive Mars color scheme:

1. **Class Distribution**: Risk label balance analysis
2. **Feature Importance**: Top features driving predictions
3. **ROC Curves**: Model discrimination comparison
4. **F1 vs AUC**: Understanding metric tradeoffs
5. **Confusion Matrix**: Detailed error breakdown
6. **Text Features**: NLP analysis by risk level
7. **Performance Radar**: Multi-metric model comparison

All visualizations saved to `figures/mars_theme/`

---

## Educational Value

### What This Project Demonstrates

**NLP Fundamentals**
- Text preprocessing and cleaning
- TF-IDF feature extraction
- N-gram analysis
- Lexical feature engineering

**Machine Learning**
- Handling severe class imbalance (SMOTE, class weights, undersampling)
- Model comparison and selection
- Hyperparameter considerations
- Evaluation beyond accuracy

**Data Science Best Practices**
- Reproducible pipelines
- Comprehensive diagnostics
- Clear documentation
- Interpretable results

**Policy Applications**
- Threshold optimization
- False positive vs. false negative tradeoffs
- Cost-sensitive decision making
- Human-in-the-loop considerations

---

## Manual Causal Analysis (Library-Free)

To ensure full transparency and reproducibility, this project implements manual causal inference techniques rather than relying on specialized causal libraries (e.g., EconML or DoWhy).

Key components include:

Naïve ATE estimation to establish a baseline relationship between system load and overcapacity events

Stratified ATE estimation to control for heterogeneity across operational regimes

A manual Double Machine Learning (DML)–inspired workflow, using residualization and cross-fitting logic implemented directly in NumPy and pandas

This approach avoids dependency issues, clarifies assumptions, and emphasizes conceptual understanding of causal mechanisms—particularly important for safety-critical domains like space operations.

---

## Failure Modes of AI in Space Operations

Beyond estimating causal effects, this project analyzes four fundamental failure modes of AI systems operating in autonomous, high-risk environments:

Distributional Blindness
Average effects mask rare but catastrophic system states.

Load–Capacity Interaction Collapse
Failures emerge from variable interactions rather than single thresholds.

Delayed Feedback Instability
Risk accelerates faster than AI systems can adapt using historical feedback.

Policy Overgeneralization
Global decision rules fail in heterogeneous operational regimes.

Each failure mode is supported by a dedicated visualization, along with a final state-space synthesis plot that highlights where AI systems are most vulnerable. Together, these analyses demonstrate why causal reasoning is essential for reliable AI deployment in space and other safety-critical systems.

---

## Design Philosophy

### Why Simple Models?

This project deliberately uses **interpretable models** (Logistic Regression, Random Forest, Gradient Boosting) rather than deep learning because:

1. **Transparency**: Stakeholders can understand *why* a report is flagged as high-risk
2. **Debuggability**: Feature importance reveals what drives predictions
3. **Resource Efficiency**: Fast training and inference
4. **Generalization**: Simple models often perform better on small datasets

### Why Simulated Data?

Real policy reports are sensitive and proprietary. Simulation allows:
- **Open sharing** of methodology and code
- **Controlled experiments** with known ground truth
- **Reproducibility** without data access barriers
- **Demonstration** of techniques applicable to real-world scenarios

---

## Future Extensions

- **Cost-Sensitive Learning**: Assign different costs to false positives vs. false negatives
- **Temporal Analysis**: Detect risk drift over time
- **Human-in-the-Loop**: Simulation of expert review workflows
- **Advanced NLP**: BERT embeddings, topic modeling, named entity recognition
- **Threshold Optimization**: Automated selection under policy constraints
- **Active Learning**: Prioritize which reports to manually review

---

## Author

**William V. Fullerton**  
Statistics & Data Science | Finance Minor  
Graduated December 2025

This project serves as a **portfolio centerpiece** demonstrating:
- Statistical thinking
- NLP fundamentals
- ML engineering
- Research hygiene
- Communication skills

---

## Acknowledgments

- Inspired by real-world challenges in policy analytics and institutional risk management
- Built with standard Python data science libraries
- Mars-themed visualizations designed for maximum visual impact

---

** If you find this project useful, please consider giving it a star!**

*Built with ❤️ for transparent, interpretable, and policy-relevant data science.*
