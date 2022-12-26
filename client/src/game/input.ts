/** Handlers for user input (keyboard and mouse) */

import { TILE_SIZE } from "../utility/config";
import { ClientMessage, Key, Position, ServerMessage } from "../utility/types";
import { SafeSend } from "./connection";

export const addInputListeners = (
  gameCanvas: HTMLCanvasElement,
  updateHoverMenuPosition: (x: number, y: number) => void,
  safeSend: SafeSend
) => {
  const processMouseEvent = (e: MouseEvent) => {
    const rect = gameCanvas.getBoundingClientRect();
    const xPixel = e.clientX - rect.left;
    const yPixel = e.clientY - rect.top;

    const pixelPos: Position = { x: xPixel, y: yPixel };
    const tilePos: Position = {
      x: Math.trunc(pixelPos.x / TILE_SIZE),
      y: Math.trunc(pixelPos.y / TILE_SIZE),
    };

    return { tilePos, pixelPos };
  };

  gameCanvas.onmousemove = (e) => {
    const { tilePos, pixelPos } = processMouseEvent(e);
    updateHoverMenuPosition(pixelPos.x, pixelPos.y);
    safeSend({ type: "tileHover", content: tilePos });
  };

  gameCanvas.onmousedown = (e) => {
    const { tilePos } = processMouseEvent(e);
    safeSend({ type: "tileClick", content: tilePos });
  };

  const sendKey = (key: Key) => {
    safeSend({ type: "keypress", content: key });
  };

  // Registers a key handler on the main window for
  // any keys supported by the game
  window.addEventListener("keydown", (e) => {
    e.preventDefault();

    switch (e.key) {
      case "ArrowUp":
        sendKey(Key.Up);
        break;
      case "ArrowRight":
        sendKey(Key.Right);
        break;
      case "ArrowDown":
        sendKey(Key.Down);
        break;
      case "ArrowLeft":
        sendKey(Key.Left);
        break;
    }
  });
};
