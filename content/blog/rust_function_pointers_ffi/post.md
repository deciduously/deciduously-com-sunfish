---
date: 2021-09-10T12:00:00.000Z
title: Rust Function Pointers Across FFI Boundary 
tags:
  - help
  - rust
  - c
  - ffi
---
Hi DEV - it's been ages!  I'm stumped on a problem, and remembered my old adage: "ask DEV stuff, they know things".  So, here's hoping someone can unstick me!

EDIT:  Figured it out.  Left the solution in the comments!

I'm trying to store a pointer to a function defined in C in a Rust struct, and call it from the Rust side. The rest of my FFI seems to be hooked up okay, but I'm getting a segfault when I make the actual function call. Debugging sessions have not proved useful.

I'll try to keep the context streamlined, but let me know if you need more to see what's going on.

First, here's the C externs, the function I'm pointing to, and the passage across the FFI boundary:

```c
typedef struct scheduler scheduler_t;
typedef struct job job_t;
typedef void (*unit_to_unit_t)(void);

extern void run(job_t *, scheduler_t *, unit_to_unit_t);
extern job_t *every(uint32_t);
extern job_t *seconds(job_t *);

// Define a job
void job(void)
{
    //printf("Hello!  It is now %s\n", now());
    printf("Hello!");
}

run(seconds(every(8)), scheduler, job);
```

Super exciting stuff. The commented out line calls a `now()`function that returns a `char *`, but I wanted to get anything working first. The `every()` function returns `job_t *`, and `seconds()` takes a `job_t *` and returns a `job_t *`, which is finally passed into `run()`. I have a typedef named `unit_to_unit_t` for the job function pointer - it's a function taking no arguments and returning nothing. The scheduler is also an FFI entity.

This is the definition of `run()` on the Rust side:

```rust
#[no_mangle]
pub unsafe extern "C" fn run(job: *mut Job, scheduler: *mut Scheduler, work: *const fn() -> ()) {
    let job = {
        assert!(!job.is_null());
        Box::from_raw(job)
    };
    let mut scheduler = {
        assert!(!scheduler.is_null());
        &mut *scheduler
    };

    let work: &fn() -> () = {
        assert!(!work.is_null());
        &*work
    };

    job.run(&mut scheduler, *work)
        .unwrap_or_else(|e| eprintln!("Error: {}", e));
}
```

I'm passing the C function as the last argument here, and trying to cast it to `work`. This step doesn't complain, seems to go okay.

On the Rust side, when that `job.run()` method gets called, the following trait/struct is used to store the function pointer and call it:

```rust
pub trait Callable {
    /// Execute this callable
    fn call(&self) -> Option<bool>;
    /// Get the name of this callable
    fn name(&self) -> &str;
}

/// A named callable function taking no parameters and returning nothing.
#[derive(Debug)]
pub struct UnitToUnit {
    name: String,
    work: fn() -> (),
}

impl UnitToUnit {
    pub fn new(name: &str, work: fn() -> ()) -> Self {
        Self {
            name: name.into(),
            work,
        }
    }
}

impl Callable for UnitToUnit {
    fn call(&self) -> Option<bool> {
        // gets HERE just fine...
        (self.work)();
        None
    }
    fn name(&self) -> &str {
        &self.name
    }
}
```

This all works fine with Rust function pointers. I determined that with the C version, we do get inside the `call()` implementation - the segfault happens when I use `(self.work)();`

I'm not sure if it's relevant, but the actual call is triggered here in C:

```c
extern void run_pending(scheduler_t *);    

// Run some jobs
for (int i = 0; i < 100; i++)
{
    run_pending(scheduler);
    sleep(1);
}
```

Which corresponds to this Rust interface fn:

```rust
#[no_mangle]
pub unsafe extern "C" fn run_pending(ptr: *mut Scheduler) {
    let scheduler = {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    
    scheduler
        .run_pending()
        .unwrap_or_else(|e| eprintln!("Error: {}", e));
}
```

Here's `scheduler::run_pending()`:

```rust
    /// Run all jobs that are scheduled to run.  Does NOT run missed jobs!
    pub fn run_pending(&mut self) -> Result<()> {
        //let mut jobs_to_run: Vec<&Job> = self.jobs.iter().filter(|el| el.should_run()).collect();
        self.jobs.sort();
        let mut to_remove = Vec::new();
        for (idx, job) in self.jobs.iter_mut().enumerate() {
            if job.should_run() {
                let keep_going = job.execute()?;
                if !keep_going {
                    debug!("Cancelling job {}", job);
                    to_remove.push(idx);
                }
            }
        }
        // Remove any cancelled jobs
        to_remove.sort_unstable();
        to_remove.reverse();
        for &idx in &to_remove {
            self.jobs.remove(idx);
        }

        Ok(())
    }
```

And finally, `job::execute()`:

```rust
    /// Run this job and immediately reschedule it, returning true.  If job should cancel, return false.
    ///
    /// If the job's deadline has arrived already, the job does not run and returns false.
    ///
    /// If this execution causes the deadline to reach, it will run once and then return false.
    pub fn execute(&mut self) -> Result<bool> {
        if self.is_overdue(self.now()) {
            debug!("Deadline already reached, cancelling job {}", self);
            return Ok(false);
        }

        debug!("Running job {}", self);
        if self.job.is_none() {
            debug!("No work scheduled, moving on...");
            return Ok(true);
        }
        let _ = self.job.as_ref().unwrap().call(); // CALLED RIGHT HERE
        self.last_run = Some(self.now());
        self.schedule_next_run()?;

        if self.is_overdue(self.now()) {
            debug!("Execution went over deadline, cancelling job {}", self);
            return Ok(false);
        }

        Ok(true)
    }
```

I added a comment where the actual `call()` happens.

This whole FFI shindig is just an experiment, I don't actually have a real need for this to work - but now that I got this far, I kinda want to know what I'm getting wrong! Thanks in advance.
