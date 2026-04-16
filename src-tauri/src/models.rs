use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OptimizationSense {
    Max,
    Min,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConstraintRelation {
    LessOrEqual,
    GreaterOrEqual,
    Equal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SolutionStatus {
    Optimal,
    Infeasible,
    Unbounded,
}

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
    // Indexes reference the augmented list of constraints:
    // user constraints first, then x1 >= 0 and x2 >= 0.
    pub active_constraints: Vec<usize>,
    pub bounding_box: BoundingBox,
    pub optimum: Option<OptimalSolution>,
    pub domain_bounded: bool,
    pub message: String,
}
