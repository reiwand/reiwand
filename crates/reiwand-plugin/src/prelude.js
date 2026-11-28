((raw) => {
  const words = (headers) =>
    Object.entries(headers ?? {}).map(([name, value]) => [String(name), String(value)]);
  const fetch = async (url, init = {}) => {
    const asked = {
      url: String(url),
      method: String(init.method ?? "GET").toUpperCase(),
      headers: words(init.headers),
      body: init.body == null ? null : String(init.body),
    };
    const answer = JSON.parse(raw(JSON.stringify(asked)));
    const headers = new Map(Object.entries(answer.headers));
    return Object.freeze({
      status: answer.status,
      ok: answer.status >= 200 && answer.status < 300,
      url: answer.url,
      headers: Object.freeze({ get: (name) => headers.get(String(name).toLowerCase()) ?? null }),
      text: async () => answer.body,
      json: async () => JSON.parse(answer.body),
    });
  };
  Object.defineProperty(globalThis, "fetch", { value: fetch, enumerable: false });
})
