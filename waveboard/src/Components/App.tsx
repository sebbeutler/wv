import { createSignal, onMount } from "solid-js";
import "./App.css";

import Board from "./Board/Board";
import CodeArea from "./CodeArea/CodeArea";

async function run_wasm(canvasId: string) {
  const wasm = await import(`./webgl2/webgpu`); // Load WebAssembly
  await wasm.default(); // Instantiate WebAssembly
  wasm.surfboard(canvasId);
}

function App() {
  const [currentTab, setCurrentTab] = createSignal("wgpu");

  onMount(() => {
    run_wasm("app-canvas");
  });

  return (
    <main class="container">
      <select
        value={currentTab()}
        onInput={(e) => setCurrentTab(e.target.value)}
        class="language-selector"
      >
        <option value="codearea">CodeArea</option>
        <option value="Board">Board</option>
        <option value="wgpu">wgpu</option>
      </select>

      {currentTab() === "codearea" && (
        <>
          <br />
          <CodeArea />
        </>
      )}

      {currentTab() === "Board" && (
        <Board>
          <div
            style={{
              position: "absolute",
              top: "100px",
              left: "100px",
              width: "200px",
              height: "100px",
              "text-align": "center",
              background: "rgba(255, 0, 0, 0.3)",
              "border-radius": "8px",
            }}
          >
            Hello DOM
          </div>
        </Board>
      )}

      {currentTab() === "wgpu" && (
        <canvas class="main-canvas" id="canvas"></canvas>
      )}
    </main>
  );
}

export default App;
