/***************************************************
 * Hybrid Scrolling Demo
 * - We have a 100 x 100 grid = 10,000 squares.
 * - Each square is 50x50 px => total area: 5000x5000 px.
 * - We only render squares intersecting the viewport
 *   plus a small buffer for smooth scrolling.
 ***************************************************/

/* -------------------- Constants -------------------- */

const ROW_COUNT = 100;      // Number of rows
const COL_COUNT = 100;      // Number of columns
const SQUARE_SIZE = 50;     // Size of each square in px
const BUFFER = 200;         // Buffer around the visible area in px

// Map of "row-col" -> square DOM element
const squareElements = new Map();

/* -------------------- DOM Elements -------------------- */

const scrollContainer = document.getElementById("scroll-container");
const bigSpacer = document.getElementById("big-spacer");
const minimap = document.getElementById("minimap");
const minimapViewport = document.getElementById("minimap-viewport");

/* ------------- Initial Setup & Event Listeners ------------- */

setupContainerSize();
centerScrollContainer();
setupMinimap();
updateMinimapViewport();
updateSquaresViewport();

scrollContainer.addEventListener("scroll", updateSquaresViewport);
window.addEventListener("resize", updateSquaresViewport);

// Prevent default scrolling (for diagonal manual scroll)
scrollContainer.addEventListener("wheel", onWheelScroll);

/* ---------------------- Functions ---------------------- */

/**
 * Sets the size of the spacer that defines the total scrollable area.
 */
function setupContainerSize() {
  bigSpacer.style.width = `${COL_COUNT * SQUARE_SIZE}px`;
  bigSpacer.style.height = `${ROW_COUNT * SQUARE_SIZE}px`;
}

/**
 * Centers the visible portion of the container in the middle of the grid.
 */
function centerScrollContainer() {
  const containerRect = scrollContainer.getBoundingClientRect();
  scrollContainer.scrollLeft = (bigSpacer.offsetWidth - containerRect.width) / 2;
  scrollContainer.scrollTop = (bigSpacer.offsetHeight - containerRect.height) / 2;
}

/**
 * Sets the width/height of the minimap to maintain the same aspect ratio
 * as the main scroll area.
 */
function setupMinimap() {
  const minimapHeight = minimap.offsetHeight;
  const minimapWidth = (minimapHeight * bigSpacer.offsetWidth) / bigSpacer.offsetHeight;

  minimap.style.width = `${minimapWidth}px`;
  minimap.style.height = `${minimapHeight}px`;
}

/**
 * Updates which squares should be rendered (or removed)
 * based on the current visible area + BUFFER.
 */
function updateSquaresViewport() {
  const containerRect = scrollContainer.getBoundingClientRect();
  const { scrollLeft, scrollTop } = scrollContainer;

  const visibleLeft = scrollLeft - BUFFER;
  const visibleRight = scrollLeft + containerRect.width + BUFFER;
  const visibleTop = scrollTop - BUFFER;
  const visibleBottom = scrollTop + containerRect.height + BUFFER;

  // Determine the min/max rows/cols to render
  const minCol = Math.max(0, Math.floor(visibleLeft / SQUARE_SIZE));
  const maxCol = Math.min(COL_COUNT - 1, Math.floor(visibleRight / SQUARE_SIZE));
  const minRow = Math.max(0, Math.floor(visibleTop / SQUARE_SIZE));
  const maxRow = Math.min(ROW_COUNT - 1, Math.floor(visibleBottom / SQUARE_SIZE));

  // Skip if there’s no change in which squares should be visible
  if (
    minRow === lastMinRow && maxRow === lastMaxRow &&
    minCol === lastMinCol && maxCol === lastMaxCol
  ) {
    return;
  }

  renderVisibleSquares(minRow, maxRow, minCol, maxCol);
  removeInvisibleSquares(minRow, maxRow, minCol, maxCol);
}

/**
 * Renders (or reuses) squares that fall within the specified row/col bounds.
 */
function renderVisibleSquares(minRow, maxRow, minCol, maxCol) {
  for (let row = minRow; row <= maxRow; row++) {
    for (let col = minCol; col <= maxCol; col++) {
      const key = `${row}-${col}`;

      // If the square isn’t in the DOM yet, create it
      if (!squareElements.has(key)) {
        const square = document.createElement("div");
        square.className = "square";
        square.style.backgroundColor = randomColor();
        square.style.top = `${row * SQUARE_SIZE}px`;
        square.style.left = `${col * SQUARE_SIZE}px`;

        squareElements.set(key, square);
        scrollContainer.appendChild(square);
      }
    }
  }
}

/**
 * Removes squares that no longer fall within the specified row/col bounds.
 */
function removeInvisibleSquares(minRow, maxRow, minCol, maxCol) {
  for (const [key, element] of squareElements) {
    const [rowStr, colStr] = key.split("-");
    const row = parseInt(rowStr, 10);
    const col = parseInt(colStr, 10);

    const outOfBounds =
      row < minRow || row > maxRow ||
      col < minCol || col > maxCol;

    if (outOfBounds) {
      element.remove();
      squareElements.delete(key);
    }
  }
}

/**
 * Updates the minimap viewport size and position to reflect
 * the current scroll region in the main container.
 */
function updateMinimapViewport() {
  const { clientWidth: mmWidth, clientHeight: mmHeight } = minimap;
  const { offsetWidth: spacerWidth, offsetHeight: spacerHeight } = bigSpacer;
  const {
    offsetWidth: containerWidth,
    offsetHeight: containerHeight,
    scrollLeft,
    scrollTop,
  } = scrollContainer;

  const borderWidth = parseFloat(getComputedStyle(minimapViewport).borderWidth) || 0;
  const scaleX = mmWidth / spacerWidth;
  const scaleY = mmHeight / spacerHeight;

  // Size of the minimap's viewport
  minimapViewport.style.width = `${containerWidth * scaleX - 2 * borderWidth}px`;
  minimapViewport.style.height = `${containerHeight * scaleY - 2 * borderWidth}px`;

  // Constrain its position within the minimap boundaries
  const left = Math.max(
    0,
    Math.min(scrollLeft * scaleX, mmWidth - minimapViewport.offsetWidth)
  );
  const top = Math.max(
    0,
    Math.min(scrollTop * scaleY, mmHeight - minimapViewport.offsetHeight)
  );

  minimapViewport.style.left = `${left}px`;
  minimapViewport.style.top = `${top}px`;
}

/**
 * Handles wheel scrolling manually (diagonal scrolling),
 * prevents default browser scrolling, and updates the minimap.
 */
function onWheelScroll(event) {
  event.preventDefault();
  scrollContainer.scrollLeft += event.deltaX;
  scrollContainer.scrollTop += event.deltaY;
  updateMinimapViewport();
}

/**
 * Generates a random hex color code.
 */
function randomColor() {
  const r = Math.floor(Math.random() * 256).toString(16).padStart(2, "0");
  const g = Math.floor(Math.random() * 256).toString(16).padStart(2, "0");
  const b = Math.floor(Math.random() * 256).toString(16).padStart(2, "0");
  return `#${r}${g}${b}`;
}
