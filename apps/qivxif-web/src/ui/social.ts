export function renderSocialPane(state, actions) {
  const section = document.createElement("section");
  section.className = "social";
  section.append(subheading("Feed"));
  section.append(profileContext(state));
  section.append(postForm(actions));
  section.append(profileTargets(state, actions));
  section.append(relationshipEdges(state, actions));
  for (const item of state.feedItems ?? []) {
    section.append(feedRow(item.item ?? item));
  }
  return section;
}

function profileContext(state) {
  const box = document.createElement("div");
  box.className = "social-context";
  box.append(text(`Current profile: ${state.auth?.user?.profile_node_id ?? "signed out"}`));
  return box;
}

function profileTargets(state, actions) {
  const box = document.createElement("div");
  box.className = "social-targets";
  box.append(subheading("Profile targets"));
  const targets = targetProfiles(state);
  if (!state.auth?.user?.profile_node_id) {
    box.append(text("Sign in with a profile to manage relationships."));
    return box;
  }
  if (targets.length === 0) {
    box.append(text("No discovered profile targets."));
    return box;
  }
  for (const target of targets) {
    const row = document.createElement("div");
    row.className = "relationship-row";
    row.append(
      text(profileLabel(target)),
      command("Follow", () => actions.followProfile?.(target.id)),
      command("Mute", () => actions.muteProfile?.(target.id)),
      command("Block", () => actions.blockProfile?.(target.id)),
    );
    box.append(row);
  }
  return box;
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

function relationshipEdges(state, actions) {
  const list = document.createElement("div");
  list.className = "social-edges";
  list.append(subheading("Relationship edges"));
  const edges = activeSocialEdges(state);
  if (edges.length === 0) {
    list.append(text("No local relationship edges."));
    return list;
  }
  for (const edge of edges) {
    const row = document.createElement("div");
    row.className = "relationship-row";
    row.append(text(`${edge.kind}: ${edge.to_node}${edge.dirty ? " (dirty)" : ""}`));
    row.append(command(clearLabel(edge.kind), () => actions.clearSocialEdge?.(edge.id, clearKind(edge.kind))));
    list.append(row);
  }
  return list;
}

function targetProfiles(state) {
  const current = state.auth?.user?.profile_node_id;
  return (state.nodes ?? []).filter((node) => node.kind === "profile" && node.id !== current);
}

function activeSocialEdges(state) {
  const current = state.auth?.user?.profile_node_id;
  return (state.edges ?? []).filter((edge) => {
    return edge.from_node === current && ["follows", "mutes", "blocks"].includes(edge.kind) && !edge.tombstone;
  });
}

function clearKind(kind) {
  return {
    blocks: "social.unblock",
    follows: "social.unfollow",
    mutes: "social.unmute",
  }[kind];
}

function clearLabel(kind) {
  return {
    blocks: "Unblock",
    follows: "Unfollow",
    mutes: "Unmute",
  }[kind];
}

function profileLabel(node) {
  return node.metadata_map?.display_name ?? node.metadata_map?.name ?? node.id;
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
