# Design Decisions

There's various aspects of Micro I've thought about changing or have changed multiple times
in the past. I'm writing down my thoughts so I don't keep forgetting why I chose something
and then change it again on a whim.

## glow vs. wgpu

For a time, I was using glow instead of wgpu because wgpu had a few frames of input lag
with vsync turned on. It now has a way to configure that to not happen, and glow requires
a lot of mistake-prone unsafe code, so wgpu is clearly the better choice.

## Separate update/draw callbacks vs. one unified callback

I'm used to separating update and draw from LOVE, and I like keeping logic and presentation
separate. One downside of separating the callbacks is it forces my hand a bit with ECS design,
since I have to have separate schedules for updating and drawing.

## Returning `Result`s from callbacks

I've been back and forth on whether the update/draw/event/etc. callbacks should return `Result`s
or not. Whatever I choose for the `App` callbacks, I also choose for `Scene` and `System`
callbacks. Technically, I don't have to make the same choice for all these things, but it would
be weird not to.

The benefit of returning `Result`s is that it makes error handling more ergonomic. Instead of
having to write `unwrap` or `expect`, I just write `?`. This is especially true of filesystem
code, where every step returns a `Result`. But I think it also encourages lazy handling
of `Result`s in situations where bubbling the error up isn't the right thing to do. For instance,
hecs has some functions that return a `Result`, but these `Result`s indicate either a normal
situation in the game logic that I should handle differently, or a programmer error, in which
case I should panic or log a warning. But it's so tempting to just write a `?` and bubble the
error up, even if it doesn't make sense.

There's only so many situations in a game where I should be dealing with `Result`s anyway.
Mostly when loading or saving files. And if a `Result` bubbles all the way up to the `App`
callbacks, it's too late to handle them properly. So I think I'm going to remove the `Result`s
from the callbacks. I should really be handling errors much closer to where they originate.

## `ctx` vs. no `ctx`

For most of Micro's history, the `App` callbacks have taken a `&mut Context` argument, which is
the type that lets you interact with the framework to query inputs, change window settings, etc.
It also has to be passed into every `draw` function. It's a bit verbose! Especially with
function signatures that have multiple contexts, like:

```rs
fn update(ctx: &mut Context, globals: &mut Globals, gameplay_ctx: &mut GameplayContext)
```

I really want to figure out how to not have this top-down flow of more and more contexts getting
passed everywhere. If I could do that, Micro's supporting crates (like the scene manager and ECS)
could make less assumptions about the architecture of the game.

As for the `Context` itself, I can put it into a thread local static and take all of the methods
that were on `Context` and make them free-floating functions. I've successfully done that before.
Last time I tried to make `Globals` a static in AetherBeats, I started getting deadlocks from mutexes.
I probably made a silly mistake somewhere.
