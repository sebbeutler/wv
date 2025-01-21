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
      <svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg" style="pointer-events: none;border: 1px groove #ccc;">
        <defs>
          <pattern id="smallGrid" width="10" height="10" patternUnits="userSpaceOnUse">
        <path d="M 10 0 L 0 0 0 10" fill="none" stroke="rgba(128, 128, 128, 0.3)" stroke-width="0.3"></path>
          </pattern>
          <pattern id="grid" width="100" height="100" patternUnits="userSpaceOnUse">
        <rect width="100" height="100" fill="url(#smallGrid)"></rect>
        <path d="M 100 0 L 0 0 0 100" fill="none" stroke="rgba(128, 128, 128, 0.7)" stroke-width="0.7"></path>
          </pattern>
        </defs>
        <rect width="100%" height="100%" fill="url(#grid)"></rect>
      </svg>
    </div>
  );
};

export default DOMLayer;