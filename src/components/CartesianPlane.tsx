import React, { useMemo } from "react";
import * as d3 from "d3";

interface PlaneProps {
  width: number;
  height: number;
  boundingBox: { minX1: number; maxX1: number; minX2: number; maxX2: number };
  children: (scales: {
    xScale: d3.ScaleLinear<number, number>;
    yScale: d3.ScaleLinear<number, number>;
  }) => React.ReactNode;
}

export function CartesianPlane({
  width,
  height,
  boundingBox,
  children,
}: PlaneProps) {
  const margin = { top: 30, right: 30, bottom: 50, left: 60 };
  const innerWidth = width - margin.left - margin.right;
  const innerHeight = height - margin.top - margin.bottom;

  const xScale = useMemo(
    () =>
      d3
        .scaleLinear()
        .domain([boundingBox.minX1, boundingBox.maxX1])
        .range([0, innerWidth]),
    [boundingBox.minX1, boundingBox.maxX1, innerWidth],
  );

  const yScale = useMemo(
    () =>
      d3
        .scaleLinear()
        .domain([boundingBox.minX2, boundingBox.maxX2])
        .range([innerHeight, 0]),
    [boundingBox.minX2, boundingBox.maxX2, innerHeight],
  );

  return (
    <svg width={width} height={height} className="overflow-visible select-none">
      <g transform={`translate(${margin.left},${margin.top})`}>
        {/* Grille secondaire (Muted) */}
        <g className="stroke-muted/30">
          {xScale.ticks(10).map((t) => (
            <line key={t} x1={xScale(t)} x2={xScale(t)} y2={innerHeight} />
          ))}
          {yScale.ticks(10).map((t) => (
            <line key={t} y1={yScale(t)} y2={yScale(t)} x2={innerWidth} />
          ))}
        </g>

        {/* Axes principaux (Foreground) */}
        <g className="stroke-foreground/50" strokeWidth="1.5">
          <line x1={0} y1={innerHeight} x2={innerWidth} y2={innerHeight} />
          <line x1={0} y1={0} x2={0} y2={innerHeight} />
        </g>

        {/* Graduations et Textes */}
        <g className="fill-muted-foreground text-[11px] font-mono">
          {xScale.ticks(5).map((t) => (
            <text
              key={t}
              x={xScale(t)}
              y={innerHeight + 20}
              textAnchor="middle"
            >
              {t}
            </text>
          ))}
          {yScale.ticks(5).map((t) => (
            <text
              key={t}
              x={-15}
              y={yScale(t)}
              textAnchor="end"
              alignmentBaseline="middle"
            >
              {t}
            </text>
          ))}
          <text
            x={innerWidth}
            y={innerHeight + 40}
            textAnchor="end"
            className="font-bold fill-foreground"
          >
            x₁
          </text>
          <text
            x={-40}
            y={-10}
            textAnchor="start"
            className="font-bold fill-foreground"
          >
            x₂
          </text>
        </g>

        {children({ xScale, yScale })}
      </g>
    </svg>
  );
}
