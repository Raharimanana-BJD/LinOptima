use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use num_rational::Ratio;

use crate::models::{
    BoundingBox, ConstraintInput, ConstraintRelation, LinearProgramInput, ObjectiveInput,
    OptimalSolution, OptimizationSense, Point2D, RationalValue, SolutionStatus, SolveResponse,
};

type Rational = Ratio<i64>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolveError {
    InvalidNumber(String),
}

impl Display for SolveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNumber(value) => {
                write!(f, "Impossible de parser la valeur rationnelle `{value}`.")
            }
        }
    }
}

impl std::error::Error for SolveError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x1: Rational,
    x2: Rational,
}

#[derive(Debug, Clone)]
struct Direction {
    dx: Rational,
    dy: Rational,
}

#[derive(Debug, Clone)]
struct ParsedObjective {
    sense: OptimizationSense,
    x1: Rational,
    x2: Rational,
}

#[derive(Debug, Clone)]
struct ParsedConstraint {
    index: usize,
    a: Rational,
    b: Rational,
    rhs: Rational,
    relation: ConstraintRelation,
}

pub fn solve(problem: &LinearProgramInput) -> Result<SolveResponse, SolveError> {
    let objective = parse_objective(&problem.objective)?;
    let constraints = parse_constraints(&problem.constraints)?;
    let augmented_constraints = augment_with_non_negativity(&constraints);

    let feasible_points = collect_feasible_points(&augmented_constraints);
    if feasible_points.is_empty() {
        return Ok(SolveResponse {
            status: SolutionStatus::Infeasible,
            vertices: Vec::new(),
            active_constraints: Vec::new(),
            bounding_box: default_bounding_box(),
            optimum: None,
            domain_bounded: true,
            message: "Incoherence detectee : le systeme de contraintes ne definit aucun domaine admissible (S).".to_string(),
        });
    }

    let hull = monotone_chain(feasible_points);
    let domain_bounded = !has_feasible_ray(&augmented_constraints);
    let bounding_box = compute_bounding_box(&hull, &augmented_constraints, None, domain_bounded);

    if has_improving_ray(&objective, &augmented_constraints) {
        return Ok(SolveResponse {
            status: SolutionStatus::Unbounded,
            vertices: hull.iter().map(to_model_point).collect(),
            active_constraints: Vec::new(),
            bounding_box,
            optimum: None,
            domain_bounded,
            message:
                "Analyse terminee : la fonction objectif est non bornee sur le domaine admissible."
                    .to_string(),
        });
    }

    let optimum_point = select_optimum(&hull, &objective)
        .expect("a non-empty feasible hull must contain at least one optimum point");
    let objective_value = evaluate_objective(&objective, optimum_point);
    let active_constraints = augmented_constraints
        .iter()
        .filter(|constraint| is_saturated(constraint, optimum_point))
        .map(|constraint| constraint.index)
        .collect::<Vec<_>>();
    let optimum = OptimalSolution {
        point: to_model_point(optimum_point),
        objective_value: to_model_rational(&objective_value),
    };
    let bounding_box = compute_bounding_box(
        &hull,
        &augmented_constraints,
        Some(optimum_point),
        domain_bounded,
    );

    Ok(SolveResponse {
        status: SolutionStatus::Optimal,
        vertices: hull.iter().map(to_model_point).collect(),
        active_constraints,
        bounding_box,
        optimum: Some(optimum),
        domain_bounded,
        message: format!(
            "Analyse terminee. Le sommet optimal a ete identifie en (x1 : {} ; x2 : {}). La valeur optimale est Z = {}.",
            format_decimal(&optimum_point.x1),
            format_decimal(&optimum_point.x2),
            format_decimal(&objective_value)
        ),
    })
}

fn parse_objective(objective: &ObjectiveInput) -> Result<ParsedObjective, SolveError> {
    Ok(ParsedObjective {
        sense: objective.sense,
        x1: parse_rational(&objective.x1)?,
        x2: parse_rational(&objective.x2)?,
    })
}

fn parse_constraints(input: &[ConstraintInput]) -> Result<Vec<ParsedConstraint>, SolveError> {
    input
        .iter()
        .enumerate()
        .map(|(index, constraint)| {
            Ok(ParsedConstraint {
                index,
                a: parse_rational(&constraint.x1)?,
                b: parse_rational(&constraint.x2)?,
                rhs: parse_rational(&constraint.rhs)?,
                relation: constraint.relation,
            })
        })
        .collect()
}

fn augment_with_non_negativity(constraints: &[ParsedConstraint]) -> Vec<ParsedConstraint> {
    let mut augmented = constraints.to_vec();
    let zero = Rational::from_integer(0);
    let one = Rational::from_integer(1);
    let x1_index = augmented.len();
    let x2_index = x1_index + 1;

    augmented.push(ParsedConstraint {
        index: x1_index,
        a: one.clone(),
        b: zero.clone(),
        rhs: zero.clone(),
        relation: ConstraintRelation::GreaterOrEqual,
    });
    augmented.push(ParsedConstraint {
        index: x2_index,
        a: zero.clone(),
        b: one,
        rhs: zero,
        relation: ConstraintRelation::GreaterOrEqual,
    });

    augmented
}

fn collect_feasible_points(constraints: &[ParsedConstraint]) -> Vec<Point> {
    let mut points = Vec::new();

    for left in 0..constraints.len() {
        for right in (left + 1)..constraints.len() {
            if let Some(point) = intersect(&constraints[left], &constraints[right]) {
                if is_feasible(&point, constraints) {
                    points.push(point);
                }
            }
        }
    }

    points.sort_by(compare_points);
    points.dedup_by(|left, right| left == right);
    points
}

fn intersect(left: &ParsedConstraint, right: &ParsedConstraint) -> Option<Point> {
    let determinant = left.a.clone() * right.b.clone() - right.a.clone() * left.b.clone();
    if determinant == Rational::from_integer(0) {
        return None;
    }

    let x1 = (left.rhs.clone() * right.b.clone() - right.rhs.clone() * left.b.clone())
        / determinant.clone();
    let x2 =
        (left.a.clone() * right.rhs.clone() - right.a.clone() * left.rhs.clone()) / determinant;

    Some(Point { x1, x2 })
}

fn is_feasible(point: &Point, constraints: &[ParsedConstraint]) -> bool {
    constraints
        .iter()
        .all(|constraint| satisfies(constraint, point))
}

fn satisfies(constraint: &ParsedConstraint, point: &Point) -> bool {
    let lhs = constraint.a.clone() * point.x1.clone() + constraint.b.clone() * point.x2.clone();

    match constraint.relation {
        ConstraintRelation::LessOrEqual => lhs <= constraint.rhs,
        ConstraintRelation::GreaterOrEqual => lhs >= constraint.rhs,
        ConstraintRelation::Equal => lhs == constraint.rhs,
    }
}

fn is_saturated(constraint: &ParsedConstraint, point: &Point) -> bool {
    let lhs = constraint.a.clone() * point.x1.clone() + constraint.b.clone() * point.x2.clone();
    lhs == constraint.rhs
}

fn monotone_chain(mut points: Vec<Point>) -> Vec<Point> {
    if points.len() <= 1 {
        return points;
    }

    points.sort_by(compare_points);
    points.dedup_by(|left, right| left == right);

    let mut lower = Vec::new();
    for point in &points {
        while lower.len() >= 2
            && cross(&lower[lower.len() - 2], &lower[lower.len() - 1], point)
                <= Rational::from_integer(0)
        {
            lower.pop();
        }
        lower.push(point.clone());
    }

    let mut upper = Vec::new();
    for point in points.iter().rev() {
        while upper.len() >= 2
            && cross(&upper[upper.len() - 2], &upper[upper.len() - 1], point)
                <= Rational::from_integer(0)
        {
            upper.pop();
        }
        upper.push(point.clone());
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);

    if lower.is_empty() {
        points
    } else {
        lower
    }
}

fn cross(origin: &Point, a: &Point, b: &Point) -> Rational {
    let ax = a.x1.clone() - origin.x1.clone();
    let ay = a.x2.clone() - origin.x2.clone();
    let bx = b.x1.clone() - origin.x1.clone();
    let by = b.x2.clone() - origin.x2.clone();

    ax * by - ay * bx
}

fn compare_points(left: &Point, right: &Point) -> Ordering {
    compare_rationals(&left.x1, &right.x1).then_with(|| compare_rationals(&left.x2, &right.x2))
}

fn compare_rationals(left: &Rational, right: &Rational) -> Ordering {
    let lhs = (*left.numer() as i128) * (*right.denom() as i128);
    let rhs = (*right.numer() as i128) * (*left.denom() as i128);
    lhs.cmp(&rhs)
}

fn has_feasible_ray(constraints: &[ParsedConstraint]) -> bool {
    candidate_directions(constraints)
        .into_iter()
        .any(|direction| is_feasible_direction(&direction, constraints))
}

fn has_improving_ray(objective: &ParsedObjective, constraints: &[ParsedConstraint]) -> bool {
    candidate_directions(constraints)
        .into_iter()
        .any(|direction| {
            if !is_feasible_direction(&direction, constraints) {
                return false;
            }

            let delta = objective.x1.clone() * direction.dx + objective.x2.clone() * direction.dy;
            match objective.sense {
                OptimizationSense::Max => delta > Rational::from_integer(0),
                OptimizationSense::Min => delta < Rational::from_integer(0),
            }
        })
}

fn candidate_directions(constraints: &[ParsedConstraint]) -> Vec<Direction> {
    let zero = Rational::from_integer(0);
    let one = Rational::from_integer(1);
    let minus_one = Rational::from_integer(-1);

    let mut directions = vec![
        Direction {
            dx: one.clone(),
            dy: zero.clone(),
        },
        Direction {
            dx: zero.clone(),
            dy: one.clone(),
        },
        Direction {
            dx: minus_one.clone(),
            dy: zero.clone(),
        },
        Direction {
            dx: zero.clone(),
            dy: minus_one.clone(),
        },
    ];

    for constraint in constraints {
        if constraint.a == zero && constraint.b == zero {
            continue;
        }

        directions.push(Direction {
            dx: constraint.b.clone(),
            dy: -constraint.a.clone(),
        });
        directions.push(Direction {
            dx: -constraint.b.clone(),
            dy: constraint.a.clone(),
        });
    }

    directions
}

fn is_feasible_direction(direction: &Direction, constraints: &[ParsedConstraint]) -> bool {
    if direction.dx == Rational::from_integer(0) && direction.dy == Rational::from_integer(0) {
        return false;
    }

    constraints.iter().all(|constraint| {
        let delta = constraint.a.clone() * direction.dx.clone()
            + constraint.b.clone() * direction.dy.clone();
        match constraint.relation {
            ConstraintRelation::LessOrEqual => delta <= Rational::from_integer(0),
            ConstraintRelation::GreaterOrEqual => delta >= Rational::from_integer(0),
            ConstraintRelation::Equal => delta == Rational::from_integer(0),
        }
    })
}

fn select_optimum<'a>(points: &'a [Point], objective: &ParsedObjective) -> Option<&'a Point> {
    match objective.sense {
        OptimizationSense::Max => points.iter().max_by(|left, right| {
            compare_rationals(
                &evaluate_objective(objective, left),
                &evaluate_objective(objective, right),
            )
        }),
        OptimizationSense::Min => points.iter().min_by(|left, right| {
            compare_rationals(
                &evaluate_objective(objective, left),
                &evaluate_objective(objective, right),
            )
        }),
    }
}

fn evaluate_objective(objective: &ParsedObjective, point: &Point) -> Rational {
    objective.x1.clone() * point.x1.clone() + objective.x2.clone() * point.x2.clone()
}

fn to_model_point(point: &Point) -> Point2D {
    Point2D {
        x1: to_model_rational(&point.x1),
        x2: to_model_rational(&point.x2),
    }
}

fn to_model_rational(value: &Rational) -> RationalValue {
    RationalValue {
        numerator: *value.numer(),
        denominator: *value.denom(),
        exact: format_exact(value),
        approx: rational_to_f64(value),
    }
}

fn format_exact(value: &Rational) -> String {
    if *value.denom() == 1 {
        value.numer().to_string()
    } else {
        format!("{}/{}", value.numer(), value.denom())
    }
}

fn format_decimal(value: &Rational) -> String {
    let approx = rational_to_f64(value);
    let rounded = (approx * 1_000_000.0).round() / 1_000_000.0;
    let mut rendered = format!("{rounded:.6}");

    while rendered.contains('.') && rendered.ends_with('0') {
        rendered.pop();
    }
    if rendered.ends_with('.') {
        rendered.pop();
    }

    rendered
}

fn rational_to_f64(value: &Rational) -> f64 {
    (*value.numer() as f64) / (*value.denom() as f64)
}

fn compute_bounding_box(
    points: &[Point],
    constraints: &[ParsedConstraint],
    optimum: Option<&Point>,
    domain_bounded: bool,
) -> BoundingBox {
    if points.is_empty() {
        return default_bounding_box();
    }

    let zero = Rational::from_integer(0);
    let mut xs = vec![0.0_f64];
    let mut ys = vec![0.0_f64];

    for point in points {
        xs.push(rational_to_f64(&point.x1));
        ys.push(rational_to_f64(&point.x2));
    }

    if let Some(point) = optimum {
        xs.push(rational_to_f64(&point.x1));
        ys.push(rational_to_f64(&point.x2));
    }

    for constraint in constraints {
        if constraint.a != zero {
            let intercept = rational_to_f64(&(constraint.rhs.clone() / constraint.a.clone()));
            if intercept.is_finite() && intercept >= 0.0 {
                xs.push(intercept);
            }
        }

        if constraint.b != zero {
            let intercept = rational_to_f64(&(constraint.rhs.clone() / constraint.b.clone()));
            if intercept.is_finite() && intercept >= 0.0 {
                ys.push(intercept);
            }
        }
    }

    let mut max_x = xs.into_iter().fold(1.0_f64, f64::max);
    let mut max_y = ys.into_iter().fold(1.0_f64, f64::max);

    if domain_bounded {
        max_x = max_x * 1.15 + 1.0;
        max_y = max_y * 1.15 + 1.0;
    } else {
        max_x = (max_x * 1.5).max(10.0);
        max_y = (max_y * 1.5).max(10.0);
    }

    BoundingBox {
        min_x1: 0.0,
        max_x1: max_x,
        min_x2: 0.0,
        max_x2: max_y,
    }
}

fn default_bounding_box() -> BoundingBox {
    BoundingBox {
        min_x1: 0.0,
        max_x1: 10.0,
        min_x2: 0.0,
        max_x2: 10.0,
    }
}

fn parse_rational(value: &str) -> Result<Rational, SolveError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(SolveError::InvalidNumber(value.to_string()));
    }

    if let Some((numerator, denominator)) = trimmed.split_once('/') {
        let numerator = numerator
            .trim()
            .parse::<i64>()
            .map_err(|_| SolveError::InvalidNumber(value.to_string()))?;
        let denominator = denominator
            .trim()
            .parse::<i64>()
            .map_err(|_| SolveError::InvalidNumber(value.to_string()))?;
        if denominator == 0 {
            return Err(SolveError::InvalidNumber(value.to_string()));
        }

        return Ok(Ratio::new(numerator, denominator));
    }

    if trimmed.matches('.').count() == 1 {
        let sign = if trimmed.starts_with('-') {
            -1_i64
        } else {
            1_i64
        };
        let unsigned = trimmed.trim_start_matches(['+', '-']);
        let (whole, fractional) = unsigned
            .split_once('.')
            .ok_or_else(|| SolveError::InvalidNumber(value.to_string()))?;
        let whole = if whole.is_empty() { "0" } else { whole };
        let digits = format!("{whole}{fractional}");
        let numerator = digits
            .parse::<i64>()
            .map_err(|_| SolveError::InvalidNumber(value.to_string()))?;
        let denominator = 10_i64
            .checked_pow(fractional.len() as u32)
            .ok_or_else(|| SolveError::InvalidNumber(value.to_string()))?;

        return Ok(Ratio::new(sign * numerator, denominator));
    }

    trimmed
        .parse::<i64>()
        .map(Ratio::from_integer)
        .map_err(|_| SolveError::InvalidNumber(value.to_string()))
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::models::{
        ConstraintInput, ConstraintRelation, LinearProgramInput, ObjectiveInput, OptimizationSense,
        SolutionStatus,
    };

    fn build_problem(
        sense: OptimizationSense,
        objective_x1: &str,
        objective_x2: &str,
        constraints: Vec<(&str, &str, ConstraintRelation, &str)>,
    ) -> LinearProgramInput {
        LinearProgramInput {
            objective: ObjectiveInput {
                sense,
                x1: objective_x1.to_string(),
                x2: objective_x2.to_string(),
            },
            constraints: constraints
                .into_iter()
                .map(|(x1, x2, relation, rhs)| ConstraintInput {
                    x1: x1.to_string(),
                    x2: x2.to_string(),
                    relation,
                    rhs: rhs.to_string(),
                })
                .collect(),
        }
    }

    #[test]
    fn solves_the_blueprint_reference_case() {
        let problem = build_problem(
            OptimizationSense::Max,
            "1",
            "2",
            vec![
                ("4", "-3", ConstraintRelation::LessOrEqual, "2"),
                ("-2", "1", ConstraintRelation::LessOrEqual, "1"),
                ("-6", "14", ConstraintRelation::LessOrEqual, "35"),
            ],
        );

        let solution = solve(&problem).expect("the blueprint case should be solvable");

        assert_eq!(solution.status, SolutionStatus::Optimal);
        assert!(solution.domain_bounded);
        assert_eq!(solution.active_constraints, vec![0, 2]);
        assert_eq!(solution.vertices.len(), 5);

        let optimum = solution.optimum.expect("the optimum should be present");
        assert_eq!(optimum.point.x1.numerator, 7);
        assert_eq!(optimum.point.x1.denominator, 2);
        assert_eq!(optimum.point.x2.numerator, 4);
        assert_eq!(optimum.point.x2.denominator, 1);
        assert_eq!(optimum.objective_value.numerator, 23);
        assert_eq!(optimum.objective_value.denominator, 2);
        assert!((optimum.objective_value.approx - 11.5).abs() < 1e-9);
    }

    #[test]
    fn detects_infeasible_systems() {
        let problem = build_problem(
            OptimizationSense::Max,
            "1",
            "1",
            vec![
                ("1", "0", ConstraintRelation::LessOrEqual, "1"),
                ("1", "0", ConstraintRelation::GreaterOrEqual, "2"),
            ],
        );

        let solution = solve(&problem).expect("the infeasible case should return a response");

        assert_eq!(solution.status, SolutionStatus::Infeasible);
        assert!(solution.vertices.is_empty());
        assert!(solution.optimum.is_none());
    }

    #[test]
    fn detects_unbounded_objectives() {
        let problem = build_problem(
            OptimizationSense::Max,
            "1",
            "1",
            vec![("1", "-1", ConstraintRelation::Equal, "0")],
        );

        let solution = solve(&problem).expect("the unbounded case should return a response");

        assert_eq!(solution.status, SolutionStatus::Unbounded);
        assert!(!solution.domain_bounded);
        assert!(solution.optimum.is_none());
    }

    #[test]
    fn supports_vertical_constraints() {
        let problem = build_problem(
            OptimizationSense::Max,
            "1",
            "1",
            vec![
                ("1", "0", ConstraintRelation::LessOrEqual, "2"),
                ("0", "1", ConstraintRelation::LessOrEqual, "3"),
            ],
        );

        let solution = solve(&problem).expect("the vertical constraint case should be solvable");
        let optimum = solution.optimum.expect("the optimum should be present");

        assert_eq!(solution.status, SolutionStatus::Optimal);
        assert_eq!(optimum.point.x1.numerator, 2);
        assert_eq!(optimum.point.x2.numerator, 3);
        assert_eq!(optimum.objective_value.numerator, 5);
    }

    #[test]
    fn keeps_track_of_unbounded_domains_with_finite_optimum() {
        let problem = build_problem(OptimizationSense::Max, "-1", "-1", vec![]);

        let solution = solve(&problem).expect("the quadrant should remain solvable");
        let optimum = solution.optimum.expect("the optimum should be present");

        assert_eq!(solution.status, SolutionStatus::Optimal);
        assert!(!solution.domain_bounded);
        assert_eq!(optimum.point.x1.numerator, 0);
        assert_eq!(optimum.point.x2.numerator, 0);
        assert_eq!(optimum.objective_value.numerator, 0);
    }
}
