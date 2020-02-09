---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--iGTJRjd2--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/teykkpgt1lebje46zar2.jpg
edited: 2019-03-08T12:00:00.000Z
title: How I Emacs And So Can You
published: true
description: An overview of Emacs
tags: beginners, emacs, tutorial
---
Emacs is a *whole thing*.  It's a great tool to have in your belt, though, and nobody ever sat me down and showed me what to do with it.  I think it's a damn shame I took so long to find it, so pull up a chair - we're going to set us up some editor.

This first post will go over basic usage and configuration, and next I'll go over the packages I've found most helpful.

I came to Emacs, as so many do, via [Spacemacs](http://spacemacs.org/), which is a distribution of Emacs that comes preconfigured with a bunch of stuff and with its own separate abstraction over sets of packages.  It's great, actually.  It also has good integration with `evil` mode, which enable Vim keybindings, making it a lot more interesting to, well, Vim users.  I had already built some familiarity with Vim and was hesitant about undoing that progress, but far too curious about Emacs not to try it out.  Spacemacs is great, but it's a behemoth - there's a lot there, more than I'll ever need to use.  I realized after a few months that I hadn't really learned anything at all about Emacs - I was still using Vim bindings and had only ever added and removed layers from `.spacemacs` - no actual Emacsery afoot.

So, I started fresh.  I got a blank `~/.emacs.d` and set out to build the editor I wanted from scratch, and this time around I buckled down and went through the Emacs tutorial to get the "real" bindings under my fingers.  I now use a mix of VSCode and Emacs, but I'm glad I took the time to learn how powerful this tool really is, and still use the Emacs keybindings in VSCode instead of Vim now.

...Yes, I wrote this article in Emacs.  Yeah, I heard you.

If you're planning to use Emacs in earnest, you should take the time to go through the tutorial.  Until then, here's a quick overview.

## A Quick Overview

Emacs is manipulated through combinations of commands.  Like Vim, it offers a scheme for controlling your text editor from the keyboard, moving away from the home row as little as possible.  Unlike Vim, which has separate *modes* that you switch in and out of, Emacs uses sequences of key combinations.  No mode switching here, you use modifier keys to indicate an editor action.  For example, we have "Ctrl-x, Ctrl-s" to save the current buffer, which is the current opened bit of text you're working on.  You can remember it by thinking "execute save" - the "Ctrl-x" prefix is used to *execute* a number of commands.  These sequences are so common in Emacs that there's a shorthand - this command would be written `C-x C-s`.  Capital C is Control, and the other most common is capital `M`.  This is most likely your "alt" key.  One great combo to keep in mind is `M-x`, which allows you to execute any Emacs command by name.  Emacs commands are just Emacs Lisp functions, and you can write your own, but there's a ton built in. Our new best friend `C-x C-s` is shorthand for the aptly named `save-buffer`, and if you've completely forgotten the combo, you can always `M-x save-buffer` in a pinch.

If all this sounds like it's going to be a lot, that's because it absolutely is.  How would you know it's called exactly that?  What else is there?  Fret not!  In the next post we're going to install a few very helpful packages that lets us explore these trees of commands visually.  It's all quite nice, I promise, [don't panic](https://proxy.duckduckgo.com/iu/?u=https%3A%2F%2Fcatallassi.files.wordpress.com%2F2014%2F04%2Fthe-hitchhikers-guide-to-the-galaxy-dont-panic-1280x1024-wallpaper_www-wall321-com_50.jpg&f=1)!

Here's a few helpful commands.


### Movement

```
C-f Move [f]orward one character
M-f Move [f]orward one word

C-b Move [b]ackward one character
M-b Move [b]ackward one word

C-n Move to the [n]ext line
C-p Move to the [p]revious line

C-a Move to the [a]ft of the line (okay that's a little forced - the beginning of the line)
C-e Move to the [e]nd of the line

M-< Move to the top of the file
M-> Move to the bottom of the file
```

### Copy/Cut/Paste

```
Move cursor to beginning of region

C-Spc to set marker

Move cursor to the end of the region

C-w to cut the marked region or
M-w to copy the marked region

Move cursor to target

C-y to paste region
```

### File/Window/Buffer

```
C-x C-s save current buffer
C-x C-f Open file
C-x C-c Save and quit emacs
C-x b List open buffers (This will let you select one of them)
C-x 1 Delete all other open windows (This is useful for getting rid of one-off messages that spawn windows)
C-/ Undo - keep going to keep undoing
```

### Advanced:

```
C-M-f Move forward over a balanced expression (words count!  try this one on a bunch of different kinds of files)
C-M-b Move backward over a balanced expression
C-M-k Kill balanced expression forward
C-M-Spc Mark the end of the next s-expression
C-M-n Move forward a parenthetical group
C-M-p Move backward a parenthetical group
```

One honorary mention: `C-k [k]ill line`.  This will kill from the cursor to the end of the line, and also pull the text into the buffer.  You can then paste what was killed with `C-y`.  As an example a common pattern for me for moving the line I'm on is `C-a C-k` to hop to the beginning and kill it, then using `C-n` or `C-p` and `C-y` to drop it somewhere else.  Self test: what does `M-< C-Spc M-> M-w` do?

Each command is fairly mnemonic.  It doesn't take long to get them under your fingers.  I find myself saying the action I intend aloud in my head for a while when I'm learning a new one.  Also, the `M` version are often "more abstracted" versions of your favorite `C` command.  That's often a good thing to try when exploring a new library - many will define combinations with similar characteristics.

Some of those require three keys - `M->` has a shift involved too.  Wacky, right?  It definitely does take some practice, but eventually you never need to leave the home row position.

Also awesome is that these key combinations show up all over the place!  If your system has `bash`, open up a terminal - `C-f`, `C-b`, `C-a`, and `C-e` all work.  Anything that uses `readline` will use a subset of these commands as well.  This might be common knowledge, but I had no idea until I tried Emacs.  Blew my mind a little at least.

This was but a touch of the commands available.  You can make windows (try `C-x 2` or `C-x 3` to split the window horizontally or vertically) and do all kinds of fun stuff (check out [`C-x z`](https://www.gnu.org/software/emacs/manual/html_node/emacs/Repeating.html)) - I cannot hope to do it all justice here so I won't try.  I definitely recommend going through the tutorial and looking at the [`manual`](https://www.gnu.org/software/emacs/manual/html_node/emacs/index.html#Top).  This post is going to focus on the config, and this all should get you up and running.  You can also fall back to the arrow keys and mouse to hop around if you need, but it's worth it to force yourself not to!

Now we're going to start to dive through my personal `init.el`.  Contain your excitement, please, we've only just begun.

## init.el

Emacs is really a lisp interpreter with a solid text editor bundled.  I've always thought the whole "Emacs vs. Vim" debate was a little ridiculous - they're wildly different.  Vim is for when you would use a text editor, Emacs feels much more akin to driving a hyper-customizable IDE.  There's no "Notepad++ vs IntelliJ" flame war going on, why should there be one between Vim and Emacs?

Anyway, the goodness starts in a file called `init.el`.  This is an ELisp file that lives in your `emacs.d` directory and is evaluated on startup.  Mine begins with a number of variables being set.  These are my preferences, season to taste:

```elisp
(setq delete-old-versions -1 ) ; delete excess backups silently
(setq version-control t )
(setq vc-make-backup-files t )
(setq vc-follow-symlinks t )
(setq backup-directory-alist `(("." . "~/.emacs.d/backups")) )
(setq auto-save-file-name-transforms '((".*" "~/.emacs.d/auto-save-list/" t)) )
(setq inhibit-startup-screen t )
(setq ring-bell-function 'ignore ) ; silent bell on mistakes
(setq coding-system-for-read 'utf-8 )
(setq coding-system-for-write 'utf-8)
(setq sentence-end-double-space nil)
(setq-default fill-column 80) ; toggle wrapping text at this column
(setq initial-scratch-message "EEEEEEEEEEEmacs...macs...(macs)... Hi Ben." ) ; You should probably change this
(global-display-line-numbers-mode t )
(menu-bar-mode -1) ; no need for the menu bars - we've got key combos for that!
(toggle-scroll-bar -1)
(tool-bar-mode -1)
```

@yorodm hepfully suggests the following more complete UTF-8 config:

{# {% devcomment 9979 %} %}

Thanks, Yoandy!

Remember before when I said Emacs was a Lisp interpreter?  It's serious business.  You don't need to restart the editor to make changes, or even reload the whole buffer.  You can use `C-x C-e` with your cursor at the end of any of those parenthesized s-expressions to have Emacs evaluate it immediately.  Aww *yeah*.  Try toggling the scroll bar on and off BEFORE YOUR VERY EYES.  You can also use `M-x eval-buffer` to reload the whole thing or just mark a region and use `M-x eval-region` - you do you, you know?

This section is pretty readable.  You use `setq` to set the value of variables.  Anything set to a value of `-1` is like setting it to `false` - I'm disabling the menu bar and toolbar and all the extra stuff that's on by default.  All the functionality therein is also exposed via endless trees of keyboard commands.

Now for the packages!

### use-package

Packages in Emacs are powerful, and with that power does come some complexity.  To tame the beast, I recommend a tool called [`use-package`](https://github.com/jwiegley/use-package).  It's a macro that lets you compartmentalize your package declarations and set per-package configurations in a neat and tidy way.  To set it up with the Emacs package manager, add the following to `init.el`:

```elisp
;; use-package setup
(require 'package)
(setq package-enable-at-startup nil) ; dont do it immediately
(setq package-archives '(("org"       . "http://orgmode.org/elpa/")
			 ("gnu"       . "http://elpa.gnu.org/packages/")
			 ("melpa"     . "https://melpa.org/packages/")))
(package-initialize)

;; Bootstrap use-package
(unless (package-installed-p 'use-package)
  (package-refresh-contents) ; update archives
  (package-install 'use-package)) ; grab the newest use-package

;; Define packages
(require 'use-package)

;; Always download if not available
(setq use-package-always-ensure t)
```

Don't forget to `M-x eval-buffer`!

### Testing it out

To check that it's all working, lets add a package.  A good one to start with is [all-the-icons](https://github.com/domtronn/all-the-icons.el).  This installs a bunch of icons and fonts - no more blank squares anywhere.  Add the following:

```elisp
(use-package all-the-icons)
```

With your cursor at the end of the line, smash that `C-x C-e` and Emacs will install the package.  It works because we have `(setq use-package-always-ensure t)` set.  This particular package has a one-time setup step - go ahead and execute `M-x all-the-icons-install-fonts` now so you never have to worry about it again.

You should be good to go!  This is a very blank slate - head to part 2 to get productive with it!

Oh, by the way... the self test answer from above: `M-< C-Spc M-> M-w` will copy the whole buffer.  I was going to wait for the next post but I just couldn't.  Emacs is *just so exciting*.
