import { createSignal, onCleanup } from "solid-js";
import icon from "./assets/icon.png";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import * as wmd from "./commands";

import Board from "./Board/Board";
import ScriptEditor from "./CodeArea";
import Tab from "./Tab";
import NodeFrame from "./NodeFrame"

function App() {

  wmd.show_window();
  wmd.gx_init();
  // wmd.devtools();

  const [currentTab, setCurrentTab] = createSignal("Board");

  const switchTab = (e: Event) => {
    const target = e.target as HTMLSelectElement;
    setCurrentTab(target.value);
  };
  return (
    <main class="container">
  
        {/* <img src={icon} class="logo wmd" alt="Wave logo" /> */}

        <select value={currentTab()} onInput={switchTab} class="language-selector">
          <option value="codearea">CodeArea</option>
          <option value="Board">Board</option>
        </select>

        <Tab signal={currentTab} tabname="codearea">
          <br/>
          <ScriptEditor />
        </Tab>

        <Tab signal={currentTab} tabname="Board">
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
              "border-radius": "8px"
            }}
          >
            Hello DOM
          </div>
        </Board>
      </Tab>

    </main>
  );
}

export default App;
