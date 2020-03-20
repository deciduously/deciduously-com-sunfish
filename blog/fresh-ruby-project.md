---
edited: 2020-03-19
title: Setting Up A Fresh Ruby Project
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--de_-hdwU--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://dev-to-uploads.s3.amazonaws.com/i/ff04d0gluu888p0wg33i.jpg
published: true
tags: beginners, ruby, devops, tutorial
---
To start up a Ruby project, you don't need to do anything other than put some Ruby code in a file and invoke it with `ruby program.rb`.

As projects grow, though, you'll likely add dependencies and tests.  You can mitigate this with documentation - tell users what gems they need to pull, and which files contain the test suites - but that becomes unwieldy quick.  It's much better to lean on ecosystem tools to specify and automate your build steps.

To follow along, you'll need [Ruby](https://www.ruby-lang.org/en/) installed.  The latest at time of writing is version 2.7.0.

EDIT: If you don't want to do this from scratch, try `bundler gem`.

## Make a directory

Don't just spread your ruby source all across your home directory - keep your projects self contained.  Create a directory:

```
$ mkdir ruby_project
```

Open this directory in your favorite editor.  Everything else in this post goes in here.

## Add Bundler

To manage dependencies, use [Bundler](https://bundler.io/).  This tool allows you to specify gems required to run your project and lets users issue a single `bundle install` command to pull down any required.

First, install the gem in you haven't already:

```ruby
$ gem install bundler
```

Then, create a file called `Gemfile` in your project root.  For now, we'll just specify the package repository URL:

```ruby
source 'https://rubygems.org'
```

As we need dependencies, we'll add them here below this line.

## Add a Test file

The first dependency we need is for testing.  This example uses [minitest](https://github.com/seattlerb/minitest).  Add that to your `Gemfile`:

```diff
  source 'https://rubygems.org'
+ gem 'minitest'
```

Now you can invoke `bundle install` to pull this down automatically.  When you do, a new file called `Gemfile.lock` will be generated containing the specific package version in use.

This is really up to you, but I like keeping my tests in a separate subdirectory:

```
$ mkdir test
```

Create a file in this folder called `test/cool_program_test.rb` with the following contents:

```ruby
# frozen_string_literal: true

require 'minitest/autorun'
require_relative '../lib/cool_program'

# Test program coolness
class CoolProgramTest < Minitest::Test
  def test_coolness_off_the_charts
    # skip
    assert_equal CoolProgram.new.coolness, 11
  end
end
```

The first line, `# frozen_string_literal: true`, is a magic comment that is kind of like calling `Object.freeze` on every string literal.  This is done for performance reasons, but you can opt out for a given string by prefixing it with a `+` character: `+'my newly mutable string literal'`.  Now you can do `my_str << some_other_str`.  In Ruby, though, I'm not finding myself manipulating strings like this.

We know we'll be able to use the `minitest` dependency in the first line because we specified it with Bundler.  The actual source code doesn't exist yet, though!

## Add an Implementation File

This is also personal preference, but it's a sensible idea to keep your implementations in a separate subdirectory as well:

```
$ mkdir lib
```

Create the file we pulled into the test file at `lib/cool_program.rb` with these contents:

```ruby
# frozen_string_literal: true

# The coolest program
class CoolProgram
  attr_reader :coolness
  def initialize
    @coolness = 11
  end
end

puts "Coolness: #{CoolProgram.new.coolness}/10"
```

Groovy!  Everything's in place.

## Add a Rakefile

Now, we could just direct users to invoke `ruby` themselves on the proper files - `ruby lib/cool_program.rb` to run the program, or `ruby test/cool_program_test.rb` for the tests.  It would be better if we could specify those paths and abstract into `test` or `run` operations.  That's what [`rake`](https://github.com/ruby/rake) is for!  This tool is a lot like GNU Make, but for Ruby.

First, add it to your `Gemfile`:

```diff
  source 'https://rubygems.org'
  gem 'minitest'
+ gem 'rake'
```

Then, add a new file in your project root called `Rakefile`:

```ruby
task default: %w[test]

task :run do
  ruby 'lib/cool_program.rb'
end

task :test do
  ruby 'test/cool_program_test.rb'
end
```

If you know how `make` works, this is similar (if you don't, I've [got ya covered](https://dev.to/deciduously/how-to-make-a-makefile-1dep)).  The first line defines the default task, or what happens when you invoke `rake` without specifying a specific task.  In this case, we just have it run the `test` task as a dependency with no block itself.  This is a list, you can specify multiple tasks here.

Each task below can be invoked at the command line directly - you'd use `rake run` to actually execute your program and just `rake` on its own to run the test suite.

## Add a Linter

I've become fairly reliant on the [Rubocop](https://github.com/rubocop-hq/rubocop) linter to keep me in line with the [Ruby style guide](https://rubystyle.guide/).  It now ships with a ready-to-go Rake task.

First, add it to your `Gemfile`:

```diff
  source 'https://rubygems.org'
  gem 'minitest'
  gem 'rake'
+ gem 'rubocop'
```

Then, modify your `Rakefile`:

```diff
+ require 'rubocop/rake_task'

- task default: %w[test]
+ task default: %w[lint test]

+ RuboCop::RakeTask.new(:lint) do |task|
+   task.patterns = ['lib/**/*.rb', 'test/**/*.rb']
+   task.fail_on_error = false
+ end

  task :run do
    ruby 'lib/cool_program.rb'
  end

  task :test do
    ruby 'test/cool_program_test.rb'
  end
```

It's my personal preference to run the linter and the tester by default, but not fail on lint errors.  [Season to taste](https://docs.rubocop.org/en/stable/integration_with_other_tools/#rake-integration).

## Optional - Add a GitHub Action

I host almost all my code on GitHub and have become a fan of GitHub Actions.  Using YAML, you can have GitHub automatically run your tests whenever you commit or open a PR.  Create a new file at `<project root>/.github/workflows/ruby.yml`:

```yml
name: Ruby

on:
  push:
    branches: [ master ]
  pull_request:
    branches: [ master ]

jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v2
    - name: Set up Ruby 2.7
      uses: actions/setup-ruby@v1
      with:
        ruby-version: 2.7.x
    - name: Build and test with Rake
      run: |
        gem install bundler
        bundle install --jobs 4 --retry 3
        bundle exec rake
```

You can tweak each step very easily here - the commands specified will get run and if everything completes without errors, you get a green check mark.  Sweet!  Using `rake` allows this file to stay concise, and then you'd be able to manage changes to your structure in your Rakefile without having to worry about this file at all.

## Add a README and stuff

We're just about done, but you should always include a README:

```markdown
# ruby_project

The coolest dang program that you ever did see.

## Dependencies

* [Ruby](https://www.ruby-lang.org/en/).  Written with version [2.7.0](https://www.ruby-lang.org/en/news/2019/12/25/ruby-2-7-0-released/) - *[docs](https://docs.ruby-lang.org/en/2.7.0/)*.

## Usage

Install deps: `gem install bundler && bundle install`.  Run `bundle exec rake` to run the tests, or `bundle exec rake run` to run the program.
```

Add a `.gitignore`:

```gitignore
*.gem
*.rbc
/.config
/coverage/
/InstalledFiles
/pkg/
/spec/reports/
/spec/examples.txt
/test/tmp/
/test/version_tmp/
/tmp/

## Environment normalization
/.bundle/
/vendor/bundle
/lib/bundler/man/

# Used by RuboCop. Remote config files pulled in from inherit_from directive.
.rubocop-https?--*
```

You should probably add a LICENSE file, too - here's the [BSD-3-Clause](https://opensource.org/licenses/BSD-3-Clause):

```txt
Copyright 2020 Coolness McAwesome

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

At the end, your directory should look like this:

```txt
$ tree
.
|____Gemfile.lock
|____.gitignore
|____.github
| |____workflows
| | |____ruby.yml
|____Gemfile
|____LICENSE
|____Rakefile
|____test
| |____cool_program_test.rb
|____README.md
|____lib
| |____cool_program.rb
```

Finally, `git init && git add . && git commit -m "Initial commit"`.  Happy hacking!

If you don't feel a pressing need to do this from scratch, you can just use [this GitHub template](https://github.com/deciduously/ruby-template) with the code from this post.

*Photo by Jukan Tateisi on Unsplash*
