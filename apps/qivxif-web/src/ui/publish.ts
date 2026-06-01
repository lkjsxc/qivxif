export function renderPublishPane(state, actions) {
  const section = document.createElement("section");
  section.className = "publish";
  section.append(subheading("Publish"));
  section.append(draftForm(actions));
  if (state.currentBlogPostId) {
    const post = state.currentBlogPost;
    section.append(text(`Draft: ${post?.metadata_map?.title ?? state.currentBlogPostId}`));
    section.append(text(`State: ${post?.metadata_map?.publication_state ?? "queued"}`));
    section.append(publishForm(actions));
    section.append(actionButton("Unpublish", () => actions.unpublishBlogPost?.()));
  }
  if (state.lastPublicRoute) {
    section.append(text(state.lastPublicRoute));
  }
  return section;
}

function draftForm(actions) {
  const form = document.createElement("form");
  const title = field("Blog title", "text");
  const titleInput = title.querySelector("input");
  const submit = button("Create blog draft");
  form.append(title, submit);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    actions.createBlogDraft?.(titleInput.value);
  });
  return form;
}

function publishForm(actions) {
  const form = document.createElement("form");
  const slug = field("Slug", "text");
  const summary = field("Summary", "text");
  const slugInput = slug.querySelector("input");
  const summaryInput = summary.querySelector("input");
  const submit = button("Publish draft");
  form.append(slug, summary, submit);
  form.addEventListener("submit", (event) => {
    event.preventDefault();
    actions.publishBlogPost?.(slugInput.value, summaryInput.value);
  });
  return form;
}

function actionButton(label, handler) {
  const element = button(label);
  element.className = "command";
  element.type = "button";
  element.addEventListener("click", handler);
  return element;
}

function button(label) {
  const element = document.createElement("button");
  element.type = "submit";
  element.textContent = label;
  return element;
}

function field(label, type) {
  const wrapper = document.createElement("label");
  wrapper.textContent = label;
  const element = document.createElement("input");
  element.type = type;
  wrapper.append(element);
  return wrapper;
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
