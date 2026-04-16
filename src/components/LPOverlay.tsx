export function LPOverlay({ data, xScale, yScale }: any) {
  // On ne dessine rien si le système est infaisable
  if (!data || data.status === "infeasible" || data.vertices.length === 0)
    return null;

  const polyPoints = data.vertices
    .map((v: any) => `${xScale(v.x1.approx)},${yScale(v.x2.approx)}`)
    .join(" ");

  return (
    <g>
      {/* Domaine admissible S - Utilise la couleur Primary avec transparence */}
      <polygon
        points={polyPoints}
        className="fill-primary/20 stroke-primary stroke-2 transition-all duration-300"
      />

      {/* Sommets (Points d'intersection admissibles) */}
      {data.vertices.map((v: any, i: number) => (
        <circle
          key={i}
          cx={xScale(v.x1.approx)}
          cy={yScale(v.x2.approx)}
          r={4}
          className="fill-background stroke-primary stroke-2"
        />
      ))}

      {/* Point Optimal - Mis en évidence si présent */}
      {data.optimum && (
        <g className="animate-in fade-in zoom-in duration-500">
          <circle
            cx={xScale(data.optimum.point.x1.approx)}
            cy={yScale(data.optimum.point.x2.approx)}
            r={6}
            className="fill-primary shadow-lg"
          />
          <circle
            cx={xScale(data.optimum.point.x1.approx)}
            cy={yScale(data.optimum.point.x2.approx)}
            r={12}
            className="stroke-primary fill-none animate-ping opacity-50"
          />
        </g>
      )}
    </g>
  );
}
