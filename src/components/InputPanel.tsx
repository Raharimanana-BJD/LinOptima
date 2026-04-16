import { Plus, Trash2, Calculator } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

/**
 * Composant de saisie pour LinOptima.
 * Gère dynamiquement l'objectif et les contraintes du PL.
 */
export function InputPanel({ input, setInput }: any) {
  const addConstraint = () => {
    setInput({
      ...input,
      constraints: [
        ...input.constraints,
        { x1: "0", x2: "0", relation: "lessOrEqual", rhs: "0" },
      ],
    });
  };

  const updateConstraint = (index: number, field: string, value: string) => {
    const newConstraints = [...input.constraints];
    newConstraints[index] = { ...newConstraints[index], [field]: value };
    setInput({ ...input, constraints: newConstraints });
  };

  const removeConstraint = (index: number) => {
    const newConstraints = input.constraints.filter(
      (_: any, i: number) => i !== index,
    );
    setInput({ ...input, constraints: newConstraints });
  };

  return (
    <Card className="border-none shadow-2xl">
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-4">
        <CardTitle className="text-lg font-mono flex items-center gap-2">
          <Calculator className="w-5 h-5" />
          Modélisation du Problème
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        {/* Section Objectif */}
        <div className="space-y-3">
          <label className="text-xs uppercase tracking-widest font-bold">
            Fonction Objectif (Z)
          </label>
          <div className="flex items-center gap-3">
            <Select
              value={input.objective.sense}
              onValueChange={(v) =>
                setInput({
                  ...input,
                  objective: { ...input.objective, sense: v },
                })
              }
            >
              <SelectTrigger className="w-25">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="max">MAX</SelectItem>
                <SelectItem value="min">MIN</SelectItem>
              </SelectContent>
            </Select>
            <Input
              className="w-24"
              value={input.objective.x1}
              onChange={(e) =>
                setInput({
                  ...input,
                  objective: { ...input.objective, x1: e.target.value },
                })
              }
            />
            <span className=" font-mono">x₁ +</span>
            <Input
              className="w-24 "
              value={input.objective.x2}
              onChange={(e) =>
                setInput({
                  ...input,
                  objective: { ...input.objective, x2: e.target.value },
                })
              }
            />
            <span className=" font-mono">x₂</span>
          </div>
        </div>

        {/* Section Contraintes */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <label className="text-xs uppercase tracking-widest  font-bold">
              Contraintes Linéaires
            </label>
            <Button
              size="sm"
              variant="outline"
              onClick={addConstraint}
              className="h-8"
            >
              <Plus className="w-4 h-4 mr-1" /> Ajouter
            </Button>
          </div>

          <div className="space-y-3 max-h-100 overflow-y-auto pr-2 scrollbar-thin scrollbar-thumb-slate-700">
            {input.constraints.map((c: any, i: number) => (
              <div
                key={i}
                className="flex items-center gap-2 group animate-in fade-in slide-in-from-left-2 duration-200"
              >
                <Input
                  className="w-16"
                  value={c.x1}
                  onChange={(e) => updateConstraint(i, "x1", e.target.value)}
                />
                <span className="text-slate-500">x₁ +</span>
                <Input
                  className="w-16"
                  value={c.x2}
                  onChange={(e) => updateConstraint(i, "x2", e.target.value)}
                />
                <span className="text-slate-500">x₂</span>

                <Select
                  value={c.relation}
                  onValueChange={(v) => updateConstraint(i, "relation", v)}
                >
                  <SelectTrigger className="w-17.5">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="lessOrEqual">≤</SelectItem>
                    <SelectItem value="greaterOrEqual">≥</SelectItem>
                    <SelectItem value="equal">=</SelectItem>
                  </SelectContent>
                </Select>

                <Input
                  className="w-20"
                  value={c.rhs}
                  onChange={(e) => updateConstraint(i, "rhs", e.target.value)}
                />

                <Button
                  variant="ghost"
                  size="icon"
                  onClick={() => removeConstraint(i)}
                  className="text-slate-500 hover:text-red-400 hover:bg-red-400/10 transition-all opacity-0 group-hover:opacity-100"
                >
                  <Trash2 className="w-4 h-4" />
                </Button>
              </div>
            ))}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
