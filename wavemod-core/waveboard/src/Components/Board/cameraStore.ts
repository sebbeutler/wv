// cameraStore.ts
import { createSignal } from "solid-js";

export function createCameraStore() {
  const [position, setPosition] = createSignal({ x: 0, y: 0 });
  const [scale, setScale] = createSignal(1);

  function move(deltaX: number, deltaY: number) {
    setPosition(prev => ({
      x: prev.x + deltaX,
      y: prev.y + deltaY
    }));
  }

  function zoom(factor: number, centerX = 0, centerY = 0) {
    // Basic zoom around the screen center or pointer coords
    // For example, just scale in/out without advanced recentering:
    setScale(prev => prev * factor);
  }

  return {
    position,
    scale,
    translate: move,
    zoom
  };
}