export function actionButton(label, handler, className = "command") {
  const button = document.createElement("button");
  button.className = className;
  button.type = "button";
  button.textContent = label;
  button.addEventListener("click", handler);
  return button;
}

export function field(label, type, autocomplete = "") {
  const wrapper = document.createElement("label");
  wrapper.textContent = label;
  const input = document.createElement("input");
  input.autocomplete = autocomplete;
  input.type = type;
  wrapper.append(input);
  return wrapper;
}

export function heading(value, level = 1) {
  const element = document.createElement(`h${level}`);
  element.textContent = value;
  return element;
}

export function text(value, className = "") {
  const element = document.createElement("p");
  if (className) {
    element.className = className;
  }
  element.textContent = value;
  return element;
}

export function panel(className, ...children) {
  const section = document.createElement("section");
  section.className = className;
  section.append(...children);
  return section;
}
