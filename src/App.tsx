import { useState, useEffect } from "react";
import { useSolver } from "./hooks/useSolver";
import { InputPanel } from "./components/InputPanel";
import { CartesianPlane } from "./components/CartesianPlane";
import { LPOverlay } from "./components/LPOverlay";
import { TracingTable } from "./components/TracingTable"; // Nouveau composant
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Slider } from "@/components/ui/slider";
import "./App.css";

const INITIAL_INPUT = {
  objective: { sense: "max", x1: "1", x2: "2" },
  constraints: [
    { x1: "4", x2: "-3", relation: "lessOrEqual", rhs: "2" },
    { x1: "-2", x2: "1", relation: "lessOrEqual", rhs: "1" },
    { x1: "-6", x2: "14", relation: "lessOrEqual", rhs: "35" },
  ],
};

function App() {
  const [input, setInput] = useState(INITIAL_INPUT);
  const { data, loading } = useSolver(input);
  const [currentZ, setCurrentZ] = useState(0);

  // Réinitialise le slider à 0 ou à une fraction de l'optimum quand les données changent
  useEffect(() => {
    if (data?.optimum) {
      setCurrentZ(Math.floor(data.optimum.objectiveValue.approx / 2));
    }
  }, [data]);

  const isOutOfDomain =
    data?.optimum && currentZ > data.optimum.objectiveValue.approx;

  return (
    <main className="min-h-screen bg-background text-foreground p-6">
      <header className="mb-8 flex items-baseline justify-between border-b pb-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight text-slate-900">
            LinOptima <span className="text-primary/60 font-light">v1.0</span>
          </h1>
          <p className="text-sm text-muted-foreground">
            Optimisation Linéaire & Visualisation Géométrique
          </p>
        </div>
        <div className="flex gap-2">
          <div className="flex items-center gap-2 px-3 py-1 bg-green-50 border border-green-100 rounded-full">
            <div className="w-2 h-2 bg-primary rounded-full animate-pulse" />
            <span className="text-[10px] font-bold text-primary uppercase">
              Moteur Rust Actif
            </span>
          </div>
        </div>
      </header>
      <div className="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* Colonne Gauche : Inputs + Tableau */}
        <div className="lg:col-span-4 space-y-6">
          <InputPanel input={input} setInput={setInput} />

          <TracingTable input={input} data={data} currentZ={currentZ} />

          {data?.message && (
            <div className="p-4 rounded-lg bg-muted text-muted-foreground text-xs font-mono border">
              {data.message}
            </div>
          )}
        </div>

        {/* Colonne Droite : Graphique + Slider */}
        <div className="lg:col-span-8 space-y-4">
          <Card className="border-none bg-card shadow-xl overflow-hidden">
            <CardHeader className="border-b bg-muted/30">
              <CardTitle className="text-sm font-mono uppercase tracking-widest text-muted-foreground flex justify-between">
                Visualisation Isoprofit
                {loading && (
                  <span className="animate-pulse text-primary text-xs">
                    Calcul...
                  </span>
                )}
              </CardTitle>
            </CardHeader>
            <CardContent className="flex items-center justify-center min-h-125 p-0 bg-white/50">
              {data ? (
                <CartesianPlane
                  width={800}
                  height={500}
                  boundingBox={data.boundingBox}
                >
                  {(scales) => (
                    <LPOverlay
                      data={data}
                      input={input}
                      currentZ={currentZ}
                      {...scales}
                    />
                  )}
                </CartesianPlane>
              ) : (
                <div className="text-muted-foreground animate-pulse font-mono text-sm">
                  Chargement du moteur Rust...
                </div>
              )}
            </CardContent>
          </Card>

          {/* Contrôle du Slider de Profit */}
          <Card className="p-6 border-dashed border-2">
            <div className="flex justify-between items-center mb-6 font-mono text-sm">
              <div className="flex items-center gap-3">
                <span className="px-2 py-1 bg-primary text-primary-foreground rounded">
                  Z
                </span>
                <span className="text-lg font-bold">{currentZ.toFixed(1)}</span>
              </div>
              {isOutOfDomain && (
                <span className="text-destructive font-bold animate-bounce text-xs bg-destructive/10 px-2 py-1 rounded">
                  ⚠️ PROFIT INATTEIGNABLE
                </span>
              )}
            </div>
            <Slider
              value={[currentZ]}
              min={0}
              max={
                data?.optimum ? data.optimum.objectiveValue.approx * 1.5 : 20
              }
              step={0.1}
              onValueChange={(val) => setCurrentZ(val[0])}
              className="py-4"
            />
          </Card>
        </div>
      </div>
    </main>
  );
}

export default App;
