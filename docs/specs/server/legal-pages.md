# Legal pages and Operating a public server

On this page: legal pages, operating a public server.

## Legal pages

`/legal` (operator, contact, jurisdiction, copyright agent), `/legal/terms`,
`/legal/privacy`, `/legal/content`, `/legal/notice` (form covering copyright and other illegal
content; answers with a receipt number, and about a private repository only to those who may read it), `/legal/notice/{receipt}` (state of the decision) and
`/legal/counter-notice`. The pages use no JavaScript and no third-party resources. A notice keeps
the sender's name, email and signature for `REIWAND_NOTICE_DAYS`, then only its content.

## Operating a public server

This is guidance for the person who runs an instance open to others, not legal advice. A server
used only by one household or one person needs none of it. What applies to you depends on where you
live, where your users are and whether the service is offered for money; ask someone qualified.

What the server does for you:

- It publishes `/legal` with the operator, the single contact point, the jurisdiction and the
  copyright agent, and it carries the same values into the terms. It shows only what you set.
- It has the notice form (`/legal/notice`) that covers copyright and other illegal content, a
  receipt number, a page for the state of each decision, the counter-notice form and the
  moderation queue with its decisions (see "Moderation and administration").
- It keeps an audit of every moderation action, the removal of held files after
  `REIWAND_TAKEDOWN_DAYS`, and the notice data for `REIWAND_NOTICE_DAYS`. `/api/v1/admin/transparency`
  gives counts without personal data for any report you owe.

What only you can do:

1. Fill `REIWAND_LEGAL_OPERATOR`, `REIWAND_LEGAL_CONTACT`, `REIWAND_LEGAL_JURISDICTION` and
   `REIWAND_LEGAL_DMCA_AGENT`. Empty fields leave `/legal` incomplete. Use a contact address that
   someone reads: authorities and users both write to it.
2. Where a safe harbour for copyright notices applies to you (the United States, for example),
   register the copyright agent with the competent office and keep that registration current. The
   server only prints the agent's name on `/legal`; it does not register anyone.
3. Where the law asks a service offered in a region to name a legal representative there and you
   operate from outside it, appoint one and list it in the contact field. The server cannot do it.
4. Answer notices within the time your law gives. Take down with the `remove` or `revoke` decision,
   write the statement of reasons the uploader will see, restore after a valid counter-notice, and
   use the strike option for repeat infringers. Keep the log: do not delete the audit.
5. Keep the moderation log and the notice data as long as your law asks, and no longer than
   `REIWAND_NOTICE_DAYS` allows for personal data; set that value on purpose.
6. If you learn of content that threatens someone's life or safety, or of sexual abuse of children,
   remove it at once and report it to the authorities yourself. The server has no automatic report.
7. Publish terms and a privacy statement you can stand behind; the generated pages are a starting
   point you must read and adapt.
