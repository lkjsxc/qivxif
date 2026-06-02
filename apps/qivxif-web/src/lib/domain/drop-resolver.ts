const EDGE_FRACTION = 0.28;
const EDGE_MIN = 56;
const EDGE_MAX = 128;

export function measurePaneRects(pane, head, body) {
  const paneRect = pane.getBoundingClientRect();
  const bodyRect = body.getBoundingClientRect();
  const strip = head.querySelector(".tab-strip");
  const stripRect = strip?.getBoundingClientRect();
  return {
    bodyOffsetTop: bodyRect.top - paneRect.top,
    bodyRect,
    paneRect,
    stripBottom: stripRect?.bottom ?? head.getBoundingClientRect().bottom,
  };
}

export function resolvePaneDrop(input) {
  const { clientX, clientY, inSourceStrip, rects, targetPane } = input;
  const tabFrame = targetPane.querySelector(".tab-frame:hover, .tab.active");
  if (tabFrame) {
    return {
      kind: "rail",
      insertSide: tabInsertSide(tabFrame, clientX),
      targetPaneId: input.targetPaneId,
    };
  }
  if (inSourceStrip) {
    return { kind: "pane", targetPaneId: input.targetPaneId, zone: "center" };
  }
  const head = targetPane.querySelector(".pane-head");
  if (head) {
    const headRect = head.getBoundingClientRect();
    if (clientY <= headRect.bottom && clientX >= headRect.left && clientX <= headRect.right) {
      return { kind: "pane", targetPaneId: input.targetPaneId, zone: "center" };
    }
  }
  return {
    kind: "pane",
    targetPaneId: input.targetPaneId,
    zone: bodyDropZone(rects.bodyRect, clientX, clientY),
  };
}

export function tabInsertSide(tab, clientX) {
  const box = tab.getBoundingClientRect();
  return clientX < box.left + box.width / 2 ? "before" : "after";
}

function bodyDropZone(bodyRect, clientX, clientY) {
  const edgeX = Math.min(EDGE_MAX, Math.max(EDGE_MIN, bodyRect.width * EDGE_FRACTION));
  const edgeY = Math.min(EDGE_MAX, Math.max(EDGE_MIN, bodyRect.height * EDGE_FRACTION));
  if (clientY - bodyRect.top < edgeY) {
    return "top";
  }
  if (bodyRect.bottom - clientY < edgeY) {
    return "bottom";
  }
  if (clientX - bodyRect.left < edgeX) {
    return "left";
  }
  if (bodyRect.right - clientX < edgeX) {
    return "right";
  }
  return "center";
}

export function dropZoneToMove(zone) {
  if (zone === "center") {
    return "center";
  }
  return zone;
}

export function applyDropPreview(pane, body, rects, zone) {
  delete pane.dataset.dropZone;
  delete body.dataset.dropZone;
  if (!zone || zone === "center") {
    body.dataset.dropZone = "center";
    body.style.setProperty("--drop-offset", `${rects.bodyOffsetTop}px`);
    return;
  }
  body.dataset.dropZone = zone;
  body.style.setProperty("--drop-offset", `${rects.bodyOffsetTop}px`);
}

export function clearDropPreview(pane, body) {
  delete pane.dataset.dropZone;
  delete body.dataset.dropZone;
}
