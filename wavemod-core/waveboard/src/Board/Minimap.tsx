import { Component, onMount, createSignal, onCleanup, createMemo, createEffect } from 'solid-js';
import type { createCameraStore, createViewportStore } from './viewportStore';

import './Board.css';

interface MinimapProps {
  camera: ReturnType<typeof createCameraStore>;
  viewport: ReturnType<typeof createViewportStore>;
}

const Minimap: Component<MinimapProps> = (props: MinimapProps) => {
  let minimapRef: HTMLDivElement | undefined;
  let minimapCameraRef: HTMLDivElement | undefined;

  // Expose camera & viewport as getter functions for reactivity
  const camera = () => props.camera;
  const viewport = () => props.viewport;

  // Cache camera/viewport sizes in memos to avoid repeated function calls
  const cameraSize = createMemo(() => camera().size());
  const viewportSize = createMemo(() => viewport().size());

  // Keep track of isDragging and last pointer ID in signals
  const [isDragging, setIsDragging] = createSignal(false);
  const [dragPointerId, setDragPointerId] = createSignal<number | null>(null);

  // Dynamically compute the minimap size
  const [minimapSize, setMinimapSize] = createSignal({
    width: 0,
    height: cameraSize().height * 0.1,
  });

  // Recompute minimap dimensions when camera/viewport size changes
  createEffect(() => {
    const vSize = viewportSize();
    const height = cameraSize().height * 0.1;
    // Example: keep the same aspect ratio or just recalc proportionally
    // For demonstration: using the ratio vSize.width / vSize.height
    // so the minimap width scales with actual board ratio
    const width = height * (vSize.width / vSize.height);

    setMinimapSize({ width, height });
  });

  // Translate the camera (red box) within the minimap
  const translateCamera = createMemo(() => {
    const { x, y } = camera().position();
    const { width: vW, height: vH } = viewportSize();
    const { width: mW, height: mH } = minimapSize();
    const scale = camera().scale();

    const newX = (mW * x) / vW;
    const newY = (mH * y) / vH;

    return `translate(${newX}px, ${newY}px) scale(${scale})`;
  });

  // Camera box dimensions (the size of the camera's view in minimap scale)
  const cameraBoxSize = createMemo(() => {
    const { width: cW, height: cH } = cameraSize();
    const { width: vW, height: vH } = viewportSize();
    const { width: mW, height: mH } = minimapSize();

    return {
      width: (mW * cW) / vW,
      height: (mH * cH) / vH,
    };
  });

  // Unified pointer handler (optional style). 
  // You can also keep them separate if you prefer.
  const onPointerDown = (e: PointerEvent) => {
    if (e.button === 0) {
      setIsDragging(true);
      setDragPointerId(e.pointerId);
      // optionally: minimapCameraRef?.setPointerCapture(e.pointerId);
    }
  };

  const onPointerUp = (e: PointerEvent) => {
    if (e.pointerId === dragPointerId()) {
      setIsDragging(false);
      setDragPointerId(null);
      // optionally: minimapCameraRef?.releasePointerCapture(e.pointerId);
    }
  };

  const onPointerMove = (e: PointerEvent) => {
    if (!isDragging() || e.pointerId !== dragPointerId()) return;
    e.preventDefault();

    const { width: vW, height: vH } = viewportSize();
    const { width: mW, height: mH } = minimapSize();

    camera().translate(
      (e.movementX * vW) / mW,
      (e.movementY * vH) / mH
    );
  };

  // Register event listeners on mount; cleanup on unmount
  onMount(() => {
    minimapCameraRef?.addEventListener('pointerdown', onPointerDown);
    minimapCameraRef?.addEventListener('pointermove', onPointerMove);
    minimapCameraRef?.addEventListener('pointerup', onPointerUp);
    minimapCameraRef?.addEventListener('pointerout', onPointerUp);
  });

  onCleanup(() => {
    minimapCameraRef?.removeEventListener('pointerdown', onPointerDown);
    minimapCameraRef?.removeEventListener('pointermove', onPointerMove);
    minimapCameraRef?.removeEventListener('pointerup', onPointerUp);
    minimapCameraRef?.removeEventListener('pointerout', onPointerUp);
  });

  return (
    <div
      ref={minimapRef}
      class="minimap"
      style={{
        border: '1px solid black',
        margin: '2px',
        width: `${minimapSize().width}px`,
        height: `${minimapSize().height}px`,
        position: 'relative',
      }}
    >
      <div
        ref={minimapCameraRef}
        class="minimap-camera"
        style={{
          position: 'absolute',
          border: '1px solid red',
          transform: translateCamera(),
          width: `${cameraBoxSize().width}px`,
          height: `${cameraBoxSize().height}px`,
          // If you prefer the camera box to remain clickable,
          // ensure pointer-events are enabled or transform is translated
          // with the correct origin.
        }}
      ></div>
    </div>
  );
};

export default Minimap;
