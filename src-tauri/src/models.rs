use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OptimizationSense { Max, Min }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConstraintRelation { LessOrEqual, GreaterOrEqual, Equal }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SolutionStatus { Optimal, Infeasible, Unbounded }

// --- STRUCTURES D'ENTRÉE ---

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveInput {
    pub sense: OptimizationSense,
    pub x1: String,
    pub x2: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintInput {
    pub x1: String,
    pub x2: String,
    pub relation: ConstraintRelation,
    pub rhs: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearProgramInput {
    pub objective: ObjectiveInput,
    pub constraints: Vec<ConstraintInput>,
}

// --- STRUCTURES DE SORTIE ---
// f64 n'implémente pas Eq → on utilise seulement PartialEq ici

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RationalValue {
    pub numerator: i64,
    pub denominator: i64,
    pub exact: String,
    pub approx: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point2D {
    pub x1: RationalValue,
    pub x2: RationalValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HatchArea {
    pub points: Vec<Point2D>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineMetadata {
    pub label: String,
    pub p1: Point2D,
    pub p2: Point2D,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimalSolution {
    pub point: Point2D,
    pub objective_value: RationalValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    pub min_x1: f64,
    pub max_x1: f64,
    pub min_x2: f64,
    pub max_x2: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolveResponse {
    pub status: SolutionStatus,
    pub vertices: Vec<Point2D>,
    pub hatch_areas: Vec<HatchArea>,
    pub lines: Vec<LineMetadata>,
    pub bounding_box: BoundingBox,
    pub optimum: Option<OptimalSolution>,
    pub message: String,
}