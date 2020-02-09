---
cover_image: https://res.cloudinary.com/practicaldev/image/fetch/s--s6IkvTl3--/c_imagga_scale,f_auto,fl_progressive,h_420,q_auto,w_1000/https://thepracticaldev.s3.amazonaws.com/i/3qs6h3t9dpkyonovre6m.png
edited: 2019-06-22T12:00:00.000Z
title: The Dumb Things I Did And How I'm Going To Fix Them
published: true
description: A retrospective and a refactor plan
tags: devjournal, rust, reason, beginners
---
One of my first posts on this site was about webapp I made to handle taking attendance:

{# {% link deciduously/rust--reasonml---a-beginners-love-story-45a2 %} #}

When I wrote that post, it had already been a while since I'd completed writing the application.  It works, too, the center has been using this daily for almost a year now, with minimal intervention on my part since I left that job.  I'm about to sit down and polish it up after not touching it for a long time, and have decided by "polish up" I mean rewrite most of the backend and much of the frontend entirely, again.  Record scratch, freeze frame, I bet you're wondering how I got here.

The reason this app is written...well,  silly, is not, in retrospect, because it was the best of my ability at the time and I've since learned.  No, this was a desperation application, and I'm writing this post mostly for my own benefit.  If we don't remember the mistakes of the future, we are doomed to repeat them for the first time or something.

I *fully well* knew better for each one of the problems I outline here, and opted to do the lazy, quick thing in the interest of producing something that worked.  I'm a little bit peeved at Past Ben.  Let this post be a warning, Future Ben.

The application lives [here](https://github.com/deciduously/mifkad).

## The Background

I worked in the administrative office of a non-profit preschool.  It was little bit hectic.  It was adorable, yes, but unpredictable at times, what with the children, the teachers, the parents, you know.  The elements of a preschool and whatnot.  Administrative would be putting my duties lightly, but "miscellaneous" isn't really a great job posting title.  I was all over the facility for various reasons, the phone calls were frequent, an uninterrupted fifteen minutes at any given task was a rarity.  Thus, time was at a premium and every scrap of organization was to be cherished.

Having already been hobby-gramming for a while by then, I had a dislike for doing tasks that I thought were "for computers", even though sometimes for necessity a pencil and paper is the simplest and cheapest way to do something.  In one case, though, it *clearly* wasn't.

## The Problem

One of my daily duties was collection the school's attendance and logging it in a spreadsheet.  Then, after logging any absences, I'd calculate the "Extended Day Roster" and email it out to everyone so that afternoon staffing could be solidified.

At 4 PM, most kids go home.  That's what's included in the base contract.  Parents had the option to keep their kids there until 6 PM instead, at a different hourly rate.  As only a fraction of families choose this, the fifteen classrooms during the day reduce down to just five.  In order to ensure we maintained appropriate staff-child ratios, the directors (and teachers) needed to know how many kids to expect in this extended part of the day, including any absences.  The more staff you can safely cut, the better!

This was all done in a small spiral notebook.  The classrooms were spread around the building, so instead of making the loop and being far from the desk if someone needed me I'd call each and ask the teacher over the phone if anyone was absent.  I'd write down the absences for each classroom.  Then, after I'd managed to connect with all fifteen (easier said than done), I'd reconcile it against the children who are contracted to stay late and those who have signed up on an ad-hoc basis to stay extra for that specific date, adding and subtracting from the expected headcount.  Then I'd type up the result in a formatted email, including the names of any additions and removals.

This took a *significant* chunk of my day.  Some days, because of poor telephone timing and various interruptions, it could be a full two hours between the first phone call and the final email.

## The Solution

The application I built presents the user with a series of buttons, one per child, organized by class.  Clicking the child's name will toggle them Present or Absent, and each child that isn't already scheduled to be in Extended Day also gets a button to add them ad-hoc.  The actual Extended Day headcount email is just a function of who's in and who's extra, so it keeps a version ready to copy/paste updated at the top of the page, or download as a text file:

![app screenshot](https://camo.githubusercontent.com/44596acec59b2793fc5f773271a6cc355249ad68/68747470733a2f2f692e696d6775722e636f6d2f7a6777706e6b512e706e67)

The ad-hoc sign-up forms are printed on pink copy paper, thus "pink sheet".  The user can specify how the core rooms funnel into the extended rooms with a simple UI:

![room picker](https://camo.githubusercontent.com/ad48c887994c5276c32dd5962bdd7f9ac2b0865e/68747470733a2f2f692e696d6775722e636f6d2f494d5133356d632e706e67)

That "Download" button just encodes the text as base64 and embeds it right in the link:

```ocaml

let make = (~school, ~refreshClicked, ~resetClicked, _children) => {
  ...component,
  render: _self => {
    let dload =
      "data:application/octet-stream;charset=utf8;base64,"
      ++ btoa(Report.school(school));
    <div>
      // ..
      <a href=dload> <button> {ReasonReact.string("Download")} </button> </a>
      // ..
    </div>;
  },
};
```

## The Mess

Carrying out this task every day really ground my gears.  I *hated* doing it, so I was eager to push out a solution that could cut down the amount of time spent.  Thus, I cut every corner I could in order to get to working.

### The Scraping, Oh The Scraping

The first problem was populating the application with the rosters for the day.  Unfortunately, despite my lobbying, I am unable to query the organization's database directly, and must use pre-created Crystal Reports to pull any data out.  Now, these reports are not exactly designed for data scrobbling, and I have zero control over what they're specifically pulling out.  They're for human consumption, formatted all nicely and designed to be printed as-is or exported to a PDF.  Crystal Reports does provide the option to export to Excel, though, which is as good as it's gonna get.  The sheet you end up with is funky, there's weird rows and weird data in rows as artifacts from the nicely formatted PDF the report was intending to create.  The report designed to be printed and given to the classrooms as their daily attendance sheet does contain all the info I need to populate this app, though.

Getting a nice clean Rust data structure out of this spreadsheet is not necessarily complicated, but I was rushing:

```rust
pub fn scrape_enrollment(
    day: Weekday,
    extended_config: ExtendedDayConfig,
    config: &Config,
) -> Result<School> {
    lazy_static! {
        // Define patterns to match
        static ref KID_RE: Regex =
            Regex::new(r"((@|#|&) )?(?P<last>[A-Z]+), (?P<first>[A-Z]+)").unwrap();
        static ref CLASS_RE: Regex = Regex::new(r"CLASSROOM: ([A-Z])").unwrap();
        static ref CAPACITY_RE: Regex = Regex::new(r"CLASS MAXIMUM: (\d+)").unwrap();
    }

    info!("Loading {:?} from {:?}", day, &config.roster);
    let mut school = School::new(day, extended_config);

    // Use calamine to read in the input sheet
    let mut excel: Xls<_> = open_workbook(&config.roster).unwrap();

    let mut headcount = 0;
    let mut classcount = 0;

    // Try to get "Sheet1" as `r` - it should always exist
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        // Process each row
        for row in r.rows() {
            use calamine::DataType::*;
            // Column A is either a Class or a Kid
            let column_a = &row[0];
            match column_a {
                String(s) => {
                    // If it's a class, open up a new class
                    // If its a kid, push it to the open class
                    // If it's anything else, ignore it.
                    if CLASS_RE.is_match(&s) {
                        debug!("MATCH CLASS: {}", &s);
                        let caps = CLASS_RE.captures(&s).unwrap();
                        // the capacity is found in Column B
                        let capacity: u8;
                        match &row[1] {
                            String(s2) => {
                                let capacity_caps = CAPACITY_RE.captures(&s2).unwrap();
                                capacity = (&capacity_caps[1])
                                    .parse::<u8>()
                                    .chain_err(|| "Unable to parse capacity as u8")?;
                            }
                            _ => {
                                bail!("Column B of Classroom declaration contained unexpected data")
                            }
                        }

                        // Display the previous class headcount  -this needs to happen once againa the end, and not the first time
                        if !school.classrooms.is_empty() {
                            let last_class = school.classrooms[school.classrooms.len() - 1].clone();
                            let prev_headcount = last_class.kids.len();
                            debug!("Room {} headcount: {}", last_class.letter, prev_headcount);
                        }

                        // create a new Classroom and push it to the school
                        let new_class = Classroom::new(classcount, caps[1].to_string(), capacity);
                        debug!(
                            "FOUND CLASS: {} (max {})",
                            &new_class.letter, &new_class.capacity
                        );
                        school.classrooms.push(new_class);
                        classcount += 1;
                    } else if KID_RE.is_match(&s) {
                        let caps = KID_RE.captures(&s).unwrap();

                        // Reformat name from LAST, FIRST to FIRST LAST
                        let mut name = ::std::string::String::from(&caps["first"]);
                        name.push_str(" ");
                        name.push_str(&caps["last"]);

                        // init Kid datatype

                        // Add schedule day
                        let sched_idx = match day {
                            schema::Weekday::Monday => 6,
                            schema::Weekday::Tuesday => 7,
                            schema::Weekday::Wednesday => 8,
                            schema::Weekday::Thursday => 9,
                            schema::Weekday::Friday => 10,
                        };
                        let sched = &row[sched_idx];
                        let new_kid = Kid::new(headcount, name, &format!("{}", sched));
                        debug!(
                            "FOUND KID: {} - {} ({:?})",
                            new_kid.name, sched, new_kid.schedule.expected
                        );
                        // If the kid is scheduled, push the kid to the latest open class
                        if new_kid.schedule.expected == Expected::Unscheduled {
                            debug!(
                                "{} not scheduled on {:?} - omitting from roster",
                                &new_kid.name, day
                            );
                        } else {
                            let mut classroom = school.classrooms.pop().expect(
                                "Kid found before classroom declaration - input file malformed",
                            );
                            classroom.push_kid(new_kid);
                            school.classrooms.push(classroom);
                            headcount += 1;
                            debug!("Adding to response");
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    // Print out the status info
    let last_class = school.classrooms[school.classrooms.len() - 1].clone();
    info!(
        "Room {} headcount: {}",
        last_class.letter,
        last_class.kids.len(),
    );
    warn!(
        "Successfully loaded {:?} enrollment from {:?} - total headcount {}, total classcount {}",
        day, config.roster, headcount, classcount
    );

    Ok(school)
}
```

Looking at it just *makes me upset*.  This abomination was knocked together in an afternoon to get it over with, and is not exactly maintainable.  Should the format change on me, as it's liable to do at any time without warning, this is *not* a fun refactor.  Each section of this can be broken out into helper functions that can be edited in isolation.

### The Launcher

This application is designed as your standard web shindig.  There's a backend application that serves up a webpage and provides an HTTP API for futzing with the app state.  Users navigate to this page via their web browser, and use UI elements to interact with these API endpoints.

I never quite got around to centralizing it, though, partially because of an overworked IT department that had little time for my hobby experiment.  They're busy doing "real work" or whatever.  Thus, in order to use this program, the user must launch the webserver themselves first.  It then serves on `localhost`.  To make this easier to swallow, I put together a minuscule batch file and called it a "launcher":

```batchfile
:: Suppress command output
ECHO OFF
:: Launch server
start mifkad.exe
:: Launch client
start chrome http://127.0.0.1:8080
```

So, yeah.  It works, but, talk about janky.  This also means that everyone who uses it is running a local, isolated instance of the app.  The persistent data store that gets manipulated by this instance is indeed shared, but each app touches it independently.  Even sillier, the app is actually well-architected to avoid write collisions - as long as you have multiple connections coming to the *same* instance!  Running it like this completely bypasses that built-in safety guarantee.  Double oof.  Here's the code to actually adjust the app state:

```rust
pub fn adjust_school(
    (path, state): (Path<(String, u32)>, State<AppState>),
) -> Box<Future<Item = Json<School>, Error = actix_web::Error>> {
    use self::Action::*;
    let action = Action::from_str(&path.0).unwrap();
    let id = path.1;

    {
        // Grab a blocking write lock inside inner scope
        let mut a = state.school.write().unwrap();

        // Perform the mutation
        match action {
            Toggle => (*a).toggle_kid(id),
            AddExt => (*a).addext_kid(id),
            Collect => (*a).collect_room(id),
            Reset => {
                reset_db(&state.config).unwrap();
                (*a) = init_db(&state.config).unwrap();
            }
        }
        // blocking lock is dropped here
    }

    // grab a new non-blocking reader
    let a = state.school.read().unwrap();

    // Sync the on-disk DB and return the json
    write_db(&*a).unwrap();
    result(Ok(Json((*a).clone()))).responder()
}
```

All that care taken to handle concurrent writes safely, and this application is only ever used by a single connection to a local instance.  Nice going, Ben.

### The JSON

The other big dumb shortcut is the data persistence.  Because multiple users will interact with the same data from different workstations (running different instances of the application), I decided to write each change to a shared network drive.  When the app launches, it checks to see if there's an already-created app state first, and populates from that instead of re-reading the roster spreadsheet.  This should likely have been implemented using a relational database off the bat, but no.  I just serialize the app state to JSON and store it as a datestamped text file, e.g. "20190621.json".  The app just looks up the date to see if there's a corresponding file.

It sorta works, but these files are not that helpful.  The folder where they get created is just incrementally growing for no reason, and after the day is over these are generally useless.  I could clean them up, but they're also the only record anywhere of which kids stayed for extended day, outside of their AR line.  It's helpful to have that extra backup, but digging through it means sifting through a pile of JSON with no whitespace.  That's not fun, and pretty much not happening for folks down there that aren't me.

It's also brittle - if the file for the day disappears or gets altered, its unreadable and you have to start over.  A more highly managed data interface would prevent problems like that.

## The Revival

I'm coming back to this project now because I wanted to try to generalize it for use in the other locations within this network.  I think I probably could slap that generalization on the current incarnation, but digging through the code again has made me feel like I need a shower, and I can't just leave it like this.

Also, since I wrote this, both the niche and new backend framework and niche and new frontend framework made significant updates.  Normally I'd not be too concerned about this, but the backend framework, `actix_web` stabilized to 1.0 over 0.7.  That's pretty nice, considering I'd like to eventually (mostly) walk away from this and have it work for them indefinitely.  ReasonReact also stabilized a much neater component syntax which I feel makes the code more readable - perfect for a project I expect to touch once every several months at most.

The frontend I don't expect to be a complicated migration.  I'm more or less pleased with it, actually, though I think there is some opportunity for cleanup.  For instance, the whole crux of this applications usefulness is the ability to look at the 15 core classrooms and pull out the 5 reduced classrooms.  This is handled via a rather opaque fold in Reason:

```ocaml
let add_extended_room = (school, classroom) => {
  /* This is our folding fn from get_extended_rooms below.
     It should take a room and a school and either add the new room
     or if a room already exists with the same letter, just add those kids */
  let target = ref(school.classrooms);

  if (Array.length(target^) == 0) {
    target := Array.append(target^, Array.make(1, classroom));
  } else {
    let already_included =
      Array.map((oldr: classroom) => oldr.letter, school.classrooms);
    let found = ref(false);
    let idx = ref(0) /* This will only be read later if found is toggled to true*/;
    Array.iteri(
      (i, l) =>
        if (classroom.letter == l) {
          found := true;
          idx := i;
        },
      already_included,
    );
    if (found^) {
      /* We've already seen this letter - mash the new kid list into the matching existing kid list */
      let old_classroom = school.classrooms[idx^];
      let new_classroom = {
        ...old_classroom,
        capacity:
          get_extended_capacity(classroom.letter, school.extended_day_config)
          |> int_of_string,
        kids: ref(Array.append(old_classroom.kids^, classroom.kids^)),
      };
      target^[idx^] = new_classroom;
    } else {
      /* This is a new extended day room - add it as-is, grabbing the extended day capacity */
      target :=
        Array.append(
          target^,
          Array.make(
            1,
            {
              ...classroom,
              capacity:
                get_extended_capacity(
                  classroom.letter,
                  school.extended_day_config,
                )
                |> int_of_string,
            },
          ),
        );
    };
  };

  {...school, classrooms: target^};
};

let get_extended_rooms = school => {
  /* Returns a `school` of the extended kids */
  let s = get_extended_kids(school);
  Array.fold_left(
    add_extended_room,
    {...school, classrooms: [||]},
    s.classrooms,
  );
};
```

Honestly, I'm impressed it works as reliably as it does.  This just screams "I only think I know how functional programming works".  I don't really remember how I arrived at this particular solution, and I'm a little scared to touch it.  That's no good.

The backend is the bigger priority though.  The new stable version of `actix_web` is a significant breaking change from what I'm using now, so I will likely be rewriting at least my handlers from scratch anyway.  While I'm at it, I'm going to rip out the JSON serialization apparatus in favor of an SQLite store.  This will still be portable, as it can live inside a file on the OS in a very self-contained fashion, but much harder to accidentally corrupt.  It will also have the added benefit of being query-able - if you have to know when Albert Gore signed up for extra hours between January 2 and February 12, you can just ask that using SQL.  After making sure the base app works as intended I'd like to add a UI exposing some of this functionality.

Another new feature that can be built off a more solid base would be allowing classrooms to do their own entry.  If this app is used as intended, i.e. it exists as an always-running process that users connect to remotely instead of launching locally, there's no reason why the teachers can't just take attendance directly on it.  That would get rid of the need for them to collect it separately and then play phone tag for a while about it.  If they could navigate from their in-classroom iPad to a webpage just for their students just to mark who's in or out, the attendance for the site would basically take itself.  This whole concept requires that I move to a centralized app model, though.

I'd also like to flesh out the testing story.  There are some tests present for various parts of the backend, but I would not call this a well-tested application.  The more I can do to automate ongoing maintenance of this product, the better.

Finally, it's pretty clear that the UI is built by someone with a heavy phobia of any CSS beyond the bare minimum.  It doesn't *need* to be so gorram ugly.  I should probably take the opportunity to practice that skillset.

It sounds like I've got my work cut out for me still on this application, after all this time.  Little did I know when I first got the idea to give it a whirl.  Guess I'd better get going!
