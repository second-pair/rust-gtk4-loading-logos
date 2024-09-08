/*  *--<Preface>--*  //

 -=-  Author Details  -=-
 Blair Edwards
 They wouldn't have paid me enough anyway.

 -=-  Dates  -=-
 Started 2023-08-29

 -=-  Description  -=-
 This programme attempts to ...
 IO is primarily through GTK GUI Widgets.
 Extra debug is provided via STDOUT.
 Config is through compile-time 'const's.

 -=-  Task  -=-
 -=>  Set up a Cairo canvas.
 -=>  Render it on a timeout (IE animated).
 -=>  Figure out how to draw something cool.
 -=>  Le Many Step #3.

 -=-  Notes  -=-
 -=>  I've developed my own commenting notation for things that "aren't done" one way or another.  Such as:
	 -  //#  TODO
	 -  //?  Not sure / query
	 -  //!  Important note / relevant as technology advances
 -=>  Logging with `_LOG ()` takes a 'logLevel' argument, which roughly indicates:
	 -  0:  Critical, major errors, should Always be printed.
	 -  1:	Important info particularly critical functions, minor / user errors.
	 -  2:	Useful info / general programme flow.
	 -  3:	Debug info, steps throughout a function.
	 -  4:  Useful spam - printed often such as in a loop.
	 -  5:  Debug spam - printed often such as in a loop.
 -=>

//  *--</Preface>--*  */



//  *--<Preparations>--*  //

//  Local Compiler Pragmas
#! [allow (unused_variables)]
#! [allow (non_snake_case)]
#! [allow (unused_parens)]
#! [allow (dead_code)]
#! [allow (unused_imports)]

//  Imports
//use std ::io ::{stdin, stdout};
//use std ::time;
//use std ::fs ::File;
//use std ::path ::Path;
//use std ::io ::{Read, BufReader, BufRead};
//use std ::io ::{Write, BufWriter};
use std ::sync ::atomic ::{AtomicUsize, Ordering};
//use std ::rc ::Rc;
//use std ::cell ::RefCell;
use std ::f64 ::consts ::PI;
//  of Which are GTK4
use gtk4 as gtk;
use gtk ::prelude ::*;
use gtk ::glib;
use gtk ::glib ::clone;
//  of Which are Local
//mod subModule;

//  Global Constants
const APP_ID: &str = "uk.second-pair.testing.gtk.loading-logos";
const APP_TITLE: &str = "Gtk Template";
const TIME_ANIM: u16 = 50;
const TIME_ANIM_SLOW: u16 = 1000;
/*  Sizing:
0:  Do Not Expand
1:  Default Size (User-Expandable)
2:  Fixed Size
3:  Fullscreen
*/
const APP_SIZING: u8 = 0;
const APP_W: i32 = 640;
const APP_H: i32 = 480;

const DRAW_W: i32 = 1000;
const DRAW_H: i32 = 1000;
/*  Animation Types
1.  Pulsing radius-filling squircle.
2.  Circumference-filling circle CCW.
3.  Circumference-filling circle, CW.
4.  Orbiting N-Ary balls.
5.  Circumference-filling circle, with N-Ary orbiting balls.
6.  N-start circumference-following arcs.
7.  Concentric reverse-direction circumference-following circles.
8.  Concentric reverse-direction circumference-following circles V2.
9.  Concentric reverse-direction circumference-following circles, multi-speed.
10.  Orbiting N-Ary balls, with radius lines.
11.  Orbiting N-Ary balls, with radius-following pulsers.
*/
const ANIM_TYPE: u8 = 11;
//#  It should be possible to parameterise all of the animation types.

//  Global Variables

//  Local Constants

//  Local Variables

//  Structures

//  *--</Preparations>--*  //



//  *--<Macros>--*  //

//  TODO:  implement macros.rs.
/*macro_rules! macroName
{
	($a: expr, $b: expr) =>
	{
		$a + $b
	}
	($a: expr) =>
	{
		$a
	}
}*/

//  *--</Macros>--*  //



//  *--<Traits & Implementations>--*  //

//  *--</Traits & Implementations>--*  //



//  *--<Main Code>--*  //

fn main () -> glib ::ExitCode
{
	let app = gtk ::Application ::builder ()
		.application_id (APP_ID)
		.build ();

	app .connect_activate (|app| {guiMain_create (app)});
	return app .run ();
}

fn guiWindow_create (app: &gtk ::Application) -> gtk ::ApplicationWindow
{
	//  Create the window.
	let theWindow = gtk ::ApplicationWindow ::builder ()
		.application (app)
		.title (APP_TITLE)
		.build ();
	//  Set the sizing based off our #defines.
	match (APP_SIZING)
	{
		0 => (),
		1 =>
		{
			theWindow .set_default_width (APP_W);
			theWindow .set_default_height (APP_H);
			theWindow .set_resizable (true);
		},
		2 =>
		{
			theWindow .set_default_width (APP_W);
			theWindow .set_default_height (APP_H);
			theWindow .set_resizable (false);
		},
		3 => theWindow .set_fullscreened (true),
		_ => panic! ("'APP_SIZING' out-of-range!"),
	};
	//  Apply theming.
	//#  CSS Implement
	theWindow .settings () .set_gtk_application_prefer_dark_theme (true);

	return theWindow
}

fn guiMain_create (app: &gtk ::Application)
{
	//  Create the main window.
	let window_main = guiWindow_create (app);

	//  Create the top-level layout widget.
	let box_main = gtk ::Box ::builder ()
		.orientation (gtk ::Orientation ::Vertical)
		.build ();
	window_main .set_child (Some (&box_main));

	//  Attach the Cairo canvas.
	let cairo_loading = gtk ::DrawingArea ::builder ()
		.content_width (DRAW_W)
		.content_height (DRAW_H)
		.build ();
	cairo_loading .set_draw_func (cairo_loading_render);
	//  Local, so we don't mess with GTK's main-thread requirements.
	gtk ::glib ::timeout_add_local
	(
		core ::time ::Duration ::from_millis (TIME_ANIM as u64),
		clone! (@strong cairo_loading => move ||
		{
			cairo_loading .queue_draw ();
			return Continue (true);
		}
	));
	box_main .append (&cairo_loading);

	//  Show the window and get outta here.
	window_main .present ();
}

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

fn cairo_loading_render (area: &gtk ::DrawingArea, cairo: &gtk ::cairo ::Context, width: i32, height: i32)
{
	//  'static' iteration counter.
	static ITER: AtomicUsize = AtomicUsize ::new (0);
	let iter = ITER .fetch_add (1, Ordering ::Relaxed) as f64;

	//  Move the origin to the middle and flip the Y-axis.
	let matrix = gtk ::cairo ::Matrix ::new (1.0, 0.0, 0.0, -1.0, width as f64 / 2.0, height as f64 / 2.0);
	cairo .transform (matrix);

	match ANIM_TYPE
	{
		1 =>
		{
			let iterScaled = iter * 3.0;
			let radMax = 100.0;
			if (iterScaled % (radMax * 2.0) <= radMax)
			{
				cairo .move_to (iterScaled % (radMax * 2.0), 0.0);
				cairo .arc (0.0, 0.0, iterScaled % (radMax * 2.0), 0.0, PI * 2.0);
			}
			else
			{
				cairo .move_to (radMax * 2.0 - iterScaled % (radMax * 2.0), 0.0);
				cairo .arc (0.0, 0.0, radMax * 2.0 - iterScaled % (radMax * 2.0), 0.0, PI * 2.0);
			}
		},
		2 =>
		{
			let iterScaled = iter * 0.26 % (PI * 4.0);
			let radMax = 100.0;
			cairo .move_to (radMax, 0.0);
			match (iterScaled <= PI * 2.0)
			{
				true => cairo .arc (0.0, 0.0, radMax, 0.0, iterScaled),
				false => cairo .arc_negative (0.0, 0.0, radMax, 0.0, iterScaled),
			};
		},
		3 =>
		{
			let iterScaled = iter * 0.26 % (PI * 4.0);
			let iterRev = PI * 2.0 - iterScaled;
			let radMax = 100.0;
			cairo .move_to (radMax * iterRev .cos (), radMax * iterRev .sin ());
			match (iterScaled <= PI * 2.0)
			{
				true => cairo .arc (0.0, 0.0, radMax, iterRev, PI * 2.0),
				false => cairo .arc_negative (0.0, 0.0, radMax, iterRev, PI * 2.0),
			};
		},
		4 =>
		{
			let radCircle = 20.0;
			let radOrbit = 100.0;
			let countCircle = 3;
			let iterScaled = iter * 0.1;

			for circle in 0..countCircle
			{
				let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
				cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
				cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
			}
		},
		5 =>
		{
			let radOuter = 200.0;
			let radOrbit = 100.0;
			let radCircle = 20.0;
			let countCircle = 3;
			let iterCirc = (iter * 0.26) % (PI * 4.0);
			cairo .move_to (radOuter, 0.0);
			match (iterCirc <= PI * 2.0)
			{
				true => cairo .arc (0.0, 0.0, radOuter, 0.0, iterCirc),
				false => cairo .arc_negative (0.0, 0.0, radOuter, 0.0, iterCirc),
			};

			for circle in 0..countCircle
			{
				let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
				cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
				cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
			}
		},
		6 =>
		{
			let starts = 5;
			let iterCirc = (iter * 0.07) % (PI * 2.0);
			let radCircle = 120.0;
			let lengthArc = 0.4;
			for start in 0..starts
			{
				let iterStart = iterCirc + PI * 2.0 * start as f64 / starts as f64;
				cairo .move_to (radCircle * iterStart .cos (), radCircle * iterStart .sin ());
				cairo .arc (0.0, 0.0, radCircle, iterStart, iterStart + lengthArc);
			}
		},
		7 =>
		{
			let iterScale = (iter * 0.26) % (PI * 4.0);
			let radStart = 80.0;
			let radSpace = 25.0;
			let countCircle = 3;
			for circle in 0..countCircle
			{
				let rad = radStart + (radSpace * circle as f64);
				if (circle % 2 == 0)
				{
					cairo .move_to (rad, 0.0);
					match (iterScale <= PI * 2.0)
					{
						true => cairo .arc (0.0, 0.0, rad, 0.0, iterScale),
						false => cairo .arc_negative (0.0, 0.0, rad, 0.0, iterScale),
					};
				}
				else
				{
					let iterRev = PI * 4.0 - iterScale;
					cairo .move_to (rad * iterRev .cos (), rad * iterRev .sin ());
					match (iterScale <= PI * 2.0)
					{
						true => cairo .arc (0.0, 0.0, rad, iterRev, PI * 2.0),
						false => cairo .arc_negative (0.0, 0.0, rad, iterRev, PI * 2.0),
					};
				}
			}
		},
		8 =>
		{
			let iterScale = (iter * 0.26) % (PI * 4.0);
			let radStart = 80.0;
			let radSpace = 25.0;
			let countCircle = 3;
			for circle in 0..countCircle
			{
				let rad = radStart + (radSpace * circle as f64);
				if (circle % 2 == 0)
				{
					cairo .move_to (rad, 0.0);
					match (iterScale <= PI * 2.0)
					{
						true => cairo .arc (0.0, 0.0, rad, 0.0, iterScale),
						false => cairo .arc_negative (0.0, 0.0, rad, 0.0, iterScale),
					};
				}
				else
				{
					let iterRev = PI * 4.0 - iterScale;
					cairo .move_to (rad * (iterRev + PI) .cos (), rad * (iterRev + PI) .sin ());
					match (iterScale <= PI * 2.0)
					{
						true => cairo .arc (0.0, 0.0, rad, iterRev + PI, PI),
						false => cairo .arc_negative (0.0, 0.0, rad, iterRev + PI, PI),
					};
				}
			}
		},
		9 =>
		{
			let iterScale = iter * 0.1;
			let radStart = 80.0;
			let radSpace = 25.0;
			let countCircle = 3;
			for circle in 0..countCircle
			{
				let rad = radStart + (radSpace * circle as f64);
				let iterScaleSpd = (iterScale + iterScale * 0.5 * circle as f64) % (PI * 4.0);
				if (circle % 2 == 0)
				{
					cairo .move_to (rad, 0.0);
					match (iterScaleSpd <= PI * 2.0)
					{
						true => cairo .arc (0.0, 0.0, rad, 0.0, iterScaleSpd),
						false => cairo .arc_negative (0.0, 0.0, rad, 0.0, iterScaleSpd),
					};
				}
				else
				{
					let iterScaleRev = PI * 4.0 - iterScaleSpd;
					cairo .move_to (rad * iterScaleRev .cos (), rad * iterScaleRev .sin ());
					match (iterScaleRev <= PI * 2.0)
					{
						true => cairo .arc_negative (0.0, 0.0, rad, iterScaleRev, PI * 2.0),
						false => cairo .arc (0.0, 0.0, rad, iterScaleRev, PI * 2.0),
					};
				}
			}
		},
		10 =>
		{
			let radCircle = 20.0;
			let radOrbit = 200.0;
			let sparkStart = 30.0;
			let sparkGap = 40.0;
			let countCircle = 3;

			for circle in 0..countCircle
			{
				let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
				//  Spark Line
				cairo .move_to (sparkStart * iterStart .cos (), sparkStart * iterStart .sin ());
				cairo .line_to ((radOrbit - radCircle / 2.0 - sparkGap) * iterStart .cos (), (radOrbit - radCircle / 2.0 - sparkGap) * iterStart .sin ());
				//  Circle
				cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
				cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
			}
		},
		11 =>
		{
			let radCircle = 20.0;
			let radOrbit = 200.0;
			let sparkStart = 30.0;
			let sparkGap = 40.0;
			let sparkStop = (radOrbit - radCircle / 2.0 - sparkGap);
			let countCircle = 3;
			let iterSpark = (iter * 8.0) % ((sparkStop - sparkStart) * 2.0);

			for circle in 0..countCircle
			{
				let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
				//  Spark Line
				match (iterSpark <= sparkStop - sparkStart)
				{
					true =>
					{
						cairo .move_to (sparkStart * iterStart .cos (), sparkStart * iterStart .sin ());
						cairo .line_to ((sparkStart + iterSpark) * iterStart .cos (), (sparkStart + iterSpark) * iterStart .sin ());
					},
					false =>
					{
						let iterSpark = iterSpark % (sparkStop - sparkStart);
						cairo .move_to ((sparkStart + iterSpark) * iterStart .cos (), (sparkStart + iterSpark) * iterStart .sin ());
						cairo .line_to (sparkStop * iterStart .cos (), sparkStop * iterStart .sin ());
					},
				};
				//  Circle
				cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
				cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
			}
		},
		_ => panic! ("'ANIM_TYPE' out-of-range!"),
	}

	//  Render that line.
	cairo .set_line_width (10.0);
	cairo .set_line_cap (gtk ::cairo ::LineCap ::Round);
	cairo .set_line_join (gtk ::cairo ::LineJoin ::Round);
	cairo .set_source_rgba (1.0, 1.0, 1.0, 1.0);
	cairo .stroke () .unwrap ();
}

//  *--</Callbacks>--*  //
