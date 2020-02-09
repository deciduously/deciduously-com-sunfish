---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--aYHDIAyT--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/08t3ydtxov6ym0aghmin.jpg
edited: 2019-03-08T12:00:00.000Z
title: How I Emacs And So Can You: Packages
published: true
description: The packages in my init.el
tags: beginners, emacs, tutorial
---
In the first post we looked at some basic usage and navigation, and set up `use-package` so we can easily add community packages to our Emacs installation.

## Breathing Room

I think I go a little overboard with this, but every time one of my `use-package` declarations goes over a single line, I like to pull it out to its own file.  That way I just have one line to comment/uncomment in `init.el` to activate/deactivate a package.  To set this up, create a directory inside `.emacs.d` - I just called mine `.emacs.d/lisp`.  We can ensure it gets evaluated by adding the following to `init.el`:

```elisp
;; Pull in ./lisp/*
(add-to-list 'load-path (expand-file-name "lisp" user-emacs-directory))
```

Now any `whatever.el` elisp file we put in this directory will be visible to `init.el`.

The Emacs ecosystem is big, and there are multiple solutions and sets of solutions for any given problem.  I like to keep mine pretty minimal, this is just the set that works for me - I do urge you to explore!  The packages used in this set, notably, are not the same set that Spacemacs is based around.  When you do your own research, it sorta-kinda comes down to [`helm`](https://github.com/emacs-helm/helm) & friends vs. [`ivy/swiper/counsel`](https://github.com/abo-abo/swiper) - this is the `ivy` route.  I intentionally wanted to try something different from what I had gotten to know via Spacemacs, but it shouldn't be taken as a value judgement at all.  I've enjoyed using both greatly.

These are completion engines.  Remember the last post, when we forgot `C-x C-s` but then still miraculously knew it was `save-buffer`?  With `ivy`, you'd be able to just hit `M-x` and then frantically start typing `save` and `ivy` will find everything it possibly could be.  It will even helpfully show you the assigned key combination for a given command if there is one.  Pretty damn handy with a tool as vast as Emacs!  It's a personal always-on concierge.

### Ivy/Counsel/Swiper

That's as good a place to start as any.  Ivy is the main event here, and `counsel` and `swiper` are `ivy`-imbued versions of common commands and file search, respectively.  Create a file called `init-ivy.el`:

```elisp
;;; #init-ivy.el
;;; Commentary:
;;; http://oremacs.com/swiper/#installation
;;; https://sam217pa.github.io/2016/08/30/how-to-make-your-own-spacemacs/#fnref:3
;;; https://writequit.org/denver-emacs/presentations/2017-04-11-ivy.html#fnr.2
;;; Code:
(use-package ivy
  :diminish (ivy-mode . "")
  :init (ivy-mode 1) ; globally at startup
  :config
  (setq ivy-use-virtual-buffers t)
  (setq ivy-height 20)
  (setq ivy-count-format "%d/%d "))
(provide 'init-ivy)
;;; init-ivy.el ends here.
```

In this same file, I also set up `counsel`.  This package overrides some built-in Emacs commands with more user friendly versions.  Add this above the final comment:

```elisp
;; Override the basic Emacs commands
(use-package counsel
  :bind* ; load when pressed
  (("M-x"     . counsel-M-x)
   ("C-s"     . swiper)
   ("C-x C-f" . counsel-find-file)
   ("C-x C-r" . counsel-recentf)  ; search for recently edited
   ("C-c g"   . counsel-git)      ; search for files in git repo
   ("C-c j"   . counsel-git-grep) ; search for regexp in git repo
   ("C-c /"   . counsel-ag)       ; Use ag for regexp
   ("C-x l"   . counsel-locate)
   ("C-x C-f" . counsel-find-file)
   ("<f1> f"  . counsel-describe-function)
   ("<f1> v"  . counsel-describe-variable)
   ("<f1> l"  . counsel-find-library)
   ("<f2> i"  . counsel-info-lookup-symbol)
   ("<f2> u"  . counsel-unicode-char)
   ("C-c C-r" . ivy-resume)))     ; Resume last Ivy-based completion
```

Don't worry too too much about memorizing everything here right off the bat - it will be here when you need it.  For a while I had an index card with a few of the most handy ones sitting on my desk.  In the last post we covered the "save" action, which was a whole keypress more than you're probably used to - this is because `C-s` is reserved for searching for text in the given file.  Check out the [video demo](https://www.youtube.com/watch?v=VvnJQpTFVDc).

### Interlude: Wait, There Totally Are Modes

Well, yes, but they're not Vim modes!  In Emacs, a `mode` determines how Emacs semantically understands the text in the current buffer.  These fall into two categories, `major` and `minor` - each buffer has one major mode, and can have multiple minor modes.  A major mode might be something like `clojure-mode` - this text is only Clojure code, not some other type of code as well, but could have `ivy-mode` and `spellcheck-mode` enabled as well, because that functionality can stack.

Alright, now that `init-ivy.el` has been added to `lisp/`, we can add it to `init.el`:

```elisp
(require 'init-ivy)
```

That's it!  Evaluating that `require` expression with `C-x C-e` will read our new file and set up Ivy for us.

### Flycheck

Another package I love is [flycheck](https://www.flycheck.org/en/latest/), which provides on the fly syntax checking.  It has indicators for problematic lines, squiggly underlines, and pop-up tooltips - all the trappings of a modern syntax checker.  This declaration is simpler:

```elisp
;;; lisp/init-flycheck.el
(use-package flycheck
  :init (global-flycheck-mode))
(provide 'init-flycheck)
```

And in `init.el`:

```elisp
(require 'init-flycheck)
```

Some languages will require special setup, but most things will just work out of the box.

### Company

A perfect complement to `flycheck-mode` is [`company-mode`](https://company-mode.github.io/), which provides text-completion.  As you type, it will make suggestions.  You can scroll through them with `M-n` and `M-p`, and use the enter key to select.  There are more ways to interact with it as well - peep the docs for deets.

In `lisp/init-company.el`:

```elisp
(use-package company
  :config
  (add-hook 'after-init-hook 'global-company-mode))
(provide 'init-company)
```

And of course `(require 'init-company)` in `init.el`.  Now we're starting to feel like a real IDE!

### which-key

This is probably my favorite of the bunch.  Ivy is giving us some nice completions, but you still need to know where to start - it's not great for discovering what's available.  [Which-key](https://github.com/justbur/emacs-which-key) will pop up a window when you begin a command listing everything available.  In our `save-buffer` example, when you type the first `C-x`, you'll get a big pane detailing every combination available after `C-x`, with the combo and the command name.  This is how I find new combos to learn, and it's great for jogging your memory.

My `init-which-key.el`:

```elisp
(use-package which-key
  :init
  (which-key-mode)
  :config
  (which-key-setup-side-window-right-bottom)
  (setq which-key-sort-order 'which-key-key-order-alpha
	which-key-side-window-max-width 0.33
	which-key-idle-delay 0.05)
  :diminish which-key-mode)

(provide 'init-which-key)
```

Tweak these to your liking, these settings work for me.  Of course, don't forget `(require 'init-which-key)` in `init.el`!

### Smartparens

This minor mode helps manage your parentheses.  It has a number of [facilities](https://github.com/Fuco1/smartparens) for manipulating parenthetical expressions - a huge help no matter what programming language you use.

`lisp/init-smartparens.el`:

```elisp
(use-package smartparens
  :config
  (require 'smartparens-config)
  (add-hook 'lisp-mode-hook #'smartparens-strict-mode))
(provide 'init-smartparens)
```

I've added a hook that activates an even stricter version when I'm in a specific minor mode - this is also something you'll need to tweak for yourself!  I actually also use `smartparents-strict-mode` in `rust-mode` - we'll get to the langauge-specific stuff later.

By now you know the drill for getting it into `init.el`!

### Neotree

This is my last general package.  Neotree is a habit I picked up from Vim - it shows a graphical overview of the directory tree that you can use to switch between files.  Another nicety that IDEs feel like they should have - though for the most part I find myself invoking `C-x C-f` or `C-x b` to navigate around in a project.

`lisp/init-neotree.el`:

```elisp
(use-package neotree
  :init
  (require 'neotree)
  :config
  (setq neo-theme (if (display-graphic-p) 'icons 'arrow))
  (setq neo-smart-open t)
  )
(provide 'init-neotree)
```

I lied - that was the second to last.  I also use [`find-file-in-project`](https://github.com/technomancy/find-file-in-project).

```elisp
(use-package find-file-in-project)
```
## Keybindings

The next order of business is setting up your own keybindings.  We can use `global-set-key` for this.  The first one I set is the key to activate `neotree` - add this to your `init.el`:

```elisp
(global-set-key [f8] 'neotree-project-dir)
```

To enable this behavior, I have the following snippet stolen from the emacs wiki placed in `lisp/bl-fns.el` to facilitate NeoTree attempting to use the git project root when it opens:

```elisp
(defun neotree-project-dir ()
  "Open NeoTree using the git root."
  (interactive)
  (let ((project-dir (ffip-project-root))
	(file-name (buffer-file-name)))
    (if project-dir
	(progn
	  (neotree-dir project-dir)
	  (neotree-find file-name))
      (message "Could not find git project root."))))

(provide 'bl-fns)
```

Pretty easy, right?  Now the F8 key will toggle the NeoTree window.  Cool.  Another keybinding I add for myself that I find useful is this:

```elisp
(global-set-key (kbd "C-c q") (lambda ()
	       		       (interactive)
   			       (other-window -1)))
```

The `kbd` macro lets you define combos using the handy shorthand.  This combo, `C-c q`, will switch back to the previous active window.  I generally only have two or three open and find myself using this one a lot.

I also like this shorthand for `company-complete`:

```elisp
(global-set-key (kbd "C-c h") 'company-complete)
```

## Language-specific packages

### Clojure

For clojure, I use [CIDER](https://github.com/clojure-emacs/cider):

```elisp
;; init-clojure.el
(use-package clojure-mode)
(use-package cider)
(provide 'init-clojure)
```

CIDER is a whole can of worms in and of itself - I'll come back to that in a separate post sometime!

### Rust

Rust has a little more going on to set it up with flycheck and cargo and everything:

```elisp
;; init-rust.el
(use-package rust-mode)
(use-package flymake-rust)
(use-package racer)
(use-package company)
(use-package cargo
  :config
  (add-hook 'rust-mode-hook 'cargo-minor-mode))
(use-package flycheck-rust)
(with-eval-after-load 'rust-mode
  (add-hook 'flycheck-mode-hook #'flycheck-rust-setup))
(provide 'init-rust)
```

To be completely honest, Rust was my biggest driver in migrating toward VSCode - Rust in Emacs is fantastic, Rust in VSCode is unparalleled.  The above works great, but I just can't in good faith recommend this setup *over* using the RLS from VSCode.

### Some More

Forth, JavaScript/HTML/CSS, and Reason/OCaml I use with zero config:

```elisp
(use-package forth-mode)
(use-package js2-mode)
(use-package reason-mode)
(use-package web-mode)
(use-package ocp-indent)
```

And....that's all I got for ya!  This set of packages provides a complete multi-language IDE without much bloat.

To update your installed packages, run `M-x list-packages` - this will refresh the latest package list.  Then just type `U` (shift-u) to upgrade any that are outdated.
