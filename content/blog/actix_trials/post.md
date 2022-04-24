---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--E0qFrBnr--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/6byozbu5ld672kjcctwb.jpg
date: 2019-07-17T12:00:00.000Z
title: The Trials and Tribulations of actix-web and the OSS community
description: A lamentation on the situation of actix-web's maintainer vs. the Rust community.
tags:
  - rust
  - actix
  - discuss
---

EDIT: The crate maintainer has now merged the offending PR and acknowledged an understanding gap regarding what `unsafe` means in Rust. And so the wheel turns...

The Rust community is all up in arms about the `actix-web` crate - again. I won't rehash the problem - here's the instigating [blog post](https://64.github.io/actix/), the [reddit](https://www.reddit.com/r/rust/comments/ce09id/why_we_need_alternatives_to_actix/) discussion, and the [GitHub PR](https://github.com/actix/actix-web/pull/968) discussion that people think is problematic.

Last time around, there was an uproar around what was perceived to be cavalier usage of `unsafe`. Folks reviewed the code and found the usage to be unnecessary in a number of situations, and opened PRs to rewrite these portions using safe Rust without sacrificing performance.

This time around, the big bad is Undefined Behavior. A PR was opened that rewrites some unsafe code using safe Rust and as a side effect avoids some potential UB. The PR author provided an example of UB that this PR fixes.

The maintainer has opted not to merge the PR. Cue hellfire.

Now, some of this dissonance is Rust-specific. The `actix-web` maintainer is catching all this flak because he is making design decisions that are perceived to be at odds with the core tenets of what Rust is and why people choose it over existing alternatives - namely safety.

However, this seems to be a much more human problem than a technical one. The crate author and maintainer - `actix-web` is _largely_ the work of a single human - doesn't seem to be too concerned about this problem and can't be bothered to address it.

This begs the question: who does `actix-web` belong to? As an artifact of its infancy, the number of robust, production-ready web crates for Rust is small, and at this point `actix-web` is the _only_ option of its caliber that runs on stable Rust. Has this scarcity created an artificial sense of ownership on the part of the community? Or does this maintainer now actually have the real responsibility of reviewing and merging this PR he's personally uninterested in at the community's behest?

People are off-put by this maintainer's unwillingness to deal with this newly found UB problem. But is the maintainer in fact obligated to do so? Yes, the framework has a bunch o' stars and downloads, but that doesn't make it not this guy's project. The core rust async team is working on [their own](https://github.com/rustasync/tide) web framework, and with that crate we're having a different conversation. There is a reasonable expectation of quality from the Rust team. Here, though, the author has decided that he knows what he's doing and can write correct, performant code using `unsafe`, and this instance of UB doesn't bother him in the context of this application. He's also understandably standoffish after the whole `unsafe` debacle turned vitriolic and personal in nature last time around. I'd, too, be hesitant to act at the beck and call of what seems like a toxic and demanding community, for no compensation, to fix a problem that I don't personally perceive as a problem in the first place.

What's the solution? Never release software? I'd certainly hesitate to do so again were I this maintainer. Never rely on software that isn't supported by a large team? Often feasible, but not always, especially in a new and growing ecosystem like Rust.

Regardless of how you feel about "the spirit of Rust" and how software _should_ be architected with it, it's undeniable that `actix-web` is a good piece of software. It's fast and easy to use, even if it doesn't look inside like code you'd write in Rust. I don't know what the code to most libraries and frameworks I use looks like, though. When was the last time you opened up something like [`cairo`](https://cairographics.org/), a foundational piece of software that's widely distributed. Should its code style dictate whether or not you add it to your app's dependency tree?

This is a hard problem. I don't think there's "an answer". How do you approach these sorts of situations in the ecosystem of your choice?

Photo by Hermes Rivera on Unsplash
