export function installKeyboardShortcuts(actions, state) {
  document.addEventListener("keydown", (event) => {
    if (isEditing(event.target)) {
      return;
    }
    if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "k") {
      event.preventDefault();
      actions().toggleCommandPalette?.();
      return;
    }
    if (event.key === "Escape" && state.commandPaletteOpen) {
      event.preventDefault();
      actions().toggleCommandPalette?.(false);
    }
  });
}

function isEditing(target) {
  const tag = target?.tagName?.toLowerCase();
  return tag === "input" || tag === "textarea" || target?.isContentEditable;
}
