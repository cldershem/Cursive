extern crate cursive;
extern crate rand;

use rand::Rng;

use cursive::prelude::*;
use cursive::view::{Counter, Ticker};

use std::thread;
use std::cmp::min;
use std::time::Duration;

fn main() {
    let mut siv = Cursive::new();

    // We'll start slowly with a simple start button...
    siv.add_layer(Dialog::empty()
        .title("Progress bar example")
        .padding((0, 0, 1, 1))
        .content(Button::new("Start", phase_1)));

    // Auto-refresh is currently required for animated views
    siv.set_fps(30);

    siv.run();
}

// Function to simulate a long process.
fn fake_load(n_max: usize, ticker: Ticker) {
    for _ in 0..n_max {
        thread::sleep(Duration::from_millis(5));
        // The ticker method increases the progress value
        ticker(1);
    }
}

fn phase_1(s: &mut Cursive) {
    // Phase 1 is easy: a simple pre-loading.

    // Number of ticks
    let n_max = 500;

    // This is the callback channel
    let cb = s.cb_sink().clone();

    s.pop_layer();
    s.add_layer(Panel::new(FullView::full_width(ProgressBar::new()
        .range(0, n_max)
        .with_task(move |ticker| {
            // This closure will be called in a separate thread.
            fake_load(n_max, ticker);

            // When we're done, send a callback through the channel
            cb.send(Box::new(coffee_break)).unwrap();
        }))));
}

fn coffee_break(s: &mut Cursive) {
    // A little break before things get serious.
    s.pop_layer();
    s.add_layer(Dialog::empty()
        .title("Preparation complete")
        .content(TextView::new("Now, the real deal!"))
        .button("Again??", phase_2));
}

fn phase_2(s: &mut Cursive) {
    // Now, we'll run N tasks
    // (It could be downloading a file, extracting an archive,
    // reticulating sprites, ...)
    let n_bars = 10;
    // Each task will have its own shiny counter
    let counters: Vec<_> = (0..n_bars).map(|_| Counter::new(0)).collect();
    // To make things more interesting, we'll give a random speed to each bar
    let speeds: Vec<_> =
        (0..n_bars).map(|_| rand::thread_rng().gen_range(50, 150)).collect();

    let n_max = 100000;
    let cb = s.cb_sink().clone();

    // Let's prepare the progress bars...
    let mut linear = LinearLayout::vertical();
    for c in &counters {
        linear.add_child(ProgressBar::new()
            .max(n_max)
            .with_value(c.clone()));
    }

    s.pop_layer();
    s.add_layer(Dialog::new(FullView::full_width(linear))
        .title("Just a moment..."));

    // And we start the worker thread.
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(5));
            let mut done = true;
            for (c, s) in counters.iter().zip(&speeds) {
                let ticks = min(n_max - c.get(), *s);
                c.tick(ticks);
                if c.get() < n_max {
                    done = false;
                }
            }
            if done {
                break;
            }
        }

        cb.send(Box::new(final_step)).unwrap();
    });
}

fn final_step(s: &mut Cursive) {
    // A little break before things get serious.
    s.pop_layer();
    s.add_layer(Dialog::empty()
        .title("Report")
        .content(TextView::new("Time travel was a success!\n\
                               We went forward a few seconds!!"))
        .button("That's it?", |s| s.quit()));
}
