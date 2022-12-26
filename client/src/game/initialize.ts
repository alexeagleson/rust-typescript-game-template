/** Glue that binds together the input, canvas and connection modules */

import { connectToGameServer } from "./connection";
import { createGameApp } from "./canvas";
import {
  Key,
  MapDimensions,
  PlayerDetails,
  Position,
  ServerMessage,
} from "../utility/types";
import { assertNever, log } from "../utility/functions";
import { GAME_CONFIG_URI, TILE_SIZE } from "../utility/config";
import { addInputListeners } from "./input";

let xPixel = 0;
let yPixel = 0;

const updateHoverMenuPosition = (x: number, y: number) => {
  xPixel = x;
  yPixel = y;
};

export const initializeGame = async (
  onHover: (x: number, y: number, playerDetails?: PlayerDetails) => void,
  onClick: (log: string) => void,
  onMoveCount: (count: number) => void
) => {
  const mapDimensionsResponse = await fetch(GAME_CONFIG_URI, { method: "GET" });

  if (!mapDimensionsResponse.ok) {
    throw Error("Failed to get initial game config");
  }

  const mapDimensions: MapDimensions = await mapDimensionsResponse.json();

  const { addSprite, gameCanvas, removeSprite, setSpritePosition } =
    await createGameApp(mapDimensions, TILE_SIZE);

  const onMessage = (msg: MessageEvent<unknown>) => {
    if (typeof msg.data !== "string") {
      console.error("Received invalid message", msg.data);
      throw Error;
    }
    const response: ServerMessage = JSON.parse(msg.data);

    switch (response.type) {
      case "playerPosition":
        setSpritePosition(response.content);
        break;
      case "allPlayerPositions":
        response.content.forEach((playerPos) => {
          addSprite(playerPos, "bunny");
        });
        break;
      case "removedPlayer":
        removeSprite(response.content);
        break;
      case "moveCount":
        onMoveCount(response.content);
        break;
      case "tileHover":
        onHover(xPixel, yPixel, response.content);
        break;
      case "tileClick":
        onClick(response.content);
        break;
      default:
        assertNever(response);
    }
  };

  const { safeSend } = connectToGameServer({
    onOpen: () => {
      log.trace("Connected");
    },
    onClose: () => {
      log.trace("Disconnected");
    },
    onMessage,
  });

  addInputListeners(gameCanvas, updateHoverMenuPosition, safeSend);

  let interval = setInterval(() => {
    let result = safeSend({ type: "initialize" });
    if (result === "success") {
      clearInterval(interval);
    }
  }, 100);

  return { gameCanvas };
};
