/********************************************
 * Hybrid Scrolling Demo:
 * - We have a 100 x 100 grid = 10,000 squares.
 * - Each square is 50x50 px. The total area is 5000x5000 px.
 * - We'll only render squares that intersect with
 *   the viewport + a small buffer region.
 ********************************************/

// Total grid dimensions (in squares)
const ROW_COUNT = 100;
const COL_COUNT = 100;

// Dimensions of each square (in pixels)
const SQUARE_SIZE = 50;

// Buffer (in pixels) around the visible area
// to avoid removing/adding too aggressively.
const BUFFER = 200;

// We’ll store references to created DOM nodes in a map:
// Key: "row-col", Value: DOM element
const squareElements = new Map();

// Get the scrollable container
const scrollContainer = document.getElementById("scroll-container");

// Set the scroll space size
const bigSpacer = document.getElementById("big-spacer");
bigSpacer.style.width = `${COL_COUNT * SQUARE_SIZE}px`;
bigSpacer.style.height = `${ROW_COUNT * SQUARE_SIZE}px`;

// Center the scroll position within the bigSpacer dimensions
centerViewport();

const minimap = document.getElementById("minimap");
// Set the minimap size to maintain the same aspect ratio as bigSpacer
const minimapHeight = minimap.offsetHeight; // Get the current height of the minimap
const minimapWidth = (minimapHeight * bigSpacer.offsetWidth) / bigSpacer.offsetHeight;
minimap.style.width = `${minimapWidth}px`;
minimap.style.height = `${minimapHeight}px`;

const minimapViewport = document.getElementById("minimap-viewport");
updateMinimapViewport()

// On initial load, do a render pass
updateSquaresViewport();

// Also re-render on scroll
scrollContainer.addEventListener("scroll", updateSquaresViewport);

// Also re-render on resize (optional, in case container size changes)
window.addEventListener("resize", updateSquaresViewport);

function centerViewport() {
  const containerRect = scrollContainer.getBoundingClientRect();
  const initialScrollLeft = (bigSpacer.offsetWidth - containerRect.width) / 2;
  const initialScrollTop = (bigSpacer.offsetHeight - containerRect.height) / 2;
  scrollContainer.scrollLeft = initialScrollLeft;
  scrollContainer.scrollTop = initialScrollTop;
}

function updateSquaresViewport() {
  // Get the bounding box of the scroll container's viewport
  const containerRect = scrollContainer.getBoundingClientRect();

  // Current scroll offsets
  const scrollLeft = scrollContainer.scrollLeft;
  const scrollTop = scrollContainer.scrollTop;

  // Visible area in container coordinates:
  // We'll add a buffer around the edges
  const visibleLeft = scrollLeft - BUFFER;
  const visibleRight = scrollLeft + containerRect.width + BUFFER;
  const visibleTop = scrollTop - BUFFER;
  const visibleBottom = scrollTop + containerRect.height + BUFFER;

  // Figure out which rows/columns intersect this rectangle.
  // Each square is 50px in width/height, so we can calculate min and max indices.
  const minCol = Math.max(0, Math.floor(visibleLeft / SQUARE_SIZE));
  const maxCol = Math.min(COL_COUNT - 1, Math.floor(visibleRight / SQUARE_SIZE));

  const minRow = Math.max(0, Math.floor(visibleTop / SQUARE_SIZE));
  const maxRow = Math.min(ROW_COUNT - 1, Math.floor(visibleBottom / SQUARE_SIZE));

  viewportRenderSquares(minRow, maxRow, minCol, maxCol);
  viewportClipSquares(minRow, maxRow, minCol, maxCol);

}

function viewportRenderSquares(minRow, maxRow, minCol, maxCol) {
  // Create or re-use squares for all visible (row, col) positions.
  for (let row = minRow; row <= maxRow; row++) {
    for (let col = minCol; col <= maxCol; col++) {
      const key = `${row}-${col}`;
      if (!squareElements.has(key)) {
        // Not yet in the DOM, so create and position it
        const square = document.createElement("div");
        square.className = "square";

        // Assign a random color (or any color scheme)
        square.style.backgroundColor = randomColor();

        // Position the square absolutely within the container
        // top/left are multiples of the square size
        square.style.top = `${row * SQUARE_SIZE}px`;
        square.style.left = `${col * SQUARE_SIZE}px`;

        // Keep a reference in our map
        squareElements.set(key, square);

        // Append to #scroll-container
        scrollContainer.appendChild(square);
      }
    }
  }
}

function viewportClipSquares(minRow, maxRow, minCol, maxCol) {
  // Remove squares that are no longer visible
  // (not within [minRow..maxRow] and [minCol..maxCol]).
  for (const [key, element] of squareElements) {
    const [rowStr, colStr] = key.split("-");
    const row = parseInt(rowStr, 10);
    const col = parseInt(colStr, 10);

    if (
      row < minRow || row > maxRow ||
      col < minCol || col > maxCol
    ) {
      // Remove from the DOM
      element.remove();
      // Remove from the map
      squareElements.delete(key);
    }
  }
}

function updateMinimapViewport() {
  const { clientWidth: minimapWidth, clientHeight: minimapHeight } = minimap;
  const { offsetWidth: spacerWidth, offsetHeight: spacerHeight } = bigSpacer;
  const { offsetWidth: containerWidth, offsetHeight: containerHeight, scrollLeft, scrollTop } = scrollContainer;
  const borderWidth = parseFloat(getComputedStyle(minimapViewport).borderWidth) || 0;

  // Calculate scale factors
  const scaleWidth = minimapWidth / spacerWidth;
  const scaleHeight = minimapHeight / spacerHeight;

  // Update minimap viewport dimensions
  minimapViewport.style.width = `${containerWidth * scaleWidth - 2 * borderWidth}px`;
  minimapViewport.style.height = `${containerHeight * scaleHeight - 2 * borderWidth}px`;

  // Calculate and constrain positions
  const left = Math.max(0, Math.min(scrollLeft * scaleWidth, minimapWidth - minimapViewport.offsetWidth));
  const top = Math.max(0, Math.min(scrollTop * scaleHeight, minimapHeight - minimapViewport.offsetHeight));

  minimapViewport.style.left = `${left}px`;
  minimapViewport.style.top = `${top}px`;
}

/**
 * Simple helper to generate a random color
 * in hexadecimal format.
 */
function randomColor() {
  const r = Math.floor(Math.random() * 256).toString(16).padStart(2, "0");
  const g = Math.floor(Math.random() * 256).toString(16).padStart(2, "0");
  const b = Math.floor(Math.random() * 256).toString(16).padStart(2, "0");
  return `#${r}${g}${b}`;
}

// Prevent default scrolling behavior and manually apply scroll offsets (support diagonal)
scrollContainer.addEventListener("wheel", (event) => {
  event.preventDefault(); // Prevent the browser’s default scroll

  // Manually adjust scroll offsets
  scrollContainer.scrollLeft += event.deltaX;
  scrollContainer.scrollTop += event.deltaY;
  updateMinimapViewport();
});