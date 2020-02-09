---
cover_image: https://storage.googleapis.com/programming-idioms-pictures/idiom/149/princess-lisp.png
edited: 2018-11-23T12:00:00.000Z
title: Reactive Canvas with ClojureScript and Vue
published: true
description: How to create a reactive canvas component with ClojureScript and Vue
tags: clojure, vue, beginners, webdev
---
# Or How I Learned To Stop Worrying And Ditch Custom Directives

Since writing my post [Reactive Canvas with TypeScript and Vue](https://dev.to/deciduously/reactive-canvas-with-typescript-and-vue-1ne9) I've discovered [`glue`](https://github.com/Gonzih/glue), a library for defining [Vue](https://vuejs.org/) components in [ClojureScript](https://clojurescript.org/).  Ever the hipster, I had to give it a spin.  This post details the same functionality as that post but using ClojureScript instead of TypeScript.

## Setup

To start, you'll need to have a [JDK](https://openjdk.java.net/install/) installed.  You'll also need to obtain [`leiningen`](https://leiningen.org/) which provides package managment and build tooling for Clojure/ClojureScript.

Once you've installed the above navigate to your project directory and issue: `lein new figwheel rxcanvas-cljs`.  Navigate to your new folder `rxcanvas-cljs` and open up `project.clj`.  We just need to make one change.  Find your `:dependencies` key and make it look like this:

```clojure

:dependencies [[org.clojure/clojure "1.9.0"]
              [org.clojure/clojurescript "1.10.238"]
              [org.clojure/core.async  "0.4.474"]
              [glue "0.1.3-SNAPSHOT"]]
```

We've just added `glue` to the list.  Don't worry too much if your version numbers don't match exactly - this is just what the template came with on the date of this writing.

Now we execute `lein figwheel`.  The first run will be the longest as it gathers dependencies.  When it loads, open your browser to `localhost:3449`.  When the page loads you should see the REPL prompt appear in your terminal - try issuing `(js/alert "Hello from ClojureScript")`:

```
// ...
[Rebel readline] Type :repl/help for online help info
ClojureScript 1.10.238
dev:cljs.user=> (js/alert "Hello from ClojureScript")
```

You should see the requested alert in your browser.  Leave this running as you develop and when you're ready to close type `:cljs/quit` at the REPL prompt.

If you're new to [`figwheel`](https://figwheel.org/) take a moment to familiarize yourself with the blank project layout.  There's not too much here.  The `dev` directory just sets up some convenience functions, and our HTML and CSS will live in `resources/public`.  It has pre-populated a `.gitignore` and a `README.md` for you.  All of our logic will live in `src/rxcanvas_cljs/core.cljs`.

## Add a template

We're not using Single-File Components.   This would currently involve some non-trivial DIY plumbing.  There's no `vue-loader` equivalent to do the parsing for us yet - you could write the first!  If I'm wrong about this, somebody pipe up below.

We're just going to keep our template separate.  Open up `resources/public/index.html`.  The figwheel template comes with a `div` with the id `app`.  We'll keep the div but replace the contents:

```html
<div id="app">
  <rxcanvas></rxcanvas>
</div>
```

Now we can use the `<template>` tag to define our resizable dot component.  Place this above the `app` div, directly following the opening `<body>` tag:

```html
<template id="rxcanvas">
  <div>
    <span>{ size }</span>
    <input type="range" min="1" max="100" step="5" id="size" @change="drawDot">
    <label for="size">- Size</label>
    <p><canvas id="rx"></canvas></p>
  </div>
</template>
```

There are two changes from the TypeScript.  For one, I've replaced `v-model="size"` in the `range` tag with `@change="drawDot"`.  This method will handle updating our state.  I've also ditched the custom directive in the `<canvas>` tag, instead just assigning an id.

## Add some Lisp

Now we get to the good stuff.  Open up `src/rxcanvas_cljs/core.cljs`.  First, we need to override the built-in `atom` with the one `glue` provides and bring the rest of the library into scope.  Add the following to your `ns` form at the top of the file:

```clojure
(ns rxcanvas-cljs.core
    (:refer-clojure :exclude [atom])
    (:require [glue.core :as g :refer [atom]]))
```

Leave in the `(enable-console-print!)` line at the top of the file - this allows us to use the browser console for output with `println` should we so choose - but delete everything else.

We'll start with the mount point:

```clojure
(defonce app (g/vue {:el "#app"})
```

This locates the `<div id="app">` from `index.html` and mounts our Vue stuff to it.  We also need to make sure it keeps itself refreshed - add the following below:

```clojure
(defn on-js-reload []
  (g/reset-state!))
```

ClojureScript is not object-oriented like TypeScript, so we'll just define a plain old function to handle the canvas drawing logic instead of a `Dot` class.  Put this above your `app` definition:

```clojure
(defn draw
  [radius canvas]
  (let [canvas-dim (* 2 radius)]
    ;; resize canvas
    (set! (.-width canvas) canvas-dim)
    (set! (.-height canvas) canvas-dim)

    ;; draw the shape
    (let [ctx (.getContext canvas "2d")
          center-x (/ (.-width canvas) 2)
          center-y (/ (.-height canvas) 2)]
      (set! (.-fillStyle ctx) "rgb(0,0,0)")
      (.clearRect ctx 0 0 (.-width canvas) (.-height canvas))
      (.beginPath ctx)
      (.arc ctx center-x center-y radius 0 (* 2 (.-PI js/Math)) false)
      (.fill ctx)
      (.stroke ctx))))
```

Interop is dirt simple - you just put the method in the first position of the s-expression.  You can get and set properties via syntax like `(.-PI js/Math)`.  It's rather easy to get addicted to the hyper-regular syntax.

Now we're ready to define the component itself.  With `glue` we use `defcomponent`, right below `draw`:

```clojure
(g/defcomponent
  :rxcanvas
  {:template "#rxcanvas"
   :state (fn [] {:size (atom 10)})
   :methods {:draw-dot (fn [this state _]
      ;; update the state
      (reset! (:size state) (.-value (.querySelector js/document "#size")))
      ;; grab the new value and the canvas for drawing
      (draw @(:size state) (.querySelector js/document "#rx"))
      )}})
```

Instead of `data()` we're using the key `:state` but it still returns a function.  We've explicitly stored the `size` in an `atom`, ClojureScript's mechanism for allowing mutability in an otherwise immutable language.  This particular `atom`, as discussed, is from `glue` and has some extra goodness built in to ease use in Vue components.  Using it we can access `size` using simple forms like `(:size state)`.

Also note - in our template we style the method name `drawDot`, and in our ClojureScript it's called `draw-dot`.  This is another part of what `glue` is [handling](https://github.com/Gonzih/glue/blob/8df738c2e6256b914998bb1537f85ff4bef2e4e5/src/glue/core.cljs#L50)!

We need the `@` operator as in `@(:size state)` to get at the current value of the `atom` in our call to `draw`.

That's it!  Now our canvas will resize and redraw on each change to our slider.

The completed code can be found [here](https://github.com/deciduously/rxcanvas-cljs).
