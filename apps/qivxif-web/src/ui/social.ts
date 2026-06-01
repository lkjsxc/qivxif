export function renderSocialPane(state, actions) {
  const section = document.createElement("section");
  section.className = "social";
  section.append(subheading("Feed"));
  section.append(postForm(actions));
  section.append(relationshipForm(actions));
  section.append(clearRelationshipForm(actions));
  section.append(edgeList(state));
  for (const item of state.feedItems ?? []) {
    section.append(feedRow(item.item ?? item));
  }
  return section;
}

function relationshipForm(actions) {
  const form = document.createElement("form");
  const label = document.createElement("label");
  label.textContent = "Target profile node";
  const input = document.createElement("input");
  label.append(input);
  form.append(
    label,
    command("Follow", () => actions.followProfile?.(input.value)),
    command("Mute", () => actions.muteProfile?.(input.value)),
    command("Block", () => actions.blockProfile?.(input.value)),
  );
  form.addEventListener("submit", (event) => event.preventDefault());
  return form;
}

function clearRelationshipForm(actions) {
  const form = document.createElement("form");
  const label = document.createElement("label");
  label.textContent = "Relationship edge";
  const input = document.createElement("input");
  label.append(input);
  form.append(
    label,
    command("Unfollow", () => actions.clearSocialEdge?.(input.value, "social.unfollow")),
    command("Unmute", () => actions.clearSocialEdge?.(input.value, "social.unmute")),
    command("Unblock", () => actions.clearSocialEdge?.(input.value, "social.unblock")),
  );
  form.addEventListener("submit", (event) => event.preventDefault());
  return form;
}

function postForm(actions) {
  const form = document.createElement("form");
  const label = document.createElement("label");
  label.textContent = "Short post";
  const input = document.createElement("textarea");
  input.rows = 3;
  const submit = document.createElement("button");
  submit.type = "submit";
  submit.textContent = "Create short post";
  label.append(input);
  form.append(label, submit);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    actions.createShortPost?.(input.value);
  });
  return form;
}

function feedRow(item) {
  const row = document.createElement("article");
  row.className = "feed-item";
  row.append(text(item.author_name ?? "unknown"));
  row.append(text(item.body ?? ""));
  return row;
}

function edgeList(state) {
  const list = document.createElement("div");
  list.className = "social-edges";
  for (const edge of state.edges ?? []) {
    if (["follows", "mutes", "blocks"].includes(edge.kind)) {
      list.append(text(`${edge.kind}: ${edge.id}${edge.dirty ? " (dirty)" : ""}`));
    }
  }
  return list;
}

function command(label, handler) {
  const button = document.createElement("button");
  button.className = "command";
  button.type = "button";
  button.textContent = label;
  button.addEventListener("click", handler);
  return button;
}

function subheading(value) {
  const element = document.createElement("h2");
  element.textContent = value;
  return element;
}

function text(value) {
  const element = document.createElement("p");
  element.textContent = value;
  return element;
}
