import { item } from "./lib/shape.ts";
import type { Answer } from "./lib/shape.ts";

type Settings = { query: string; per_page: number; videos: boolean };

export async function list({ settings, page }: { settings: Settings; page: string | null }) {
  const pairs = { q: settings.query, per_page: String(settings.per_page), page: page ?? "1" };
  const query = Object.entries(pairs)
    .map(([key, value]) => `${key}=${encodeURIComponent(value)}`)
    .join("&");
  const answer = await fetch(`https://api.example.org/v1/photos?${query}`, {
    headers: { Accept: "application/json" },
  });
  if (!answer.ok) throw new Error(`the gallery answered ${answer.status}`);
  const found = (await answer.json()) as Answer;
  const items = found.photos.filter((p) => settings.videos || p.kind === "photo").map(item);
  return { items, next: found.next_page === null ? null : String(found.next_page) };
}
