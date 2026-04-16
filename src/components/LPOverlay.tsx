import { useMemo, useState, useRef } from "react";

const MARGIN = { top: 30, right: 30, bottom: 50, left: 60 };
const WIDTH = 800;
const HEIGHT = 500;
const INNER_WIDTH = WIDTH - MARGIN.left - MARGIN.right;
const INNER_HEIGHT = HEIGHT - MARGIN.top - MARGIN.bottom;

export function LPOverlay({ data, xScale, yScale, currentZ, input }: any) {
  const [hoverData, setHoverData] = useState<{
    line: any;
    x: number;
    y: number;
  } | null>(null);
  const containerRef = useRef<SVGGElement>(null);

  if (!data) return null;

  const handleMouseMove = (e: React.MouseEvent, line: any) => {
    if (!containerRef.current) return;
    const CTM = containerRef.current.getScreenCTM();
    if (!CTM) return;
    const mouseX = (e.clientX - CTM.e) / CTM.a;
    const mouseY = (e.clientY - CTM.f) / CTM.d;
    setHoverData({ line, x: mouseX, y: mouseY });
  };

  const isoprofitLine = useMemo(() => {
    if (currentZ === undefined || !input?.objective || !data.boundingBox)
      return null;
    const objX1 = parseFloat(input.objective.x1) || 0;
    const objX2 = parseFloat(input.objective.x2) || 0;
    const bbox = data.boundingBox;
    if (objX1 === 0 && objX2 === 0) return null;

    if (objX2 !== 0) {
      return {
        p1: { x: bbox.minX1, y: (currentZ - objX1 * bbox.minX1) / objX2 },
        p2: { x: bbox.maxX1, y: (currentZ - objX1 * bbox.maxX1) / objX2 },
      };
    } else {
      return {
        p1: { x: currentZ / objX1, y: bbox.minX2 },
        p2: { x: currentZ / objX1, y: bbox.maxX2 },
      };
    }
  }, [currentZ, input, data.boundingBox]);

  return (
    <g ref={containerRef}>
      <defs>
        <pattern
          id="hatchPattern"
          patternUnits="userSpaceOnUse"
          width="8"
          height="8"
          patternTransform="rotate(45)"
        >
          <line
            x1="0"
            y1="0"
            x2="0"
            y2="8"
            stroke="#cbd5e1"
            strokeWidth="0.5"
          />
        </pattern>
        <clipPath id="chartClip">
          <rect x="0" y="0" width={INNER_WIDTH} height={INNER_HEIGHT} />
        </clipPath>
      </defs>

      <g clipPath="url(#chartClip)">
        {/* 1. Zones hachurées */}
        {data.hatchAreas?.map((area: any, i: number) => (
          <polygon
            key={`hatch-${i}`}
            points={area.points
              .map((p: any) => `${xScale(p.x1.approx)},${yScale(p.x2.approx)}`)
              .join(" ")}
            fill="url(#hatchPattern)"
            className="opacity-70"
          />
        ))}

        {/* 2. Zone admissible S */}
        {data.vertices?.length > 0 && (
          <polygon
            points={data.vertices
              .map((v: any) => `${xScale(v.x1.approx)},${yScale(v.x2.approx)}`)
              .join(" ")}
            className="fill-primary/5 stroke-red-600 stroke-[2.5]"
          />
        )}

        {/* 3. Ligne Isoprofit Z */}
        {isoprofitLine && (
          <line
            x1={xScale(isoprofitLine.p1.x)}
            y1={yScale(isoprofitLine.p1.y)}
            x2={xScale(isoprofitLine.p2.x)}
            y2={yScale(isoprofitLine.p2.y)}
            className="stroke-orange-500 stroke-2 transition-all"
            strokeDasharray="6,3"
          />
        )}

        {/* 4. Contraintes : hitbox + ligne + label Dx regroupés */}
        {data.lines?.map((line: any, i: number) => {
          const isHovered = hoverData?.line.label === line.label;
          const labelX = xScale(line.p2.x1.approx) - 15;
          const labelY = yScale(line.p2.x2.approx) - 10;

          return (
            <g key={`line-group-${i}`}>
              {/* Hitbox invisible large */}
              <line
                x1={xScale(line.p1.x1.approx)}
                y1={yScale(line.p1.x2.approx)}
                x2={xScale(line.p2.x1.approx)}
                y2={yScale(line.p2.x2.approx)}
                stroke="transparent"
                strokeWidth="15"
                onMouseMove={(e) => handleMouseMove(e, line)}
                onMouseLeave={() => setHoverData(null)}
                className="cursor-crosshair"
              />
              {/* Ligne réelle */}
              <line
                x1={xScale(line.p1.x1.approx)}
                y1={yScale(line.p1.x2.approx)}
                x2={xScale(line.p2.x1.approx)}
                y2={yScale(line.p2.x2.approx)}
                className={`pointer-events-none transition-all ${
                  isHovered
                    ? "stroke-blue-500 stroke-3"
                    : "stroke-slate-900 stroke-[1.5]"
                }`}
              />
              {/* Label Dx intégré au groupe */}
              <text
                x={labelX}
                y={labelY}
                className="fill-slate-400 text-[10px] font-mono italic font-bold pointer-events-none"
              >
                D{i + 1}
              </text>
            </g>
          );
        })}

        {/* 5. Point optimum */}
        {data.optimum && (
          <circle
            cx={xScale(data.optimum.point.x1.approx)}
            cy={yScale(data.optimum.point.x2.approx)}
            r={6}
            className="fill-red-600 stroke-white stroke-2"
          />
        )}
      </g>

      {/* Tooltip flottant — HORS du clipPath pour ne pas être rogné */}
      {hoverData && (
        <g
          transform={`translate(${hoverData.x + 10}, ${hoverData.y - 65})`}
          className="pointer-events-none"
        >
          {/* Ombre */}
          <rect
            width="145"
            height="55"
            rx="8"
            fill="rgba(0,0,0,0.1)"
            transform="translate(2,2)"
          />
          {/* Fond */}
          <rect width="145" height="55" rx="8" className="fill-slate-900" />
          <text
            x="12"
            y="22"
            className="fill-blue-400 text-[11px] font-bold uppercase tracking-wider"
          >
            Contrainte {hoverData.line.label.split(":")[0]}
          </text>
          <text
            x="12"
            y="42"
            className="fill-white text-[13px] font-mono font-medium"
          >
            {hoverData.line.label.split(":")[1]}
          </text>
          {/* Indicateur curseur */}
          <circle
            cx="-10"
            cy="65"
            r="4"
            className="fill-blue-600 stroke-white stroke-2"
          />
        </g>
      )}
    </g>
  );
}
