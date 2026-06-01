import { actionButton, field, heading, text } from "./dom.ts";

export function renderSetupTab(state, actions) {
  const section = document.createElement("section");
  section.className = "tab-panel setup";
  section.append(heading("Setup"));
  section.append(text("Create the first owner account for this qivxif data store."));
  section.append(setupForm(actions));
  if (state.setupError) {
    section.append(text(state.setupError, "error-text"));
  }
  return section;
}

function setupForm(actions) {
  const form = document.createElement("form");
  form.className = "setup-form";
  const nameLabel = field("Name", "text", "username");
  const passwordLabel = field("Password", "password", "new-password");
  const name = nameLabel.querySelector("input");
  const password = passwordLabel.querySelector("input");
  const submit = actionButton("Create owner account", () => {}, "primary");
  submit.type = "submit";
  form.append(nameLabel, passwordLabel, submit);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    actions.createOwner?.(name.value, password.value);
  });
  return form;
}
