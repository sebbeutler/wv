// CanvasLayer.tsx
import { Component, createEffect, onMount, createSignal } from "solid-js";
import type { createCameraStore } from "./viewportStore.ts";

interface Props {
  camera: ReturnType<typeof createCameraStore>;
}

const CanvasLayer: Component<Props> = (props) => {
  // Refs to our 4 canvases
  let canvasRefs: HTMLCanvasElement[] = [];

  // Let’s store the known container width/height
  // (For a quick hack, just use window dimensions.)
  const [containerW, setContainerW] = createSignal(globalThis.innerWidth);
  const [containerH, setContainerH] = createSignal(globalThis.innerHeight);

  // If you want them to update on resize:
  onMount(() => {
    const onResize = () => {
      setContainerW(globalThis.innerWidth);
      setContainerH(globalThis.innerHeight);
    };
    globalThis.addEventListener("resize", onResize);
    return () => globalThis.removeEventListener("resize", onResize);
  });

  // Each canvas will be half the container
  // in device pixels:
  const tileWidth = () => containerW() / 2;
  const tileHeight = () => containerH() / 2;

  // Re-draw whenever the camera changes
  createEffect(() => {
    const { x, y } = props.camera.position();
    const s = props.camera.scale();

    // For each canvas, do partial or full re-draw:
    canvasRefs.forEach((canvasRef, idx) => {
      const ctx = canvasRef.getContext("2d");
      if (!ctx) return;

      // Clear
      ctx.clearRect(0, 0, canvasRef.width, canvasRef.height);

      // Save context
      ctx.save();

      // Determine this canvas’s offset in the grid
      const NUM_COLS = 2;
      const col = idx % NUM_COLS;
      const row = Math.floor(idx / NUM_COLS);

      // We'll offset the drawing based on the quadrant
      const offsetX = col * tileWidth();
      const offsetY = row * tileHeight();

      // Translate + scale according to the camera, plus quadrant offset
      ctx.translate(-x - offsetX, -y - offsetY);
      ctx.scale(s, s);

      // --- EXAMPLE DRAWING ---
      ctx.fillStyle = `rgba(${50 * idx}, 80, 200, 0.3)`;
      ctx.fillRect(100, 100, 200, 100);

      ctx.restore();
    });
  });

  return (
    <div
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        /* 
          Flex container to tile canvases in a 2x2 grid
          filling the entire board container:
        */
        display: "grid",
        "grid-template-columns": "repeat(2, 1fr)",
        width: "100%",
        height: "100%",
      }}
    >
      {/* Canvas */}
      {Array.from({ length: 4 }).map((_, i) => (
        <canvas
          ref={(el) => (canvasRefs[i] = el)}
          width={tileWidth()}
          height={tileHeight()}
          style={{
            "border": "1px solid #ccc",
          }}
        />
      ))}
    </div>
  );
};

export default CanvasLayer;