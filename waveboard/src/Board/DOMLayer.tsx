// DOMLayer.tsx
import { Component, createMemo } from "solid-js";
import type { createCameraStore } from "./cameraStore";

interface Props {
  camera: ReturnType<typeof createCameraStore>;
  ref: HTMLDivElement;
}

const DOMLayer: Component<Props> = (props) => {
  const camera = () => props.camera;

  // We compute the transform
  const transform = createMemo(() => {
    const { x, y } = camera().position();
    const s = camera().scale();
    return `translate(${x}px, ${y}px) scale(${s})`;
  });

  return (
    <div
      ref={props.ref}
      class="dom-layer"
      style={{
        position: "absolute",
        top: 0,
        left: 0,
        width: "2000px",   // or something large
        height: "2000px", // or dynamic based on your content
        "transform-origin": "0 0",
        transform: transform()
      }}
    >
    </div>
  );
};

export default DOMLayer;