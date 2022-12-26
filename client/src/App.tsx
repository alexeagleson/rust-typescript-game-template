import { useEffect, useRef, useState } from "react";
import { Log, HoverMenu } from "./components";
import { HoverMenuProps } from "./components/HoverMenu/HoverMenu";
import { initializeGame } from "./game/initialize";
import { PlayerDetails } from "./utility/types";
import "./App.css";

const App = () => {
  const initialized = useRef<boolean>(false);
  const canvasContainer = useRef<HTMLDivElement | null>(null);
  const logContainer = useRef<HTMLDivElement | null>(null);

  const [hoverMenu, setHoverMenu] = useState<HoverMenuProps>();
  const [log, setLog] = useState<string[]>([]);
  const [moveCount, setMoveCount] = useState<number>();

  const onHover = (x: number, y: number, playerDetails?: PlayerDetails) => {
    if (!playerDetails) {
      setHoverMenu(undefined);
    } else {
      setHoverMenu({ menuPosition: { x, y }, playerDetails });
    }
  };

  const onClick = (logEntry: string) => {
    setLog((oldLog) => [logEntry, ...oldLog]);
  };

  // Queries the server for the game configuration (to determine the canvas size)
  // and then initializes the game.  Will only fire once (due to `initialized` check)
  // so the game state will persist during Vite dev server hot reloading
  useEffect(() => {
    if (initialized.current === false) {
      initialized.current = true;
      initializeGame(onHover, onClick, setMoveCount).then(({ gameCanvas }) => {
        canvasContainer.current?.appendChild(gameCanvas);
        const canvasHeight = gameCanvas.height + "px";
        if (canvasContainer.current && logContainer.current) {
          canvasContainer.current.style.height = canvasHeight;
          logContainer.current.style.height = canvasHeight;
        }
      });
    }
  });

  return (
    <div className="game-container">
      <p>All time totals moves: {moveCount}</p>
      <div className="canvas-and-log-container">
        <div className="canvas-container" ref={canvasContainer}>
          {hoverMenu && <HoverMenu {...hoverMenu} />}
        </div>
        <div ref={logContainer} className="log-container">
          <Log log={log} />
        </div>
      </div>
    </div>
  );
};

export default App;
