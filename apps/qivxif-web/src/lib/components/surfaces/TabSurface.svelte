<script lang="ts">
  import SetupTab from "./SetupTab.svelte";
  import WelcomeTab from "./WelcomeTab.svelte";
  import GraphTab from "./GraphTab.svelte";
  import EditorTab from "./EditorTab.svelte";
  import BoardTab from "./BoardTab.svelte";
  import FeedTab from "./FeedTab.svelte";
  import PublishTab from "./PublishTab.svelte";
  import SyncTab from "./SyncTab.svelte";
  import SettingsTab from "./SettingsTab.svelte";
  import HistoryTab from "./HistoryTab.svelte";
  import DiagnosticsTab from "./DiagnosticsTab.svelte";
  import LoginTab from "./LoginTab.svelte";

  let { state: viewState, actions } = $props();
</script>

{#if viewState.setupRequired || viewState.activeTabId === "setup"}
  <SetupTab {viewState} {actions} />
{:else if viewState.activeTabId === "login"}
  <LoginTab {actions} />
{:else if viewState.activeTabId === "graph"}
  <GraphTab {viewState} {actions} />
{:else if viewState.activeTabId === "editor"}
  <EditorTab {viewState} {actions} />
{:else if viewState.activeTabId === "board"}
  <BoardTab {viewState} {actions} />
{:else if viewState.activeTabId === "publish"}
  <PublishTab {viewState} {actions} />
{:else if viewState.activeTabId === "social"}
  <FeedTab {viewState} {actions} />
{:else if viewState.activeTabId === "sync"}
  <SyncTab {viewState} {actions} />
{:else if viewState.activeTabId === "settings"}
  <SettingsTab {viewState} {actions} />
{:else if viewState.activeTabId === "history"}
  <HistoryTab {viewState} />
{:else if viewState.activeTabId === "diagnostics"}
  <DiagnosticsTab {viewState} />
{:else}
  <WelcomeTab {viewState} {actions} />
{/if}
