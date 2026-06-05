export type MarkdownBlock = { kind: "h1" | "h2" | "p"; text: string };

export function markdownBlocks(source: string): MarkdownBlock[] {
  const blocks = source
    .replace(/\r\n/g, "\n")
    .split(/\n{2,}/)
    .map((block) => block.trim())
    .filter(Boolean);
  return blocks.map(markdownBlock);
}

export function markdownBlock(block: string): MarkdownBlock {
  if (block.startsWith("## ")) return { kind: "h2", text: block.slice(3).trim() };
  if (block.startsWith("# ")) return { kind: "h1", text: block.slice(2).trim() };
  return { kind: "p", text: block };
}

export function textStats(source: string) {
  const trimmed = source.trim();
  return {
    characters: source.length,
    words: trimmed ? trimmed.split(/\s+/).length : 0,
  };
}

export function matchCount(source: string, query: string) {
  const needle = query.trim().toLowerCase();
  if (!needle) return 0;
  let count = 0;
  let index = source.toLowerCase().indexOf(needle);
  while (index >= 0) {
    count += 1;
    index = source.toLowerCase().indexOf(needle, index + needle.length);
  }
  return count;
}
