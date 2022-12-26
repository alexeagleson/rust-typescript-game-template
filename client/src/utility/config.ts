// General config
const DOMAIN = "localhost";
const PORT = 3030;
export const STRICT_MODE: boolean = true;
export const LOG_LEVEL: "trace" | "none" = "trace";

// Game config
export const TILE_SIZE = 24;

// API config
export const GAME_CONFIG_URI: string =
  "http://" + DOMAIN + ":" + PORT + "/api/game-config";
export const WEBSOCKET_URI: string =
  "ws://" + DOMAIN + ":" + PORT + "/api/game";
