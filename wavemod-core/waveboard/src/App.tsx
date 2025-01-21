import { createSignal, onMount } from "solid-js";
import "./App.css";

import Board from "./Board/Board";
import CodeArea from "./CodeArea";

async function run_wasm(_canvasId: string) {
  const wasm = await import(`./webgl2/webgl2`); // Load WebAssembly
  await wasm.default(); // Instantiate WebAssembly
}

function App() {
  const [currentTab, setCurrentTab] = createSignal("wasm");

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
        <option value="board">Board</option>
        <option value="wasm">wasm</option>
        <option value="wgpu">wgpu</option>
      </select>

      {currentTab() === "codearea" && (
        <>
          <br />
          <CodeArea />
        </>
      )}

      {currentTab() === "board" && (
        <Board width={700} height={900}>
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

      {currentTab() === "wasm" && (
        <canvas
          class="main-canvas"
          id="canvas"
          style="width:300px;height:300px;"
        ></canvas>
      )}

      {currentTab() === "webgpu" && (
        <div>
          <label for="axis">Axis:</label>
          <select id="axis">
            <option value="0">X-Axis</option>
            <option value="1">Y-Axis</option>
            <option value="2" selected>
              Z-Axis
            </option>
          </select>
          <label for="position">Position:</label>
          <input
            id="position"
            type="range"
            min="-10000.0"
            max="100.0"
            step="0.01"
            value="0"
          />
          <canvas id="splatting-canvas" width="800" height="600"></canvas>
        </div>
        // <script type="module" src="./src/Components/splatting.ts"></script>
      )}
    </main>
  );
}

export default App;
