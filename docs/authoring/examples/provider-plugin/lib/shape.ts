export interface Photo {
  id: number;
  title: string;
  kind: "photo" | "video";
  file: string;
  link: string;
  author: string | null;
  license: string | null;
}

export interface Answer {
  photos: Photo[];
  next_page: number | null;
}

export const item = (photo: Photo) => ({
  id: String(photo.id),
  title: photo.title,
  media: photo.kind === "video" ? "video" : "image",
  url: `https://images.example.org/${encodeURIComponent(photo.file)}`,
  page: photo.link.startsWith("https://") ? photo.link : null,
  author: photo.author,
  license: photo.license,
});
