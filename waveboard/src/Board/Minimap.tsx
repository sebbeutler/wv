import { Component, onMount, createMemo, createSignal, onCleanup } from 'solid-js';
import type { createCameraStore, createViewportStore } from "./viewportStore.ts";

interface MinimapProps {
    camera: ReturnType<typeof createCameraStore>;
    viewport: ReturnType<typeof createViewportStore>;
}

const Minimap: Component<MinimapProps> = (props: MinimapProps) => {
    let minimapRef: HTMLDivElement | undefined;
    let minimapCameraRef: HTMLDivElement | undefined;

    const camera = () => props.camera;
    const viewport = () => props.viewport;

    const [minimapSize, setMinimapSize] = createSignal({width: 0, height: 0})
    
    const setupMinimap = () => {
        if (minimapRef && minimapCameraRef) {
            const viewportSize = viewport().size();
            const minimapHeight = 200;
            const minimapWidth = minimapHeight * viewportSize.height / viewportSize.height;
            
            setMinimapSize({
                width: minimapWidth,
                height: minimapHeight
            })
        }
    };

    
    let isDragging = false;
    let lastPointerId: number | null = null;

    const onPointerDown = (e: PointerEvent) => {
        if (e.button === 0) {
            isDragging = true;
            lastPointerId = e.pointerId;
            // setPointerCapture could also be used if needed
        }
    };

    const onPointerUp = (e: PointerEvent) => {
        if (e.pointerId === lastPointerId) {
            isDragging = false;
            lastPointerId = null;
        }
    };

    const onPointerMove = (e: PointerEvent) => {
        e.preventDefault()
        if (isDragging && e.pointerId === lastPointerId) {
            const viewportSize = viewport().size();
            camera().translate(
                (e.movementX * viewportSize.width / minimapSize().width),
                (e.movementY * viewportSize.height / minimapSize().height)  
            );
        }
    };

    const translateCamera = createMemo(() => {
        const cameraPosition = camera().position();
        const viewportSize = viewport().size();
        const s = camera().scale();
        const newX = minimapSize().width * cameraPosition.x / viewportSize.width;
        const newY = minimapSize().height * cameraPosition.y / viewportSize.height;
        return `translate(${newX}px, ${newY}px) scale(${s})`;
    });

    onMount(() => {
        setupMinimap();
        minimapCameraRef?.addEventListener("pointerdown", onPointerDown);
        minimapCameraRef?.addEventListener("pointermove", onPointerMove);
        minimapCameraRef?.addEventListener("pointerup", onPointerUp);
        minimapCameraRef?.addEventListener("pointerout", onPointerUp);
    });

    onCleanup(() => {
        minimapCameraRef?.removeEventListener("pointerdown", onPointerDown);
        minimapCameraRef?.removeEventListener("pointermove", onPointerMove);
        minimapCameraRef?.removeEventListener("pointerup", onPointerUp);
        minimapCameraRef?.removeEventListener("pointerout", onPointerUp);
    });
    
    return (
        <div 
        ref={minimapRef}
        class="minimap"
        style={{
            border: "1px solid black",
            margin: "2px",
            // width: "100%",
            width: `${minimapSize().width}px`,
            height: `${minimapSize().height}px`,
        }}
        >
            <div
                ref={minimapCameraRef}
                class="minimap-camera"
                style={{
                    position: "relative",
                    border: "1px solid red",
                    transform: translateCamera(),
                    width: `${minimapSize().width * camera().size().width / viewport().size().width}px`,
                    height: `${minimapSize().height * camera().size().height / viewport().size().height}px`,
                }}
            ></div>
        </div>
    );
};

export default Minimap;