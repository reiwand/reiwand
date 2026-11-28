# Low band of the title screens, Sources and Settings

On this page: low band of the title screens, sources, settings.

## Low band of the title screens

A title page can ask for the low band: a margin-to-margin strip under the menu (filter words, shelf,
card of the chosen one) and, if it needs it, a block above the band to the right of the menu. A page
that does not ask for it stays as always, in the column to the right of the menu.

- The page declares `bandLow` (the minimum height with which its band looks good; greater than zero
  turns it on), `bandHigh` (optional: the height it wants at most; without it the band reaches up to
  the menu) and `upperHeight` (optional: the height of its upper block).
- The page also declares `property rect band` and `property rect upper`; the skeleton fills them in
  the coordinates of the page, which takes the whole window. The page puts its band in `band` and its
  block in `upper`, without knowing the window's measures.
- With room: `band` goes from the left margin to the right, stuck to the bottom margin, with a height
  between `bandLow` and `bandHigh` depending on what there is under the menu; `upper` goes to the
  right of the menu, just above the band, with the requested height. Shortcuts stay in the bottom
  right corner of the band.
- If the band does not fit in the normal size but does in the compact one, the window goes to the
  compact size even if it is taller than 800 px, and goes back to normal when it grows to the height
  at which the band fits there too. That way there are no heights at which the band is missing
  between the minimum at which it fits (about 700 px on the sample sheet) and any larger height.
  That size holds for all pages at once, also Sources, Settings and Account: the menu on the left
  does not change place or size when going from one page to another. It is decided by the tallest
  band measured in the session, so the first time a page that asks for more height is visited the
  menu may change just once.
- The window does not go below about 1100 × 680 px (62 × 38 grid units): below that, pages such as
  Explore get cut. A button that changes its action in the same place (for example Install turning
  into "Set as wallpaper") ignores the mouse and Enter or Space for half a second, so that the second
  click of a double click does not do what the user did not ask for.
- Without room (a short window, or the upper block does not fit): both go to the column to the right
  of the menu, the block on top and the band below, as in the usual pages. The change happens on
  resizing, without moving the page: focus stays where it was and nothing covers the menu.
- The band veil covers the window across from 190 px above the band. If the text of the band starts
  lower than `band`, the page declares `bandFrom` (in its coordinates) and the veil starts 190 px
  above that point, so it reads over a light background. A page without a band can also declare
  `bandFrom` (the Library, at the point of its bar): the veil rises up to 190 px above that point,
  but never starts lower than in the other pages.
- A page that takes the whole height (`full`) does not use the band.
- With the band, notices go to the right of the menu, above the upper block.

## Sources

Each online provider is a card: name, state, "N photos, updated … ago", the large "Rotate these
photos" button (creates or reuses the list `source:<source>` and starts it on all outputs),
"Refresh" and "Details". If there is none, the page shows the type cards to add one; Bing, NASA and
Wikimedia are added with one click, the others first ask for what they need (search, server, key).
Keys are never shown again. A source in error says its own problem on its card. Only when a source
does not reach its server (`offline` of the daemon) does the list show at the top the notice "No
connection…" with "Retry", and that card goes on with its normal summary. On a Wallhaven source,
"More options" opens on the same page two groups without scrolling: "What photos" (categories,
content and size and colors, each with its level of checkboxes) and "How to find them" (order with
"Shuffle again" when it is random, collections of a user, similar to a photo and "Other search
options" with the words, the direction, the period, the pages and the cap). Each change is saved at
once; Esc goes back a level. Colors are named by their code. Saving the keys in a private file, when
there is no keyring, is confirmed in a small panel of its own.

In the details of a source, "Stored photos" says how much space what it brought takes and offers
"Clear…": the panel "Delete the stored photos?" deletes those photos from the computer and the
library (`Providers1.ClearCache`, `reiwand-ctl provider clear-cache ID`); the source keeps its
options and its key and brings new photos on the next refresh. "Remove this source" also removes the
source.

## Settings

- Three tabs: Wallpapers, Screens and Device. Each tab declares its rows in `rows` (`{text, value,
  place, explain, off}`): with that the header's search box filters the rows of all three by their
  label, ignoring case and accents, and on choosing a result it opens the tab and the place of the
  row and gives it focus. Without results it says "No setting speaks of…" and proposes words to try.
- Each row that opens something leads to a place with a path and "Esc back": More, Library folders,
  Lists per screen, Pause, More of the screen and Layouts.
- Screens shows the quality of scenes (Automatic, Save battery, Maximum; the seven levels under "More
  of…"), the transition of each screen with the reason for the last change, the general one, Less
  motion, Pause and Layouts. A row whose hardware is not detected (the battery) is gray with "not
  detected", and what the person set is kept.
- Layouts: each is a row with its rules; Enter offers Use it, Automatic rules, Rename and Delete it.
  When a screen is connected or removed, if no layout stores that set, a notice offers to save it
  and saves it with those screens as the rule.
- The row of each library folder offers Check it again, Rotate these photos (a list with that folder
  on all screens) and Remove it, with confirmation.
- Device starts with the "Private mode" switch (see "Private mode") and summarizes in five rows
  Scene permissions (how many scenes have any), Sound of the scenes, What you are listening to, RGB
  lights and Advanced; each opens its place. In Permissions each permission of each scene is a row
  that withdraws it with Enter. Lights are gray with "not detected" if they are on and there is no
  server or devices; their place remains at hand from the search box ("OpenRGB") to change the
  address or turn them off.
- Advanced groups the graphics card (Automatic, any integrated, any dedicated or a specific one;
  "dedicated card: not detected" if there is none), HDR (gray with "not detected" if no screen
  supports it and it is not at Never), Depth quality, Maintenance (Reload the settings, Check the
  whole library), the version with its details and For tests (surface name, draw with the
  processor). The places inside Advanced carry the full path and "Esc back" returns to Advanced.
