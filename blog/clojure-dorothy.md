---
cover_image: https://i.imgur.com/b7yQAMB.png
edited: 2019-02-23T12:00:00.000Z
title: Create Graphviz graphs in Clojure with dorothy
published: true
description: A quick tutorial for using dorothy to define and render Graphviz graphs
tags: clojure, beginners, graphviz
---
Quick post today - learning this made me feel like I levelled up, so I thought I'd share.

I [recently decided](https://dev.to/deciduously/back-to-school-57fd) to remove the "self" from self-taught and go back to school for software development.  To nobody's surprise at all, my first semester includes a Discrete Mathematics class, and this week we looked at [Hasse diagrams](https://en.wikipedia.org/wiki/Hasse_diagram), used to represent [partially ordered sets](https://en.wikipedia.org/wiki/Partially_ordered_set).  This is just the sort of diagram [`graphviz`](https://graphviz.org/) is designed for!

Being a C library, Graphviz has interfaces in just about any language you could hope for.  Python is a good choice for quick one-offs like this, but it's also a natural fit for Clojure!  Also to nobody's surprise at all, the community has created a library for defining Graphviz graphs using Clojure data structures, like what [Hiccup](https://github.com/weavejester/hiccup) does for HTML.  It's called [`dorothy`](https://github.com/daveray/dorothy).

To follow along, you'll need to install [leiningen](https://leiningen.org/) and [graphviz](https://graphviz.org/download/).  Once both are installed, create a new project:

```
$ lein new app hasse
```

Open up the `hasse` folder in your favorite text editor and find `project.clj`.  We just need to add the `dorothy` dependency.  Locate the `:dependencies` map and make it look like this:

```clojure
  :dependencies [[org.clojure/clojure "1.9.0"]
                 [dorothy "0.0.7"]]
```

Now run `lein deps` to pull in the jar and open up `src/hasse/core.clj`.  Below the `(ns)` form, add the following require statements:

```clojure
(ns hasse.core
  (:gen-class))
(require '[dorothy.core :as dot])
(require '[dorothy.jvm :refer (render save!)])]
```

To create a graph, you just define your nodes and edges - and in this case, all we need to do is define edges.  Graphviz will take care of everything else.  Remove the body of `-main` and add a `let` binding to define our graph:

```clojure
  (let [g (dot/graph [
    [:22 :2]
    [:8 :2]
    [:10 :5 :1]
    [:10 :2 :1]])])
```

There is also a `dot/digraph` which will create directed edges.  Each keyword is a node, and each list is an edge connecting two or more nodes.  Each node can take an attribute map as well, I'm just using the defaults for everything here.  Now that we've defined the graph, we use the `(dot/dot)` function to convert it to the Graphviz dot format.  We can then use `save!` to save our result:

```clojure
(-> g dot/dot (save! "out.png" {:format :png}))
```

There is also a `show!` in `dorothy.jvm` which uses a simple Swing viewer - useful for testing.  Your full snippet should look like this:

```clojure
(ns hasse.core
  (:gen-class))
(require '[dorothy.core :as dot])
(require '[dorothy.jvm :refer (render save!)])

(defn -main
  [& args]
  (let [g (dot/graph [
    [:22 :2]
    [:8 :2]
    [:10 :5 :1]
    [:10 :2 :1]])]
    (-> g dot/dot (save! "out.png" {:format :png}))))
```

Now run `lein uberjar` to compile the Clojure, execute `java -jar target/uberjar/hasse-0.1.0-SNAPSHOT-standalone.jar`, and marvel at the beauty:

![graph](https://i.imgur.com/b7yQAMB.png)

Nifty!  That sure as heck is a Hasse diagram of the relation "divides" on the set {1,2,5,8,10,22}, I tell you hwat.

This is usable from ClojureScript as well, but without the rendering and saving functions - you'll need to rely on another library to get your dot format output to something visual.

Happy diagrammin'!
