export function renderSocialPane(state, actions) {
  const section = document.createElement("section");
  section.className = "social";
  section.append(subheading("Feed"));
  section.append(postForm(actions));
  for (const item of state.feedItems ?? []) {
    section.append(feedRow(item.item ?? item));
  }
  return section;
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
