
import { createSignal, createEffect } from "solid-js";
// @ts-ignore
import Prism from "prismjs";
import "prismjs/themes/prism-vs.css";
import "prismjs/components/prism-javascript";
import "prismjs/components/prism-python";
import "prismjs/components/prism-c";
import "prismjs/components/prism-rust";
import "prismjs/components/prism-toml";

import "./CodeArea.css";

const CodeArea = () => {
  const [code, setCode] = createSignal("");
  const [language, setLanguage] = createSignal("javascript");
  let textareaRef: HTMLTextAreaElement, highlightRef: HTMLPreElement;

  const handleInput = (e: Event) => {
    const target = e.target as HTMLTextAreaElement;
    setCode(target.value);
  };

  const handleLanguageChange = (e: Event) => {
    const target = e.target as HTMLSelectElement;
    setLanguage(target.value);
  };

  const syncScroll = () => {
    if (highlightRef && textareaRef) {
      highlightRef.scrollTop = textareaRef.scrollTop;
      highlightRef.scrollLeft = textareaRef.scrollLeft;
    }
  };

  createEffect(() => {
    Prism.highlightAll();
  });

  return (
    <div class="editor-container">
      {/* Language Selector */}
      <select value={language()} onInput={handleLanguageChange} class="language-selector">
        <option value="python">Python</option>
        <option value="javascript">JavaScript</option>
        <option value="rust">Rust</option>
        <option value="c">C</option>
        <option value="toml">TOML</option>
      </select>

      {/* Textarea Layer */}
      <textarea
        ref={(el) => (textareaRef = el)}
        value={code()}
        onInput={handleInput}
        onScroll={syncScroll}
        class="editor-textarea"
        spellcheck={false}
      ></textarea>

      {/* Syntax-highlighted Layer */}
      <pre
        ref={(el) => (highlightRef = el)}
        class="highlight-layer"
        style="
          margin: 0; 
          padding: 0;
          font-size: 14px;
          line-height: 17px;
          font-family: monospace;
          letter-spacing: -.07em;"
      >
        <code
          class={`language-${language()}`}
          style="white-space: pre-wrap; word-wrap: break-word;"
          innerHTML={Prism.highlight(
            code(),
            Prism.languages[language()] || Prism.languages.plain,
            language()
          )}
        ></code>
      </pre>
    </div>
  );
};

export default CodeArea;