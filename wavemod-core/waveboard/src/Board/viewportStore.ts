import { createSignal, onMount, onCleanup } from "solid-js";

export function createCameraStore() {
  const [position, setPosition] = createSignal({ x: 0, y: 0 });
  const [scale, setScale] = createSignal(1);

  // Initialize to current window size.
  const [size, setSize] = createSignal({
    width: globalThis.innerWidth,
    height: globalThis.innerHeight
  });

  // Update size whenever the window is resized.
  function handleResize() {
    setSize({
      width: globalThis.innerWidth,
      height: globalThis.innerHeight
    });
  }

  onMount(() => {
    globalThis.addEventListener("resize", handleResize);
  });
  onCleanup(() => {
    globalThis.removeEventListener("resize", handleResize);
  });

  function translate(deltaX: number, deltaY: number) {
    setPosition(prev => ({
      x: prev.x + deltaX,
      y: prev.y + deltaY
    }));
  }

  function move(newX: number, newY: number) {
    setPosition({ x: newX, y: newY });
  }

  function zoom(factor: number, _centerX = 0, _centerY = 0) {
    setScale(prev => prev * factor);
  }

  return {
    position,
    scale,
    move,
    translate,
    zoom,
    size
  };
}

export function createViewportStore(width: number, height: number) {
  const [size, _setSize] = createSignal({ width, height });

  function ratio() {
    return width / height;
  }

  return {
    size,
    ratio
  };
}