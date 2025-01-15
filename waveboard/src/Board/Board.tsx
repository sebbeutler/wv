// Board.tsx
import { JSX, Component, onMount, onCleanup, createSignal } from 'solid-js';
import { createCameraStore, createViewportStore } from './viewportStore.ts';
import CanvasLayer from './CanvasLayer.tsx';
import DOMLayer from './DOMLayer.tsx';
import Minimap from './Minimap.tsx';

import './Board.css';

interface BoardProps {
  width: number;
  height: number;
  children: JSX.Element;
}

const Board: Component<BoardProps> = (props) => {
  // Camera/viewport creation
  const camera = createCameraStore();
  const viewport = createViewportStore(props.width, props.height);

  // Refs for DOM elements
  let boardRef: HTMLDivElement | undefined;
  let DOMLayerRef!: HTMLDivElement;

  // Signals for panning state
  const [isPanning, setIsPanning] = createSignal(false);
  const [dragPointerId, setDragPointerId] = createSignal<number | null>(null);

  // Pointer event handlers
  const onPointerDown = (e: PointerEvent) => {
    if (e.button === 0) {
      setIsPanning(true);
      setDragPointerId(e.pointerId);
      // Optionally: boardRef?.setPointerCapture(e.pointerId)
    }
  };

  const onPointerUp = (e: PointerEvent) => {
    if (e.pointerId === dragPointerId()) {
      setIsPanning(false);
      setDragPointerId(null);
      // Optionally: boardRef?.releasePointerCapture(e.pointerId)
    }
  };

  const onPointerMove = (e: PointerEvent) => {
    if (!isPanning() || e.pointerId !== dragPointerId()) return;
    // Ensure we only pan if the target is the DOM layer (or you can remove the check if you want to pan from anywhere).
    if (e.target !== DOMLayerRef) return;

    camera.translate(-e.movementX, -e.movementY);
  };

  // Wheel event for scrolling or zooming (currently set to panning in your example)
  const onWheel = (e: WheelEvent) => {
    e.preventDefault(); // important if you're overriding scroll
    // Example for panning:
    camera.translate(-e.deltaX, -e.deltaY);

    // Or if you prefer zooming:
    // const zoomFactor = e.deltaY < 0 ? 1.05 : 0.95;
    // camera.zoom(zoomFactor, e.clientX, e.clientY);
  };

  // Keyboard handler (centering on 'c')
  const onKeyPress = (e: KeyboardEvent) => {
    if (e.key === 'c') {
      const offsetX = -(DOMLayerRef.clientWidth - globalThis.innerWidth) / 2;
      const offsetY = -(DOMLayerRef.clientHeight - globalThis.innerHeight) / 2;
      camera.move(offsetX, offsetY);
    }
  };

  // Append child nodes to the DOM layer
  const addChild = (child: JSX.Element) => {
    // Solid components are typically appended via rendering, but if you need to manually
    // inject DOM nodes, you can do so here. For simple children, you could also just place them
    // inside <DOMLayer> if that’s what you intend.
    if (!DOMLayerRef) return;
    // @ts-expect-error if child is a DOM node
    DOMLayerRef.appendChild(child);
  };

  // Lifecycle hooks
  onMount(() => {
    // Attach event listeners
    boardRef?.addEventListener('pointerdown', onPointerDown);
    boardRef?.addEventListener('pointermove', onPointerMove);
    boardRef?.addEventListener('pointerup', onPointerUp);
    boardRef?.addEventListener('pointerout', onPointerUp);
    boardRef?.addEventListener('wheel', onWheel, { passive: false });

    globalThis.addEventListener('keypress', onKeyPress);

    // Add children to DOM layer
    addChild(props.children);
  });

  onCleanup(() => {
    // Remove event listeners
    boardRef?.removeEventListener('pointerdown', onPointerDown);
    boardRef?.removeEventListener('pointermove', onPointerMove);
    boardRef?.removeEventListener('pointerup', onPointerUp);
    boardRef?.removeEventListener('pointerout', onPointerUp);
    boardRef?.removeEventListener('wheel', onWheel);

    globalThis.removeEventListener('keypress', onKeyPress);
  });

  return (
    <div
      ref={boardRef}
      class="board-container"
      style={{
        width: '100vw',
        height: '100vh',
        overflow: 'hidden',
        position: 'relative',
      }}
    >
      {/* Canvas-based layer */}
      <CanvasLayer camera={camera} />

      {/* DOM overlay */}
      <DOMLayer viewport={viewport} ref={DOMLayerRef} camera={camera} />

      {/* Minimap */}
      <Minimap camera={camera} viewport={viewport} />
    </div>
  );
};

export default Board;
