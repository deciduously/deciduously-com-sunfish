---
title: Stencil: I Think I Found My Frontend Home
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--2JthHaiM--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/314jlc0r54roahmfzkxh.jpg
tags: webdev, typescript, beginners, devjournal
published: true
edited: 2020-01-08T12:00:00.000Z
---
## Getting Real

I've just started exploring [`stencil`](https://stenciljs.com/) and am already nursing a crush.

I will be the first to admit that I don't particularly enjoy building frontends.  It's the reason my own [personal website](http://deciduously.com/) is, well, sad.  It's been incomplete and neglected since the day I forced it out in the open, and largely untouched from that pathetic, half-finished state. This means it does not reflect any recent work I have to show off, at all, or accurately demonstrate what I'm capable of in any significant way.  Useful, right?  Why even have one, at this point.

I built that website as a side-effect of a DIY [static site generator](https://github.com/deciduously/deciduously-com-rs) exploratory project in Rust.  The fun part of that project for me was building the static site generator and web server.  To see if it all worked I needed some test stuff to feed it so I figured I *may as well* build a personal website.  Poor thing never stood a chance - talk about falling short of even the bare minimum.

One of my goals for 2020 is to completely rebuild it into something that isn't embarrassing, and hopefully hone a useful skill and learn more about the domain along the way.

## Runners Up

Choosing a stack in the JavaScript world is daunting, especially when you don't actually know what you're doing.  Before settling on `stencil` I considered the following options:

* [HTML](https://whatwg.org/)/[CSS](https://www.w3.org/Style/CSS/)

My "aesthetic" for tooling choices is barebones and simple.  I prefer to build my own abstractions, at least when I'm learning a domain, before trying to choose someone else's, and also think that for this website even vanilla JS is probably overkill, everything I want to present can be presented statically and simply.  All I really need is HTML and CSS.  I decided not to go this route because I do want to open myself up to more flexible growth paths in the future, and this is the most labor-intensive route if I go beyond the absolute basics.  I also find it extremely boring to do, which means I'd be less likely to finish.

* [Gatsby](https://www.gatsbyjs.org/)

The framework I've spent the most time with is React, so Gatsby was a natural choice for the next level of organization.  I liked it a lot, and developed a healthy appreciation for [GraphQL](https://graphql.org/), but ultimately found it was like "React+" - you're really on your own in terms of ecosystem, and there is a *ton* of complexity going on to achieve what (for me) is a simple end goal, even after I got my head around the basic model (both React itself and then Gatsby over it).  I still think highly of Gatsby in general and would revisit for a larger, more complicated project.

* [Svelte](https://svelte.dev/)/[Sapper](https://sapper.svelte.dev/)

I finally looked at Svelte, because I like the idea of an AOT compiler handling this complexity rather than a bunch of complex runtime logic.  I didn't spend a lot of time with it, though, because it looked like a whole new set of stuff to learn, as well designed as it is, and that's precisely what I want to avoid.  The Sapper [repo](https://github.com/sveltejs/sapper) looks a little dead, too, but that's not always a valid indicator.  I'm keeping my eye on this as well, though.

## Why Stencil Wins

I want to be crystal clear: I am not saying that `stencil` "wins".  I am only saying it wins *for me, for this project*.  I know I have a tendency to prod at controversial questions on DEV to see what happens, but this is not one of those posts.

Stencil is (mostly) just "Web".  It's actually not a "framework" at all.  Instead, it's a generator and compiler for [Web Components](https://developer.mozilla.org/en-US/docs/Web/Web_Components), with some extra niceness built-in to streamline the process.

The Web is an inherently complicated platform, and frontend is not my primary target domain, so I do want some help taming that complexity.  What I don't want is for that help to consist of a whole new set of specifics to learn which may or may not apply to anything else, and which has no guarantee of lasting relevance.  That's been my experience with [JQuery](https://jquery.com/), [Knockout](https://knockoutjs.com/), [Ember](https://emberjs.com/), [Meteor](https://www.meteor.com/), [React](https://reactjs.org/), [Vue](https://vuejs.org/), [Svelte](https://svelte.dev/), whatever else I've touched - even just listing stuff I've test driven makes my head spin and it's all new layers on top of an already complicated topic.

Now, it's a valid point that Web Components may very well represent yet another step along this chain.  There's a phenomenal discussion here:

{# {% post richharris/why-i-don-t-use-web-components-2cia %} #}

My favorite quote from that discussion came from [@josepot](https://dev.to/josepot), who noted that "web-components are just leaky abstractions built on top of other leaky abstractions".  That's hard to argue with.  Still, though, what isn't?

I like how Stencil is focused first and foremost on compliance with existing standards.  To write my app, I need to learn TypeScript, not some specific other tool.  I also love how easy it's made it for me to piece together my app.  The `stencil generate` tool can generate just your TSX or include a CSS stylesheet, a [Jest](https://jestjs.io/) `component.spec.ts` file, and a [Puppeteer](https://pptr.dev/) `component.e2e.ts` file.  I'm forcing myself to use all of these for every component as an educational exercise, E2E testing in specific is something I've never experimented with, but it's great that it's all opt-in and you could keep it straightforward as long as you want.

Within minutes of loading up the template app I had added a new component and hooked it up to a new route with the handy built-in [`stencil-router`](https://github.com/ionic-team/stencil-router) component, purely based on what I already knew about HTML templates going in and the example provided.  Sold!

## Example Component

For a simple example, here's a component I put together to display a United States postal address:

```tsx
import { Component, Prop, h } from '@stencil/core';
import { Address } from '../../cvdata';

@Component({
  tag: 'app-cv-address',
  styleUrl: 'cv-address.css',
  shadow: true
})
export class CvAddress {
  @Prop() address: Address;

  render() {
    if (this.address) {
      return (
        <p itemscope itemtype="https://schema.org/PostalAddress" id="address">
          <span itemprop="streetAddress">{this.address.street}</span><br />
          <span itemprop="addressLocality">{this.address.locality.name}</span>, <abbr title={this.address.locality.state.fullName} itemprop="addressRegion">{this.address.locality.state.abbreviation}</abbr> <span itemprop="postalCode">{this.address.locality.postalCode}</span><br />
          <span itemprop="addressCountry">{this.address.locality.state.country}</span>
        </p>
      );
    }
  }
}
```

It's just TypeScript, or an ES6 class, with some metadata stored in a decorator.  [JSX](https://reactjs.org/docs/introducing-jsx.html) is a polarizing tool, but I think it's a good fit for the problem it solves.  I've never used [Angular](https://angular.io/) but I think the "metadata-in-a-decorator" pattern is not entirely dissimilar.

This is the over-engineered `Address` TS interface I defined to pass in as a prop:

```ts
interface AddressRegion {
  fullName: string,
  abbreviation: string,
  country: string,
}

interface Locality {
  name: string,
  state: AddressRegion,
  postalCode: string,
}

export interface Address {
  street: string,
  locality: Locality
}
```

This is precisely why I like using TypeScript over JavaScript - familiarity.  I'm used to tools that let me define the shape of my data up front, and get frustrated when debugging problems in vanilla JS that would have been caught by a typechecker.  By setting this up before implementing, it's easy for me to dig through the object passed in as a prop.  I know it's not technically any different than doing so without a defined interface, but I like it, dammit.  Having my editor know what I'm doing is a huge help - I like my red squigglies showing me when I'm being stupid.  Typechecked props by default is just awesome.

This was easy to pop in the larger context:

```tsx
render() {
    return (
      <div>
        {/* other stuff... */}
        <app-cv-address address={this.data.address}></app-cv-address>
      </div>
    );
  }
```

No need to import anything, just use your new component!  It couldn't be easier, and I'm off to the races.

Beyond the perceived technical merits of this tool, beginning to build this simple application with `stencil` is pretty much the *only* time I've ever actually enjoyed the process of writing a frontend application.  I thought Gatsby was incredibly cool tech, and Svelte was a solid implementation of an elegant idea, but "fun" is a little more elusive for me in this space.

Why exactly this tool got me there when so many relatively similar others haven't is tricky for me to pinpoint, but I'm not fighting it.  It's simple to get started with, based around core Web technologies that I know will continue to be relevant, and will allow complexity to grow as needed as opposed to including it up-front in a massive convoluted "starter app" template.  I'm hoping this tool stays around for a long time and see no reason to hop around, at least for personal use, for the foreseeable future.

*Photo by Karim MANJRA on Unsplash*
