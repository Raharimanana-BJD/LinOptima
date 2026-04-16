import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

export function TracingTable({ input, data, currentZ }: any) {
  if (!data) return null;

  const calculateX2 = (
    a: string,
    b: string,
    rhs: string | number,
    x1: number,
  ) => {
    const valA = parseFloat(a);
    const valB = parseFloat(b);
    const valRhs = typeof rhs === "string" ? parseFloat(rhs) : rhs;

    if (valB === 0) return "--";
    const res = (valRhs - valA * x1) / valB;
    return res % 1 === 0 ? res.toString() : res.toFixed(2);
  };

  return (
    <div className="rounded-md border bg-card shadow-sm overflow-hidden">
      <Table>
        <TableHeader className="bg-muted/50">
          <TableRow className="h-10">
            <TableHead className="text-[10px] font-bold uppercase">
              Équation
            </TableHead>
            <TableHead className="text-center text-[10px] font-bold">
              (0, x₂)
            </TableHead>
            <TableHead className="text-center text-[10px] font-bold">
              (1, x₂)
            </TableHead>
            <TableHead className="text-center text-[10px] font-bold">
              (2, x₂)
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {input.constraints.map((c: any, i: number) => (
            <TableRow key={i} className="hover:bg-muted/20">
              <TableCell className="font-mono text-[11px] py-2">
                {c.x1}x₁ {parseFloat(c.x2) >= 0 ? "+" : ""} {c.x2}x₂ = {c.rhs}
              </TableCell>
              {[0, 1, 2].map((val) => (
                <TableCell
                  key={val}
                  className="text-center text-[11px] text-muted-foreground py-2"
                >
                  ({val}, {calculateX2(c.x1, c.x2, c.rhs, val)})
                </TableCell>
              ))}
            </TableRow>
          ))}
          {/* Ligne pour la fonction objectif (Isoprofit) */}
          <TableRow className="bg-primary/5">
            <TableCell className="font-mono text-[11px] font-bold text-primary">
              {input.objective.x1}x₁ + {input.objective.x2}x₂ ={" "}
              {currentZ.toFixed(1)}
            </TableCell>
            {[0, 1, 2].map((val) => (
              <TableCell
                key={val}
                className="text-center text-[11px] font-bold text-primary py-2"
              >
                ({val},{" "}
                {calculateX2(
                  input.objective.x1,
                  input.objective.x2,
                  currentZ,
                  val,
                )}
                )
              </TableCell>
            ))}
          </TableRow>
        </TableBody>
      </Table>
    </div>
  );
}
