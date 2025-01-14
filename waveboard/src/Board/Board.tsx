// Board.tsx
import { JSX, Component, onMount, onCleanup } from "solid-js";
import { createCameraStore, createViewportStore } from "./viewportStore.ts";
import CanvasLayer from "./CanvasLayer.tsx";
import DOMLayer from "./DOMLayer.tsx";
import Minimap from "./Minimap.tsx";

interface BoardProps {
  width: number;
  height: number;
  children: JSX.Element;
}

const Board: Component<BoardProps> = (props: BoardProps) => {
  const camera = createCameraStore();
  const viewport = createViewportStore(props.width, props.height);

  let boardRef: HTMLDivElement | undefined;
  let DOMLayerRef!: HTMLDivElement;

  // For panning
  let isPanning = false;
  let lastPointerId: number | null = null;

  const onPointerDown = (e: PointerEvent) => {
    if (e.button === 0) {
      isPanning = true;
      lastPointerId = e.pointerId;
      // setPointerCapture could also be used if needed
    }
  };

  const onPointerUp = (e: PointerEvent) => {
    if (e.pointerId === lastPointerId) {
      isPanning = false;
      lastPointerId = null;
    }
  };

  const onPointerMove = (e: PointerEvent) => {
    if (isPanning &&
      e.target === DOMLayerRef &&
      e.pointerId === lastPointerId
    ) {
      camera.translate(-e.movementX, -e.movementY);
    }
  };

  const onWheel = (e: WheelEvent) => {
    e.preventDefault();
    // const zoomFactor = e.deltaY < 0 ? 1.05 : 0.95;
    // camera.zoom(zoomFactor, e.clientX, e.clientY);
    camera.translate(-e.deltaX, -e.deltaY)
  };

  const addChild = (child: any) => {
    DOMLayerRef.appendChild(child);
  };

  const onKeyPress = (e: KeyboardEvent) => {
    if (e.key == "c") {
      // Center camera
      console.log(
        -(DOMLayerRef.clientWidth - globalThis.innerWidth) / 2,
        -(DOMLayerRef.clientHeight - globalThis.innerHeight) / 2);
      camera.move(
        -(DOMLayerRef.clientWidth - globalThis.innerWidth) / 2,
        -(DOMLayerRef.clientHeight - globalThis.innerHeight) / 2);
    }
  }

  onMount(() => {
    boardRef?.addEventListener("pointerdown", onPointerDown);
    boardRef?.addEventListener("pointermove", onPointerMove);
    boardRef?.addEventListener("pointerup", onPointerUp);
    boardRef?.addEventListener("pointerout", onPointerUp);
    boardRef?.addEventListener("wheel", onWheel, { passive: false });
    globalThis.addEventListener("keypress", onKeyPress);

    addChild(props.children);
  });

  onCleanup(() => {
    boardRef?.removeEventListener("pointerdown", onPointerDown);
    boardRef?.removeEventListener("pointermove", onPointerMove);
    boardRef?.removeEventListener("pointerup", onPointerUp);
    boardRef?.removeEventListener("pointerout", onPointerUp);
    boardRef?.removeEventListener("wheel", onWheel);
    globalThis.removeEventListener("keypress", onKeyPress);
  });

  return (
    <div
      ref={boardRef}
      class="board-container"
      style={{
        width: "100vw",
        height: "100vh",
        overflow: "hidden",
        position: "relative",
      }}
    >
      {/* 4-Canvas back layer */}
      <CanvasLayer camera={camera} />

      {/* DOM overlay */}
      <DOMLayer viewport={viewport} ref={DOMLayerRef} camera={camera} />
    
      <Minimap camera={camera} viewport={viewport} />
    </div>
  );
};

export default Board;