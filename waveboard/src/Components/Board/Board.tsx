// Board.tsx
import { JSX, Component, onMount, onCleanup } from "solid-js";
import { createCameraStore } from "./cameraStore";
import CanvasLayer from "./CanvasLayer";
import DOMLayer from "./DOMLayer";

interface BoardProps {
  children: JSX.Element;
}

const Board: Component<BoardProps> = (props: BoardProps) => {
  const camera = createCameraStore();
  
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
    if (isPanning && e.pointerId === lastPointerId) {
      camera.translate(e.movementX, e.movementY);
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

  onMount(() => {
    // You might want to listen at the document level to ensure you don't lose events
    boardRef?.addEventListener("pointerdown", onPointerDown);
    boardRef?.addEventListener("pointermove", onPointerMove);
    boardRef?.addEventListener("pointerup", onPointerUp);
    boardRef?.addEventListener("wheel", onWheel, { passive: false });

    addChild(props.children);
  });

  onCleanup(() => {
    boardRef?.removeEventListener("pointerdown", onPointerDown);
    boardRef?.removeEventListener("pointermove", onPointerMove);
    boardRef?.removeEventListener("pointerup", onPointerUp);
    boardRef?.removeEventListener("wheel", onWheel);
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
      <DOMLayer ref={DOMLayerRef} camera={camera} />
    </div>
  );
};

export default Board;