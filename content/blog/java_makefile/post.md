---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--zndzOafr--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/6ggk4pvc8gdor4l6lr2m.jpg
date: 2020-04-08T12:00:00.000Z
title: Quick and Dirty Java Makefile
tags:
  - java
  - makefile
  - beginners
  - devjournal
---

I'm not a Java user and know next to nothing about it.  Today was my first time ever running a line of Java directly.  If you've *ever* used Java before, this is not going to blow your mind.

As part of my Ruby learning journey I'm working on translating some Java (specifically from the excellent [Crafting Interpeters](https://craftinginterpreters.com/) book), and while my Ruby implementation is the focus I want to be able to follow along locally with both languages.  However, I didn't want to spend a ton of time learning about a whole ecosystem that I'm not targeting.  Previously, I'd always assumed Java required bulky build tools and IDEs and lots of peripheral knowledge to hook everything up.  I thought I *needed* use a full-fledged tool like Eclipse or IDEA to set up all the details for you and had no sense of how the build process worked.

As it turns out, while you can lean on tooling, you can of course *also* just pop some Java code in a text file and build and run it by hand!  There is a `javac` CLI tool.  Either use it directly or use GNU make to invoke it automatically.

Here's what I'm starting with:

```makefile
JAVAC=javac
sources = $(wildcard *.java)
classes = $(sources:.java=.class)

all: program

program: $(classes)

clean:
 rm -f *.class

%.class: %.java
 $(JAVAC) $<

jar: $(classes)
 jar cvf program.jar $(classes)

.PHONY: all program clean jar
```

*edit: added phony targets!  Thanks Michael.*

With this, you can invoke `make` to compile a `Thing.class` bytecode file for each `Thing.java` source file present.  Then you get to run `java Thing` to execute it!  You can also use `make jar` to roll everything together into a jar file.

All in all, not so different from anything else I've ever used!

This is, as promised, "quick and dirty".  You can go forth from here and further customize your makefile, but if you want to do more than just quick experimentation, I do still think the IDE/Java-specific build tool route ([Maven](https://maven.apache.org/), [Gradle](https://gradle.org/), etc) is gonna be the way to go.  For non-Java-users who just want to plunk about without wasting too much time on peripherals, though, this'll do you just fine.

*Photo by Jakub Dziubak on Unsplash*
