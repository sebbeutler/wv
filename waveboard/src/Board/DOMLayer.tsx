// DOMLayer.tsx
import { Component, createMemo } from "solid-js";
import type { createCameraStore, createViewportStore } from "./viewportStore.ts";

interface Props {
  camera: ReturnType<typeof createCameraStore>;
  viewport: ReturnType<typeof createViewportStore>;
  ref: HTMLDivElement;
}

const DOMLayer: Component<Props> = (props) => {
  const camera = () => props.camera;
  const viewport = () => props.viewport;

  // We compute the transform
  const transform = createMemo(() => {
    const { x, y } = camera().position();
    const s = camera().scale();
    return `translate(${-x}px, ${-y}px) scale(${s})`;
  });

  return (
    <div
      ref={props.ref}
      class="dom-layer"
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        width: `${viewport().size().width}px`,
        height: `${viewport().size().height}px`,
        "transform-origin": "0 0",
        transform: transform()
      }}
    >
      <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg" style="pointer-events: none;">
        <defs>
          <pattern id="smallGrid" width="8" height="8" patternUnits="userSpaceOnUse">
        <path d="M 8 0 L 0 0 0 8" fill="none" stroke="currentColor" stroke-width="0.5"></path>
          </pattern>
          <pattern id="grid" width="80" height="80" patternUnits="userSpaceOnUse">
        <rect width="80" height="80" fill="url(#smallGrid)"></rect>
        <path d="M 80 0 L 0 0 0 80" fill="none" stroke="currentColor" stroke-width="1"></path>
          </pattern>
        </defs>
        <rect width="100%" height="100%" fill="url(#grid)"></rect>
      </svg>
    </div>
  );
};

export default DOMLayer;