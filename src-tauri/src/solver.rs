use num_rational::Ratio;
use crate::models::*;

type Rational = Ratio<i64>;

#[derive(Debug, Clone)]
struct ParsedConstraint {
    a: Rational,
    b: Rational,
    rhs: Rational,
    relation: ConstraintRelation,
    label: String,
}

pub fn solve(problem: &LinearProgramInput) -> Result<SolveResponse, String> {
    let constraints = parse_constraints(problem)?;

    let bbox = BoundingBox {
        min_x1: -2.0,
        max_x1: 10.0,
        min_x2: -2.0,
        max_x2: 10.0,
    };

    let mut lines_metadata = Vec::new();
    let mut hatch_areas = Vec::new();

    for (i, c) in constraints.iter().enumerate() {
        let pts = get_line_bbox_intersections(c, &bbox);

        if pts.len() >= 2 {
            lines_metadata.push(LineMetadata {
                label: format!("D{}: {}", i + 1, c.label),
                p1: pts[0].clone(),
                p2: pts[1].clone(),
            });
            hatch_areas.push(compute_forbidden_area(c, &pts));
        }
    }

    let vertices = compute_feasible_region_vertices(&constraints);

    Ok(SolveResponse {
        status: SolutionStatus::Optimal,
        vertices,
        hatch_areas,
        lines: lines_metadata,
        bounding_box: bbox,
        optimum: None,
        message: "Visualisation géométrique générée avec succès.".into(),
    })
}

// --- LOGIQUE GÉOMÉTRIQUE ---

fn get_line_bbox_intersections(c: &ParsedConstraint, bbox: &BoundingBox) -> Vec<Point2D> {
    let mut points = Vec::new();
    let a = c.a.to_f64();
    let b = c.b.to_f64();
    let rhs = c.rhs.to_f64();

    // 1. Intersection avec les bords verticaux x1 = min / x1 = max
    for x1 in [bbox.min_x1, bbox.max_x1] {
        if b != 0.0 {
            let x2 = (rhs - a * x1) / b;
            if x2 >= bbox.min_x2 && x2 <= bbox.max_x2 {
                points.push(create_point(x1, x2));
            }
        }
    }

    // 2. Intersection avec les bords horizontaux x2 = min / x2 = max
    for x2 in [bbox.min_x2, bbox.max_x2] {
        if a != 0.0 {
            let x1 = (rhs - b * x2) / a;
            if x1 >= bbox.min_x1 && x1 <= bbox.max_x1 {
                points.push(create_point(x1, x2));
            }
        }
    }

    // TRI CRITIQUE : garantir un segment cohérent
    points.sort_by(|p1, p2| {
        p1.x1.approx
            .partial_cmp(&p2.x1.approx)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    points.dedup_by(|p1, p2| {
        (p1.x1.approx - p2.x1.approx).abs() < 1e-6
            && (p1.x2.approx - p2.x2.approx).abs() < 1e-6
    });

    points
}

fn create_point(x1: f64, x2: f64) -> Point2D {
    Point2D {
        x1: RationalValue {
            numerator: 0,
            denominator: 1,
            exact: x1.to_string(),
            approx: x1,
        },
        x2: RationalValue {
            numerator: 0,
            denominator: 1,
            exact: x2.to_string(),
            approx: x2,
        },
    }
}

fn compute_forbidden_area(c: &ParsedConstraint, line_pts: &[Point2D]) -> HatchArea {
    let mut points = vec![line_pts[0].clone(), line_pts[1].clone()];

    let sign = match c.relation {
        ConstraintRelation::LessOrEqual => 1.0,
        ConstraintRelation::GreaterOrEqual => -1.0,
        ConstraintRelation::Equal => 0.0,
    };

    let normal_x1 = c.a.to_f64() * sign;
    let normal_x2 = c.b.to_f64() * sign;

    points.push(Point2D {
        x1: to_val(f64_to_rational(line_pts[1].x1.approx + normal_x1 * 10.0)),
        x2: to_val(f64_to_rational(line_pts[1].x2.approx + normal_x2 * 10.0)),
    });
    points.push(Point2D {
        x1: to_val(f64_to_rational(line_pts[0].x1.approx + normal_x1 * 10.0)),
        x2: to_val(f64_to_rational(line_pts[0].x2.approx + normal_x2 * 10.0)),
    });

    HatchArea { points }
}

// --- HELPERS ---

fn parse_constraints(problem: &LinearProgramInput) -> Result<Vec<ParsedConstraint>, String> {
    problem.constraints.iter().map(|c| {
        Ok(ParsedConstraint {
            a: string_to_rational(&c.x1)?,
            b: string_to_rational(&c.x2)?,
            rhs: string_to_rational(&c.rhs)?,
            relation: c.relation,
            label: format!("{}x1 + {}x2 = {}", c.x1, c.x2, c.rhs),
        })
    }).collect()
}

fn string_to_rational(s: &str) -> Result<Rational, String> {
    s.parse::<i64>()
        .map(Ratio::from_integer)
        .map_err(|_| format!("Erreur de parsing: '{}'", s))
}

fn f64_to_rational(f: f64) -> Rational {
    Ratio::from_integer((f * 1000.0) as i64) / Ratio::from_integer(1000)
}

fn to_val(r: Rational) -> RationalValue {
    RationalValue {
        numerator: *r.numer(),
        denominator: *r.denom(),
        exact: format!("{}/{}", r.numer(), r.denom()),
        approx: *r.numer() as f64 / *r.denom() as f64,
    }
}

trait ToF64 {
    fn to_f64(&self) -> f64;
}
impl ToF64 for Rational {
    fn to_f64(&self) -> f64 {
        *self.numer() as f64 / *self.denom() as f64
    }
}

fn compute_feasible_region_vertices(_constraints: &[ParsedConstraint]) -> Vec<Point2D> {
    vec![]
}