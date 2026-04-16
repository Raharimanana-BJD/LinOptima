import { useState } from "react";
import { useSolver } from "./hooks/useSolver";
import { InputPanel } from "./components/InputPanel";
import { CartesianPlane } from "./components/CartesianPlane";
import { LPOverlay } from "./components/LPOverlay";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
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

  return (
    <main className="min-h-screen bg-background text-foreground p-6">
      <div className="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* Panneau de contrôle */}
        <div className="lg:col-span-4">
          <InputPanel input={input} setInput={setInput} />

          {data?.message && (
            <div className="mt-4 p-4 rounded-lg bg-muted text-muted-foreground text-sm font-mono border">
              {data.message}
            </div>
          )}
        </div>

        {/* Visualisation Graphique */}
        <Card className="lg:col-span-8 border-none bg-card shadow-xl overflow-hidden">
          <CardHeader className="border-b bg-muted/30">
            <CardTitle className="text-sm font-mono uppercase tracking-widest text-muted-foreground flex justify-between">
              Représentation Géométrique
              {loading && (
                <span className="animate-pulse text-primary">
                  Calcul en cours...
                </span>
              )}
            </CardTitle>
          </CardHeader>
          <CardContent className="flex items-center justify-center min-h-125 p-0">
            {data ? (
              <CartesianPlane
                width={800}
                height={500}
                boundingBox={data.boundingBox}
              >
                {(scales) => <LPOverlay data={data} {...scales} />}
              </CartesianPlane>
            ) : (
              <div className="text-muted-foreground animate-pulse">
                Initialisation du domaine...
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </main>
  );
}

export default App;
